#![recursion_limit="128"]
extern crate aws_lambda;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate log;

use aws_lambda::event::s3::S3Event;

pub mod errors {
    use std;

    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! {
        foreign_links {
            Io(std::io::Error) #[doc = "Error during IO"];
            Ffi(std::ffi::NulError) #[doc = "Error during FFI conversion"];
            Utf8(std::str::Utf8Error) #[doc = "Error during UTF8 conversion"];
            FromUtf8(std::string::FromUtf8Error) #[doc = "Error during UTF8 conversion"];
        }
//        links {
//            ErrorName(error_lib::errors::Error, error_lib::errors::ErrorKind);
//        }
//        errors {
//
//        }
    }
}

use errors::Error;

fn print_error(err: &Error) {
    error!("error: {}", err);

    for e in err.iter().skip(1) {
        error!("caused by: {}", e);
    }

// The backtrace is not always generated. Try to run this example
// with `RUST_BACKTRACE=1`.
    if let Some(backtrace) = err.backtrace() {
        error!("backtrace: {:?}", backtrace)
    }
}

fn do_stuff(input: S3Event) -> Result<(), Error> {
    Ok(())
}

fn main() {
    aws_lambda::logger::init();

    // start the runtime, and do
    aws_lambda::start(|input: S3Event| {
        info!("Event received {:?}", input);

        if let Err(ref e) = do_stuff(input) {
            print_error(e);
        }

        Ok(())
    })
}
