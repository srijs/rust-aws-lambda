extern crate aws_lambda_runtime as lambda;

fn main() {
    lambda::start(|()| Ok("Hello ƛ!"))
}
