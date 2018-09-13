# Rust on AWS Lambda

This repository contains multiple crates that make it possible to run programs written in Rust directly as functions in AWS Lambda, while keeping a low footprint with regards to memory consumption, bundle size, and start-up speed.

[![Build Status](https://travis-ci.org/srijs/rust-aws-lambda.svg?branch=master)](https://travis-ci.org/srijs/rust-aws-lambda)

[🌏 Website](https://srijs.github.io/rust-aws-lambda) | [🚀 Getting Started](/docs/getting-started.md)

## Example

The `start` function will launch a runtime which will listen for messages from the lambda environment, and call a handler function every time the lambda is invoked. This handler function can be async, as the runtime itself is based on top of `futures` and `tokio`.

```rust,no_run
// [dependencies]
// aws_lambda = { git = "https://github.com/srijs/rust-aws-lambda" }
extern crate aws_lambda as lambda;

fn main() {
    // start the runtime, and return a greeting every time we are invoked
    lambda::start(|()| Ok("Hello ƛ!"))
}
```

## Comparison to other projects

AWS Lambda does not officially support Rust. To enable using Rust with lambda, great projects such as [`rust-crowbar`](https://github.com/ilianaw/rust-crowbar) and [`serverless-rust`](https://github.com/softprops/serverless-rust) were created. They leverage Rust's C interoperability to "embed" Rust code into lambda supported language runtimes (in this case Python and Node.js respectively).

This project forgoes embedding and instead leverages lambda's official Go support. With Go, lambda runs a standard Linux binary containing a server and then sends it [`gob`-encoded messages](https://golang.org/pkg/encoding/gob/) via [rpc](https://golang.org/pkg/net/rpc/). This project reimplements all that machinery in Rust, using [`rust-gob`](https://github.com/srijs/rust-gob) for `gob` support and a custom [`tokio`](https://github.com/tokio-rs/tokio) server runtime. Lambda does not care that the Linux binary it runs is written in Rust rather than Go as long as they behave the same.

Due to the no-overhead method of adding Rust support, a project deployed to lambda using this runtime should match (and might possibly exceed) the performance of a similar project written in Go.
