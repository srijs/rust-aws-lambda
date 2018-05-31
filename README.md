# Rust on AWS Lambda

This repository contains multiple crates that make it possible to run programs written in Rust directly as functions in AWS Lambda, while keeping a low footprint with regards to memory consumption, bundle size, and start-up speed.

## Usage

### Install

Because this project is still in an early (but functional!) stage, it has not yet been published to the `crates` registry. You will therefore need to depend directly on the Github repository. Add the following to the `[dependencies]` section in your `Cargo.toml` file.

```toml
aws_lambda_runtime = { git = "https://github.com/srijs/rust-aws-lambda" }
```

### Create

The `aws_lamba_runtime` crate provides a runtime environment which will listen for messages from the lambda environment, and call a function every time the lambda is invoked. This function can be async, as the runtime itself is based on top of `futures` and `tokio`.

```rust
extern crate aws_lambda_runtime as lambda;

fn main() {
    // start the runtime, and return a greeting every time we are invoked
    lambda::start(|()| Ok("Hello ƛ!"))
}
```

To provide input data to your function, you can change the type of the argument that the function accepts. For this to work, the argument type needs to implement the `serde::Deserialize` trait (most types in the standard library do).

```rust
extern crate aws_lambda_runtime as lambda;

use std::collections::HashMap;

fn main() {
    lambda::start(|input: HashMap<String, String>| {
        Ok(format!("the values are {}, {} and {}",
            input["key1"], input["key2"], input["key3"]))
    })
}
```

While your function is running you can call `Context::current()` to get additional information, such as the ARN of your lambda, the Amazon request id or the Cognito identity of the calling application.

```rust
extern crate aws_lambda_runtime as lambda;

fn main() {
    lambda::start(|()| {
        let ctx = lambda::Context::current();

        Ok(format!("Hello from {}!", ctx.invoked_function_arn()))
    })
}
```

### Deploy

TBD
