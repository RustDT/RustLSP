#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

extern crate serde_json;
extern crate serde;

pub extern crate rustdt_util as util;
pub extern crate jsonrpc;
pub extern crate languageserver_types as ls_types;

#[macro_use] extern crate log;

pub mod lsp_transport;
pub mod lsp_server;

#[cfg(test)]
mod server_tests;