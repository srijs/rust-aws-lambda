# Tower Web on AWS Lambda

This crate makes it possible to run web apps written using [Tower Web](https://github.com/carllerche/tower-web) directly as functions in AWS Lambda, while keeping a low footprint with regards to memory consumption, bundle size, and start-up speed.

## Install

Because this project is still in an early (but functional!) stage, it has not yet been published to the `crates` registry. You will therefore need to depend directly on the Github repository. Add the following to the `[dependencies]` section in your `Cargo.toml` file.

```toml
aws_lambda_tower_web = { git = "https://github.com/srijs/rust-aws-lambda" }
```

## Getting Started

In order to run your Tower Web application on AWS Lambda, you need to change exactly three lines of code in your project.

### 1. Import the crate

```diff
  #[macro_use]
  extern crate tower_web;
+ extern crate aws_lambda_tower_web;
```

### 2. Import the extension trait

```diff
  use tower_web::ServiceBuilder;
+ use aws_lambda_tower_web::ServiceBuilderExt;
```

### 3. Use a different run method

```diff
  ServiceBuilder::new()
      // some setup code...
+     .run_lambda()
-     .run(&addr)
      .unwrap();
```

## Deploy

The first step in deploying your app to AWS Lambda is to compile the project and upload the binary to AWS Lambda. Cross-compiling for Amazon Linux can be tricky, and is best done using docker. Please check [the guide in the main README](https://github.com/srijs/rust-aws-lambda/blob/master/README.md#deploy) for more information on this.

After you've successfully deployed your AWS Lambda, you want to configure your AWS API Gateway to call the lambda function. This works best if you create a [proxy resource](https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-set-up-simple-proxy.html) and point that to your lambda. You will also want to `*/*` as a [binary media type](https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-payload-encodings-configure-with-console.html) so that the API Gateway correctly decodes binary responses.
