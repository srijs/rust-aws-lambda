extern crate aws_lambda as lambda;
extern crate env_logger;

fn main() {
    env_logger::init();

    lambda::start(|()| Ok("Hello ƛ!"))
}
