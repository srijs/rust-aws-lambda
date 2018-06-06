# Code generation for `aws_lambda_events`

Provides a CLI tool that can be used to generate Rust event type definitions from Go source code.

## Usage

```bash
$ cargo run -- --input $GOPATH/src/github.com/aws/aws-lambda-go --output ../aws_lambda_events/src/generated --overwrite
```
