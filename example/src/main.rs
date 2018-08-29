#![recursion_limit="128"]
extern crate aws_lambda;
#[macro_use] extern crate log;

use aws_lambda::event::s3::S3Event;

fn main() {
    aws_lambda::logger::init();

    // start the runtime, and do
    aws_lambda::start(|input: S3Event| {
        info!("Event received {:?}", input);

        Ok(())
    })
}
