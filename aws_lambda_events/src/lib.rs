#[macro_use]
extern crate serde_derive;
extern crate base64;
extern crate bytes;
extern crate chrono;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
extern crate serde;
#[cfg(test)]
#[macro_use]
extern crate serde_json;
#[cfg(not(test))]
extern crate serde_json;

mod custom_serde;
pub mod encodings;
pub mod event;
mod generated;
