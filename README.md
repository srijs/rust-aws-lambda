# Rust on AWS Lambda

This repository contains multiple crates that make it possible to run programs written in Rust directly as functions in AWS Lambda, while keeping a low footprint with regards to memory consumption, bundle size, and start-up speed.

[![Build Status](https://travis-ci.org/srijs/rust-aws-lambda.svg?branch=master)](https://travis-ci.org/srijs/rust-aws-lambda)

## Usage

### Install

Because this project is still in an early (but functional!) stage, it has not yet been published to the `crates` registry. You will therefore need to depend directly on the Github repository. Add the following to the `[dependencies]` section in your `Cargo.toml` file.

```toml
aws_lambda = { git = "https://github.com/srijs/rust-aws-lambda" }
```

### Create

The `start` function will launch a runtime which will listen for messages from the lambda environment, and call a handler function every time the lambda is invoked. This handler function can be async, as the runtime itself is based on top of `futures` and `tokio`.

```rust,no_run
extern crate aws_lambda as lambda;

fn main() {
    // start the runtime, and return a greeting every time we are invoked
    lambda::start(|()| Ok("Hello ƛ!"))
}
```

### Input

To provide input data to your handler function, you can change the type of the argument that the function accepts. For this to work, the argument type needs to implement the `serde::Deserialize` trait (most types in the standard library do).

```rust,no_run
extern crate aws_lambda as lambda;

use std::collections::HashMap;

fn main() {
    lambda::start(|input: HashMap<String, String>| {
        Ok(format!("the values are {}, {} and {}",
            input["key1"], input["key2"], input["key3"]))
    })
}
```

Additionally, the `event` module provides strongly-typed lambda event types for use with [AWS event sources](https://docs.aws.amazon.com/lambda/latest/dg/invoking-lambda-function.html).

For example, this would print out all the `S3Event` record names, assuming your lambda function was subscribed to the [proper S3 events](https://docs.aws.amazon.com/lambda/latest/dg/with-s3-example.html):

```rust,no_run
extern crate aws_lambda as lambda;

use lambda::event::s3::S3Event;

fn main() {
    lambda::start(|input: S3Event| {
        let mut names = Vec::new();
        for record in input.records {
            names.push(record.event_name);
        }
        Ok(format!("Event names:\n{:#?}", names))
    })
}
```

The types in the `event` module are automatically generated from the [official Go SDK](https://github.com/aws/aws-lambda-go/tree/master/events) and thus are generally up-to-date.

#### Dealing with `null` and empty strings in lambda input

The official Lambda Go SDK sometimes marks a field as required when the underlying lambda event json could actually be `null` or an empty string. Normally, this would cause a panic as Rust is much more strict.

The `event` module has two strategies for dealing with this reality. Both
are available as crate features so you can choose the behavior and API that works best for you:

- `string-null-none` - All required json string fields are `Option<String>` in Rust. Json `null` or the empty string are deserialized into Rust structs as `None`.

  This is the default behavior, as it is idiomatic Rust.

  - **Pros:** _Idiomatic Rust. It is easy to determine if lambda gave you a "real" value with data or not by checking the `Option<String>`._

  - **Cons:** _you have to `unwrap()`/`expect()`/`match` every string field to use its contents._

- `string-null-empty` - All required json string fields are `String` in Rust. Json `null` is deserialized into Rust structs as the empty string (`""`).

  This is what the official Go SDK does.

  - **Pros:** _you do not have to `unwrap()`/`expect()`/`match` every string field before using._
  - **Cons:** _Not idiomatic Rust. You manually have to check for `""` if you want to know the difference between a real value or an empty value._

  Change your `Cargo.toml` dependency to:
  ```toml
  aws_lambda = { git = "https://github.com/srijs/rust-aws-lambda", features = ["string-null-empty"] }
  ```

### Context

While your function is running you can call `Context::current()` to get additional information, such as the ARN of your lambda, the Amazon request id or the Cognito identity of the calling application.

```rust,no_run
extern crate aws_lambda as lambda;

fn main() {
    lambda::start(|()| {
        let ctx = lambda::Context::current();

        Ok(format!("Hello from {}!", ctx.invoked_function_arn()))
    })
}
```

### Logging

The `aws_runtime` crate bundles its own logger, which can be used through the
[`log`](https://crates.io/crates/log) facade.

To initialize the logging system, you can call `logger::init()`.

```rust,no_run
extern crate aws_lambda as lambda;
#[macro_use] extern crate log;

fn main() {
    lambda::logger::init();

    lambda::start(|()| {
        info!("running lambda function...");

        Ok("Hello ƛ!")
    })
}
```

### Deploy
*Note: These instructions will produce a static musl binary of your rust code. If you are looking for non-musl binaries, you might try [docker-lambda](https://github.com/lambci/docker-lambda).*

To deploy on AWS lambda, you will need a zip file of your binary built against amazonlinux. A Dockerfile is provided as an example and will work for single project binaries that need OpenSSL. The Dockerfile is based off of [rust-musl-builder](https://github.com/emk/rust-musl-builder).

    docker pull amazonlinux
    docker build --force-rm -t aws-lambda:latest --build-arg SRC=example -f docker/dockerfile .
    docker run -v /tmp/artifacts:/export --rm aws-lambda:latest
    
Build your lambda function and upload your zip file. Change the lambda runtime to `go 1.x` and set the handler function to the name of your application as defined in your `Cargo.toml`.

#### SSL considerations

If your binary requires SSL (e.g. [`rusoto`](https://github.com/rusoto/rusoto)), add the following environment variables:

    SSL_CERT_DIR=/etc/ssl/certs
    SSL_CERT_FILE=/etc/ssl/certs/ca-bundle.crt
    
If you are still running into SSL issues, you may need to modify your application per https://github.com/emk/rust-musl-builder#making-openssl-work.

#### `error_chain`
    
In general we suggest you use the [`failure`](https://github.com/rust-lang-nursery/failure) crate for error handling. If you are instead using [`error_chain`](https://github.com/rust-lang-nursery/error-chain) in your rust code, you will also have to disable default features to use the example musl Dockerfile. Add the following to your `Cargo.toml`:

    [dependencies.error-chain]
    version = "~0.12"
    default-features = false


### Troubleshooting

To help you debug your lambda function, `aws_lambda` integrates with the [`failure`](https://github.com/rust-lang-nursery/failure)
crate to extract stack traces from errors that are returned from the handler function.

In order to take advantage of this, you need to compile your program to include debugging symbols. When working with `cargo` using `--release`, you can add the following section to your `Cargo.toml` to include debug info in your release build:

```toml
[profile.release]
debug = true
```

Next, you want to instruct the runtime to collect stack traces when errors occur. You can do this by modifying the configuration of your function in AWS to set the `RUST_BACKTRACE` environment variable to `1`.

After both of these changes have been deployed, you should start to see stack traces included in both the error info returned from invocations, as well the CloudWatch logs for your function.

## Comparison to other projects

AWS Lambda does not officially support Rust. To enable using Rust with lambda, great projects such as [`rust-crowbar`](https://github.com/ilianaw/rust-crowbar) and [`serverless-rust`](https://github.com/dobrite/serverless-rust) were created. They leverage Rust's C interoperability to "embed" Rust code into lambda supported language runtimes (in this case Python and Node.js respectively).

While this works, there are some distinct downsides:

1. Higher footprint with regards to memory consumption, bundle size, and start-up speed due to the host runtime overhead.
2. Increased monetary cost due to #1.
3. More build complexity.

This project aims to remove all those downsides without adding new ones. It forgoes embedding and instead leverages lambda's official Go support. With Go, lambda runs a standard Linux binary containing a server and then sends it [`gob`-encoded messages](https://golang.org/pkg/encoding/gob/) via [rpc](https://golang.org/pkg/net/rpc/). This project reimplements all that machinery in Rust, using [`rust-gob`](https://github.com/srijs/rust-gob) for `gob` support and a custom [`tokio`](https://github.com/tokio-rs/tokio) server runtime. Lambda does not care that the Linux binary it runs is written in Rust rather than Go as long as they behave the same.

This enables:

1. Lower footprint with regards to memory consumption, bundle size, and start-up speed due to no runtime overhead.
2. Lower monetary cost due to #1.
3. No additional build complexity. Building a binary for lambda is as simple as building a Rust binary locally.

Due to the no-overhead method of adding Rust support, projects deployed to lambda using this runtime should match (and might possibly exceed) the performance of a similar project written in Go.
