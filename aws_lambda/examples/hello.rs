extern crate aws_lambda as lambda;
#[macro_use]
extern crate log;

fn main() {
    lambda::logger::init();

    lambda::start(|()| {
        info!("running lambda!");

        Ok("Hello ƛ!")
    })
}
