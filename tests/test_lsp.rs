#![allow(non_snake_case)]

extern crate rust_lsp;

use rust_lsp::rust_lsp_server::*;
use rust_lsp::util::core::*;

use std::io::BufReader;



#[test]
fn parse_transport_message__test() {
	let string = String::from("Content-Length: 10 \r\n\r\n1234567890abcdef");
	assert_eq!(parse_transport_message(BufReader::new(string.as_bytes())).unwrap(), "1234567890");
}

#[test]
fn parse_transport_message__test2() {
	let string = String::from("Content-Length: 13 \r\nBlaah-Blah\r\n\r\n1234\n567\r\n890abcdef");
	assert_eq!(parse_transport_message(BufReader::new(string.as_bytes())).unwrap(), "1234\n567\r\n890");
}

#[test]
fn parse_transport_message__testNoContentError() {
	
	let string = String::from("\r\n\r\n1234567890abcdef");
	let err : GError = parse_transport_message(BufReader::new(string.as_bytes())).unwrap_err();
	assert_eq!(format!("{}", err), "Content-Length: not defined or invalid.");
}