extern crate aws_lambda_tower_web;
extern crate futures;
#[macro_use]
extern crate tower_web;

use aws_lambda_tower_web::ServiceBuilderExt;
use futures::{future, Future};
use tower_web::ServiceBuilder;

#[derive(Clone, Debug)]
pub struct HelloWorld {
    motd: String,
}

impl_web! {
    impl HelloWorld {
        #[get("/")]
        fn hello_world(&self) -> Result<&'static str, ()> {
            Ok("This is a basic response served by tower-web")
        }

        #[get("/motd")]
        fn motd(&self) -> Result<String, ()> {
            Ok(format!("MOTD: {}", self.motd))
        }

        #[get("/hello-future")]
        fn hello_future(&self) -> impl Future<Item = String, Error = ()> + Send {
            future::ok("Or return a future that resolves to the response".to_string())
        }

        #[post("/print_std")]
        fn print_std(&self) -> Result<&'static str, ()> {
            println!("Hello from the web");
            Ok("done")
        }
    }
}

pub fn main() {
    ServiceBuilder::new()
        .resource(HelloWorld {
            motd: "tower-web is amazing!!!".to_string(),
        })
        .run_lambda()
        .unwrap();
}
