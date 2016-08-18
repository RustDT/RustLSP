#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

extern crate serde_json;
extern crate serde;
extern crate melnorme_util as util;

pub mod json_util;
pub mod service_util;
mod json_rpc;

pub use json_rpc::*;
