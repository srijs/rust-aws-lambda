extern crate aws_lambda as lambda;
#[macro_use]
extern crate log;

fn main() {
    lambda::logger::init();

    info!(
        "starting function {} (version {}) [{} bytes]",
        lambda::env::function_name(),
        lambda::env::function_version(),
        lambda::env::function_memory_size()
    );

    lambda::gateway::start(|req| {
        info!("received request: {:?}", req);

        let res = lambda::gateway::response()
            .status(200)
            .body("Hello ƛ!".into())?;

        Ok(res)
    })
}
