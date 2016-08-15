// Copyright 2016 Bruno Medeiros
//
// Licensed under the Apache License, Version 2.0 
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>. 
// This file may not be copied, modified, or distributed
// except according to those terms.


// WARNING: Rust newbie code ahead (-_-)'

#![allow(non_camel_case_types)]

extern crate serde_json;

use std::io::{self, Read, Write};

use ::util::core::*;

use serde_json::Map;
use serde_json::Value;
use serde;

use json_rpc;
use json_rpc::*;

use lsp;
use lsp::*;
use ls;
use ls::LanguageServer;

use std::collections::HashMap;
use std::rc::Rc;

/* -----------------  ----------------- */

pub struct RustLSPServer {
	
	pub ls: Rc<LanguageServer>,
	pub rpc_dispatcher : JsonRpcDispatcher,
	
}

impl RustLSPServer {
	
	pub fn start_new(ls: Rc<LanguageServer>, input: &mut io::BufRead, output : &mut io::Write) {
		let rpc_dispatcher = JsonRpcDispatcher::new();
		let mut server = RustLSPServer { rpc_dispatcher : rpc_dispatcher, ls : ls };
		
		LanguageServerHandler::init(&mut server);
		
		let result = server.read_incoming_messages(input, output);
		match result {
			Err(error) => { 
				writeln!(&mut io::stderr(), "Error reading/writing the connection streams: {}", error)
					.expect("Failed writing to stderr");
			}
			Ok(_) => { } 
		}
	}
	
	pub fn read_incoming_messages(&mut self, input: &mut io::BufRead, output : &mut io::Write) -> GResult<()> {
		loop {
			let message = try!(parse_transport_message::<&mut _>(input));
			
			match self.process_message(&message) {
				Ok(_) => {  } 
				Err(error) => {
					try!(error.write_out(output));
					// TODO log 
//					try!(output.write_fmt(format_args!("Error parsing message: "))); 
				}
			};
		}
	}
	
	pub fn process_message(&mut self, message: &str) -> JsonRpcResult<()> {
		
		let rpc_request = try!(json_rpc::parse_jsonrpc_request(message));
		
		try!(self.rpc_dispatcher.dispatch(rpc_request));
		
		Ok(())
	}
}


pub struct LanguageServerHandler {
	
}

impl LanguageServerHandler {
	
	pub fn init(lsp_handler : &mut RustLSPServer) {
		let language_server = lsp_handler.ls.clone();
		
		let handler_fn : Box<json_rpc::DispatcherFn> = Box::new(move |json_rpc_handler, params_map| { 
			//FIXME : handle return
			json_rpc_handler.handle_method(params_map, &|params| { 
				ls::FN_INITIALIZE(language_server.as_ref(), params) 
			}); 
		});
		
		lsp_handler.rpc_dispatcher.dispatcher_map.insert("blah".to_string(), handler_fn);
	}
	
}

/* ----------------- Parse content-length ----------------- */

const CONTENT_LENGTH: &'static str = "Content-Length:";
	
pub fn parse_transport_message<R : io::BufRead>(mut reader: R) -> GResult<String>
{
	
	let mut content_length : u32 = 0; 
	
	loop {
		let mut line : String = String::new();
		
		try!(reader.read_line(&mut line));
		
		if line.starts_with(CONTENT_LENGTH) {
			let len_str : &str = &line[CONTENT_LENGTH.len()..]; 
			let my_int = len_str.trim().parse::<u32>();
			
			content_length = try!(my_int);
			
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
