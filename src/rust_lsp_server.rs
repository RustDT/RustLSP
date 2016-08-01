// Note: Rust newbie code ahead (-_-)'


extern crate serde_json;

use std::io::{self, Read, Write};

use ::util::core::*;

use self::serde_json::Map;

use json_rpc;
use json_rpc::JsonRpcDispatcher;
use json_rpc::JsonRpcResult;

pub struct RustLSPServer {
	
	pub rpc_dispatcher : JsonRpcDispatcher,
	
}

impl RustLSPServer {
	
	pub fn new() -> RustLSPServer {
		let mut new = RustLSPServer { rpc_dispatcher : JsonRpcDispatcher::new() };
		init_rust_lsp_procedures(&mut new.rpc_dispatcher);
		new
	}
	
	pub fn handle_streams(&mut self, mut input: &mut io::BufRead, mut output : &mut io::Write) {
		let result = self.read_incoming_messages(&mut input, &mut output);
		match result {
			Err(error) => { 
				writeln!(&mut io::stderr(), "Error reading/writing the connection streams: {}", error)
					.expect("Failed writing to stderr");
			}
			Ok(_) => { } 
		}
	}
	
	pub fn read_incoming_messages(&mut self, mut input: &mut io::BufRead, mut output : &mut io::Write) -> Result<()> {
		loop {
			let message = try!(parse_transport_message(&mut input));
			
		    match self.process_message(&message) {
		    	Ok(_) => {  } 
		    	Err(error) => {
		    		try!(error.write_out(&mut output));
		    		// TODO log 
//		    		try!(output.write_fmt(format_args!("Error parsing message: "))); 
		    	}
		    };
		}
	}
	
	pub fn process_message(&mut self, message: &str) -> JsonRpcResult<()> {
	    
	    let rpc_request = try!(json_rpc::parse_jsonrpc_request(message));
	    
	    try!(self.rpc_dispatcher.dispatch(&rpc_request));
	    
	    Ok(())
	}
}

const CONTENT_LENGTH: &'static str = "Content-Length:";
	
pub fn parse_transport_message<R>(mut reader: R) -> Result<String>
    where R: io::BufRead,
{
	let reader : &mut io::BufRead = &mut reader;
	
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
		return Err(StringCommonException::create(String::from(CONTENT_LENGTH) + " not defined or invalid."));
	}
	
	let mut message_reader = reader.take(content_length as u64);
	let mut message = String::new();
	try!(message_reader.read_to_string(&mut message));
	return Ok(message);
}

/* -----------------  ----------------- */

use self::serde_json::Value;

fn init_rust_lsp_procedures(rcp_dispactcher: &mut JsonRpcDispatcher) {
	rcp_dispactcher.dispatcher_map.insert("initialize".to_string(), Box::new(dispatch__initialize));
}

fn dispatch__initialize(params: &Map<String, Value>) {
	let rootPath = params.get("rootPath");
	let processId = params.get("processId");
	let capabilities = params.get("capabilities");
//	InitializeParams

}

