---
title: About
layout: default
---

# About

The Rust on AWS Lambda project consists of multiple crates that make it possible to run programs written in Rust directly as functions in AWS Lambda, while keeping a low footprint with regards to memory consumption, bundle size, and start-up speed.

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
