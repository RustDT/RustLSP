// Copyright 2016 Bruno Medeiros
//
// Licensed under the Apache License, Version 2.0 
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>. 
// This file may not be copied, modified, or distributed
// except according to those terms.


use std::io::{self, Read};

use util::core::*;


/* ----------------- Parse content-length ----------------- */

const CONTENT_LENGTH: &'static str = "Content-Length:";
	
pub fn parse_transport_message<R : io::BufRead>(reader: &mut R) -> GResult<String>
{
	
	let mut content_length : u32 = 0; 
	
	loop {
		let mut line : String = String::new();
		
		try!(reader.read_line(&mut line));
		
		if line.starts_with(CONTENT_LENGTH) {
			let len_str : &str = &line[CONTENT_LENGTH.len()..]; 
			let int_result = len_str.trim().parse::<u32>();
			
			content_length = try!(int_result);
			
		} else if line.eq("\r\n") {
			break;
		}
	}
	if content_length == 0 {
		return Err(ErrorMessage::create(String::from(CONTENT_LENGTH) + " not defined or invalid."));
	}
	
	let mut message_reader = reader.take(content_length as u64);
	let mut message = String::new();
	try!(message_reader.read_to_string(&mut message));
	return Ok(message);
}


#[test]
fn parse_transport_message__test() {
	use std::io::BufReader;
	
	let string = String::from("Content-Length: 10 \r\n\r\n1234567890abcdef");
	assert_eq!(parse_transport_message(&mut BufReader::new(string.as_bytes())).unwrap(), "1234567890");

	let string = String::from("Content-Length: 13 \r\nBlaah-Blah\r\n\r\n1234\n567\r\n890abcdef");
	// Perhaps this case should result in an error.
	assert_eq!(parse_transport_message(&mut BufReader::new(string.as_bytes())).unwrap(), "1234\n567\r\n890");
	
	// Test no-content	
	let string = String::from("\r\n\r\n1234567890abcdef");
	let err : GError = parse_transport_message(&mut BufReader::new(string.as_bytes())).unwrap_err();
	assert_eq!(format!("{}", err), "Content-Length: not defined or invalid.");
}

pub fn write_transport_message<WRITE : io::Write>(message: & str, out: &mut WRITE) -> GResult<()>
{
//	let out : &mut io::Write = out;
	try!(out.write_all(CONTENT_LENGTH.as_bytes()));
	try!(out.write(&[' ' as u8]));
	let contents = message.as_bytes();
	try!(out.write_all(contents.len().to_string().as_bytes()));
	try!(out.write_all("\r\n\r\n".as_bytes()));
	try!(out.write_all(message.as_bytes()));
	try!(out.flush());
	Ok(())
}


#[test]
fn write_transport_message__test() {
	use util::tests::*;

	let mut out : Vec<u8> = vec!['x' as u8];
	write_transport_message(&"1234\n67", &mut out).unwrap();
	
	assert_equal(String::from_utf8(out).unwrap(), "xContent-Length: 7\r\n\r\n1234\n67".to_string());
}