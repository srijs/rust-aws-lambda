#[macro_use]
extern crate serde_derive;
extern crate base64;
extern crate bytes;
extern crate chrono;
extern crate serde;
#[cfg(test)]
#[macro_use]
extern crate serde_json;
#[cfg(not(test))]
extern crate serde_json;

mod deserializers;
pub mod event;
mod generated;
