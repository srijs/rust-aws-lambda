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

Additionally, the `event` module provides strongly-typed lambda event types for use with [AWS event sources](https://docs.aws.amazon.com/lambda/latest/dg/use-cases.html).

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

### Dealing with `null` and empty strings in lambda input

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

### Deploy

TBD
