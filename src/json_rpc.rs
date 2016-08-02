// Copyright 2016 Bruno Medeiros
//
// Licensed under the Apache License, Version 2.0 
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>. 
// This file may not be copied, modified, or distributed
// except according to those terms.


// WARNING: Rust newbie code ahead (-_-)'


#![allow(non_upper_case_globals)]

extern crate serde_json;

use self::serde_json::Map;
use self::serde_json::Value;

//use ::util::core::*;


/* ----------------- deserialize helpers ----------------- */

fn unwrap_object(ob: ObjectBuilder) -> Map<String, Value> {
    match ob.unwrap() {
    	Value::Object(o) => o ,
    	_ => { panic!() },
    }
}


trait JsonDeserializerHelper {
	
	fn new_request_deserialization_error(&self) -> JsonRpcError;
	
	fn obtain_Value(&mut self, mut json_map : &mut Map<String, Value>, key: & str) -> result::Result<Value, JsonRpcError> {
		let value = json_map.remove(key);
		match value {
			Some(value) => { Ok(value) }, 
			None => { return Err(self.new_request_deserialization_error()) }
		}
	}
	
	fn obtain_Value_or(&mut self, mut json_map : &mut Map<String, Value>, key: & str, default: & Fn() -> Value) 
		-> Value 
	{
		if let Some(value) = json_map.remove(key) {
			value
		} else {
			default()
		}
	}
	
	fn as_String(&mut self, value: Value) -> result::Result<String, JsonRpcError> {
		match value {
			Value::String(string) => Ok(string),
			_ => Err(self.new_request_deserialization_error()),
		}
	}
	
	fn as_Map(&mut self, value: Value) -> result::Result<Map<String, Value>, JsonRpcError> {
		match value {
			Value::Object(map) => Ok(map),
			_ => Err(self.new_request_deserialization_error()),
		}
	}
	
	fn as_u32(&mut self, value: Value) -> result::Result<u32, JsonRpcError> {
		match value {
			Value::I64(num) => Ok(num as u32), // TODO: check for truncation
			Value::U64(num) => Ok(num as u32), // TODO: check for truncation
			_ => Err(self.new_request_deserialization_error()) ,
		}
	}
	
	
	fn obtain_String(&mut self, json_map : &mut Map<String, Value>, key: &str) 
		-> result::Result<String, JsonRpcError> 
	{
		let value = try!(self.obtain_Value(json_map, key));
		self.as_String(value)
	}
	
	fn obtain_Map(&mut self, json_map : &mut Map<String, Value>, key: &str) -> result::Result<Map<String, Value>, JsonRpcError> {
		let value = try!(self.obtain_Value(json_map, key));
		self.as_Map(value)
	}
	
	fn obtain_Map_or(&mut self, json_map : &mut Map<String, Value>, key: &str, default: & Fn() -> Map<String, Value>) 
		-> result::Result<Map<String, Value>, JsonRpcError> 
	{
		let value = self.obtain_Value_or(json_map, key, &|| { Value::Object(default()) });
		self.as_Map(value)
	}
	
	fn obtain_u32(&mut self, json_map: &mut Map<String, Value>, key: &str) -> result::Result<u32, JsonRpcError> {
		let value = try!(self.obtain_Value(json_map, key));
		self.as_u32(value)
	}

}

struct JsonRequestDeserializerHelper {
	
}

//impl<JsonRpcError> JsonDeserializerHelper<JsonRpcError> for JsonRequestDeserializerHelper {
//	
//}

impl JsonDeserializerHelper for JsonRequestDeserializerHelper {
	
	fn new_request_deserialization_error(&self) -> JsonRpcError {
		return JSON_RPC_InvalidRequest;
	}
	
}


/* ----------------- JSON RPC ----------------- */


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
	let mut json_result : Value = match 
		serde_json::from_str(message) 
	{
		Ok(ok) => { ok } 
		Err(error) => { 
			return Err(JSON_RPC_ParseError);
		}
	};
	
    parse_jsonrpc_request_json(&mut json_result)
}

pub fn parse_jsonrpc_request_json(request_json: &mut Value) -> JsonRpcResult<JsonRpcRequest> {    
    
    let mut json_request_map : &mut Map<String, Value> =
    match request_json {
    	&mut Value::Object(ref mut map) => map ,
    	_ => { return Err(JSON_RPC_InvalidRequest) },
    };
    parse_jsonrpc_request_jsonObject(&mut json_request_map)
}

pub fn parse_jsonrpc_request_jsonObject(mut request_map: &mut Map<String, Value>) -> JsonRpcResult<JsonRpcRequest> {
	    
    let mut helper = JsonRequestDeserializerHelper { };
    
    let jsonrpc = try!(helper.obtain_String(&mut request_map, "jsonrpc"));
    let id = try!(helper.obtain_u32(&mut request_map, "id"));
    let method = try!(helper.obtain_String(&mut request_map, "method"));
    let params = try!(helper.obtain_Map_or(&mut request_map, "params", &|| unwrap_object(ObjectBuilder::new())));
    
    let jsonrpc_request = JsonRpcRequest { jsonrpc : jsonrpc, id : id, method : method, params : params}; 
    
    Ok(jsonrpc_request)
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

/* ----------------- Test ----------------- */

#[test]
fn parse_jsonrpc_request_json_Test() {
	
	use self::serde_json::builder::ObjectBuilder;
	
	let sample_params = unwrap_object(ObjectBuilder::new()
	    .insert("param", "2.0")
        .insert("foo", 123)
    );
	
	// Test invalid JSON
	assert_eq!(parse_jsonrpc_request("{" ).unwrap_err(), JSON_RPC_ParseError);
	
	// Test invalid JsonRpcRequest
    let mut invalid_request = ObjectBuilder::new()
	    .insert("jsonrpc", "2.0")
        .insert("id", 1)
        .insert("params", sample_params.clone())
        .unwrap();
	
    let result = parse_jsonrpc_request_json(&mut invalid_request).unwrap_err();    
	assert_eq!(result, JSON_RPC_InvalidRequest);
	
    // Test basic JsonRpcRequest
    let mut request = ObjectBuilder::new()
	    .insert("jsonrpc", "2.0")
        .insert("id", 1)
        .insert("method", "myMethod")
        .insert("params", sample_params.clone())
        .unwrap();
    
    let result = parse_jsonrpc_request_json(&mut request).unwrap();
    
	assert_eq!(result, JsonRpcRequest { 
			jsonrpc : "2.0".to_string(), 
			id : 1, 
			method : "myMethod".to_string(), 
			params : sample_params.clone(),
	});
	
	
	// Test basic JsonRpcRequest, no params
    let mut request = ObjectBuilder::new()
	    .insert("jsonrpc", "2.0")
        .insert("id", 1)
        .insert("method", "myMethod")
//       .insert("params", sample_params.clone())
        .unwrap();
        
	let result = parse_jsonrpc_request_json(&mut request).unwrap();
    assert_eq!(result, JsonRpcRequest { 
			jsonrpc : "2.0".to_string(), 
			id : 1, 
			method : "myMethod".to_string(), 
			params : unwrap_object(ObjectBuilder::new())
	});
	
}