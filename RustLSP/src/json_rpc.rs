#![allow(non_upper_case_globals)]

extern crate serde_json;

use self::serde_json::Map;
use self::serde_json::Value;

//use ::util::core::*;

#[derive(Debug, PartialEq)]
pub struct JsonRpcRequest {
	pub jsonrpc : String,
	pub id : u32,
	pub method : String,
	pub params : Map<String, Value>,
}


pub const JSON_RPC_ParseError : JsonRpcError = 
	JsonRpcError{code: -32700, message: "Invalid JSON was received by the server." };
pub const JSON_RPC_InvalidRequest : JsonRpcError = 
	JsonRpcError{code: -32600, message: "The JSON sent is not a valid Request object." };
pub const JSON_RPC_MethodNotFound : JsonRpcError = 
	JsonRpcError{code: -32601, message: "The method does not exist / is not available." };
pub const JSON_RPC_InvalidParams : JsonRpcError = 
	JsonRpcError{code: -32602, message: "Invalid method parameter(s)." };
#[allow(dead_code)]
pub const JSON_RPC_InternalError : JsonRpcError = 
	JsonRpcError{code: -32603, message: "Internal JSON-RPC error." };



use std::result;

pub type JsonRpcResult<T> = result::Result<T, JsonRpcError>;


pub fn parse_jsonrpc_request(message: &str) -> JsonRpcResult<JsonRpcRequest> {
	let json_result = match 
		serde_json::from_str(message) 
	{
		Ok(ok) => { ok } 
		Err(error) => { 
			return Err(JSON_RPC_ParseError);
		}
	};
	
    parse_jsonrpc_request_json(json_result)
}

pub fn parse_jsonrpc_request_json(request_json: Value) -> JsonRpcResult<JsonRpcRequest> {    
    
    
    let mut jsonMessage_map : Map<String, Value> =
    match request_json {
    	Value::Object(map) => map ,
    	_ => { return Err(JSON_RPC_InvalidRequest) },
    };
    
    let jsonrpc = try!(obtain_String(&mut jsonMessage_map, "jsonrpc"));
    let id = try!(obtain_u32(&mut jsonMessage_map, "id"));
    let method = try!(obtain_String(&mut jsonMessage_map, "method"));
    let params = try!(obtain_Map(&mut jsonMessage_map, "params"));
    
    let jsonrpc_request = JsonRpcRequest { jsonrpc : jsonrpc, id : id, method : method, params : params}; 
    
    Ok(jsonrpc_request)
}


fn obtain_Value(mut json_map : &mut Map<String, Value>, key: & str) -> JsonRpcResult<Value> {
	let value = json_map.remove(key);
	match value {
		Some(value) => { Ok(value) }, 
		None => { return Err(JSON_RPC_InvalidRequest) }
	}
	
}

fn obtain_String(json_map : &mut Map<String, Value>, key: &str) -> JsonRpcResult<String> {
	let value = try!(obtain_Value(json_map, key));
	match value {
		Value::String(string) => Ok(string),
		_ => { return Err(JSON_RPC_InvalidRequest) },
	}
}

fn obtain_Map(json_map : &mut Map<String, Value>, key: &str) -> JsonRpcResult<Map<String, Value>> {
	let value = try!(obtain_Value(json_map, key));
	match value {
		Value::Object(map) => Ok(map),
		_ => { return Err(JSON_RPC_InvalidRequest) },
	}
}

fn obtain_u32(json_map: &mut Map<String, Value>, key: &str) -> JsonRpcResult<u32> {
	let value = try!(obtain_Value(json_map, key));
	match value {
		Value::I64(num) => Ok(num as u32), // TODO: check for truncation
		Value::U64(num) => Ok(num as u32), // TODO: check for truncation
		_ => { return Err(JSON_RPC_InvalidRequest) },
	}
}

/* ----------------- test ----------------- */

#[test]
fn parse_jsonrpc_request_json_Test() {
	
	assert_eq!(parse_jsonrpc_request("{" ).unwrap_err(), JSON_RPC_ParseError);
	
	use self::serde_json::builder::ObjectBuilder;
	
	let sample_params = ObjectBuilder::new()
	    .insert("param", "2.0")
        .insert("foo", 123)
        .unwrap();
    let sample_params : Map<String, Value> = match sample_params {
    	Value::Object(o) => o,
    	_ => panic!(),
    };
	
    let request = ObjectBuilder::new()
	    .insert("jsonrpc", "2.0")
        .insert("id", 1)
        .insert("method", "myMethod")
        .insert("params", sample_params.clone())
        .unwrap();
    
    let result = parse_jsonrpc_request_json(request).unwrap();    
	assert_eq!(result, JsonRpcRequest { 
			jsonrpc : "2.0".to_string(), 
			id : 1, 
			method : "myMethod".to_string(), 
			params : sample_params}
	);
	
}


use std::io;
use self::serde_json::builder::ObjectBuilder;


#[derive(Debug, PartialEq)]
pub struct JsonRpcError {
	pub code : i32,
	pub message : &'static str,
}

impl JsonRpcError {
	
	pub fn to_string(&self) -> String {
	    let value = ObjectBuilder::new()
	        .insert("code", self.code)
	        .insert("message", self.message)
	        .unwrap()
        ;
        // TODO: test
        return serde_json::to_string(&value).unwrap();
	}
	
	pub fn write_out(&self, out: &mut io::Write) -> io::Result<()> {
		try!(out.write_all(self.to_string().as_bytes()));
		Ok(())
	}
	
}


/* -----------------  ----------------- */

use std::collections::HashMap;

pub struct JsonRpcDispatcher {
	pub dispatcher_map : HashMap<String, Box<Fn(&Map<String, Value>)>>, 
}

impl JsonRpcDispatcher {
	
	pub fn new() -> JsonRpcDispatcher {
		JsonRpcDispatcher { dispatcher_map : HashMap::new() }
	}
	
	pub fn dispatch(&self, request: &JsonRpcRequest) -> JsonRpcResult<()> {
		match 
			self.dispatcher_map.get(&request.method) 
		{
			Some(dispatcher_fn) => { 
				dispatcher_fn(& request.params);
				Ok(())
			}
			None => { 
				Err(JSON_RPC_MethodNotFound)
			}
		}
	}
	
}