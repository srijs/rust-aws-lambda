# Tower Web on AWS Lambda

This crate makes it possible to run web apps written using [Tower Web](https://github.com/carllerche/tower-web) directly as functions in AWS Lambda, while keeping a low footprint with regards to memory consumption, bundle size, and start-up speed.

## Install

Because this project is still in an early (but functional!) stage, it has not yet been published to the `crates` registry. You will therefore need to depend directly on the Github repository. Add the following to the `[dependencies]` section in your `Cargo.toml` file.

```toml
aws_lambda_tower_web = { git = "https://github.com/srijs/rust-aws-lambda" }
```

## How To

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

For more information on how to deploy your app to AWS Lambda, please check the [relevant section in the main README](https://github.com/srijs/rust-aws-lambda/blob/master/README.md#deploy).
