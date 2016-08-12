// Copyright 2016 Bruno Medeiros
//
// Licensed under the Apache License, Version 2.0 
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>. 
// This file may not be copied, modified, or distributed
// except according to those terms.


// WARNING: Rust newbie code ahead (-_-)'


#![allow(non_upper_case_globals)]

use serde_json;

use serde_json::Serializer;
use serde_json::Map;
use serde_json::Value;

//use ::util::core::*;
use std::result::Result;


/* ----------------- deserialize helpers ----------------- */

pub fn unwrap_object(ob: ObjectBuilder) -> Map<String, Value> {
	match ob.build() {
		Value::Object(o) => o ,
		_ => { panic!() },
	}
}

// FIXME: parameterize trait 
trait JsonDeserializerHelper {
	
	fn new_request_deserialization_error(&self) -> JsonRpcError;
	
	fn obtain_Value(&mut self, mut json_map : &mut Map<String, Value>, key: & str) 
		-> Result<Value, JsonRpcError> 
	{
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
	
	fn as_String(&mut self, value: Value) -> Result<String, JsonRpcError> {
		match value {
			Value::String(string) => Ok(string),
			_ => Err(self.new_request_deserialization_error()),
		}
	}
	
	fn as_Map(&mut self, value: Value) -> Result<Map<String, Value>, JsonRpcError> {
		match value {
			Value::Object(map) => Ok(map),
			_ => Err(self.new_request_deserialization_error()),
		}
	}
	
	fn as_u32(&mut self, value: Value) -> Result<u32, JsonRpcError> {
		match value {
			Value::I64(num) => Ok(num as u32), // TODO: check for truncation
			Value::U64(num) => Ok(num as u32), // TODO: check for truncation
			_ => Err(self.new_request_deserialization_error()) ,
		}
	}
	
	
	fn obtain_String(&mut self, json_map : &mut Map<String, Value>, key: &str) 
		-> Result<String, JsonRpcError> 
	{
		let value = try!(self.obtain_Value(json_map, key));
		self.as_String(value)
	}
	
	fn obtain_Map(&mut self, json_map : &mut Map<String, Value>, key: &str) 
		-> Result<Map<String, Value>, JsonRpcError> 
	{
		let value = try!(self.obtain_Value(json_map, key));
		self.as_Map(value)
	}
	
	fn obtain_Map_or(&mut self, json_map : &mut Map<String, Value>, key: &str, default: & Fn() -> Map<String, Value>) 
		-> Result<Map<String, Value>, JsonRpcError> 
	{
		let value = self.obtain_Value_or(json_map, key, &|| { Value::Object(default()) });
		self.as_Map(value)
	}
	
	fn obtain_u32(&mut self, json_map: &mut Map<String, Value>, key: &str) 
		-> Result<u32, JsonRpcError> 
	{
		let value = try!(self.obtain_Value(json_map, key));
		self.as_u32(value)
	}

}

/* ----------------- JSON RPC ----------------- */

#[derive(Debug, PartialEq)]
pub enum RpcId { Number(u64), String(String), }

#[derive(Debug, PartialEq)]
/// A JSON RPC request, version 2.0
pub struct JsonRpcRequest {
	// ommited jsonrpc field, must be "2.0"
	//pub jsonrpc : String, 
	pub id : Option<RpcId>,
	pub method : String,
	pub params : Map<String, Value>,
}

/// A JSON RPC response, version 2.0
/// Only one of 'result' or 'error' is defined
#[derive(Debug, PartialEq)]
pub struct JsonRpcResponse {
	pub id : RpcId,
	pub result : Option<Value>,
	pub error: Option<JsonRpcError>,
}

#[derive(Debug, PartialEq)]
pub struct JsonRpcError {
	pub code : i64,
	pub message : String,
	pub data : Option<Value>,
}

impl JsonRpcError {
	pub fn new(code : i64, message : String) -> JsonRpcError {
		JsonRpcError { code: code, message: message, data: None }
	}
}

pub fn error_JSON_RPC_ParseError() -> JsonRpcError { 
	JsonRpcError::new(-32700, "Invalid JSON was received by the server.".to_string())
}
pub fn error_JSON_RPC_InvalidRequest() -> JsonRpcError { 
	JsonRpcError::new(-32600, "The JSON sent is not a valid Request object.".to_string())
}
pub fn error_JSON_RPC_MethodNotFound() -> JsonRpcError { 
	JsonRpcError::new(-32601, "The method does not exist / is not available.".to_string())
}
pub fn error_JSON_RPC_InvalidParams() -> JsonRpcError { 
	JsonRpcError::new(-32602, "Invalid method parameter(s).".to_string())
}
pub fn error_JSON_RPC_InternalError() -> JsonRpcError { 
	JsonRpcError::new(-32603, "Internal JSON-RPC error.".to_string())
}



pub type JsonRpcResult<T> = Result<T, JsonRpcError>;

struct JsonRequestDeserializerHelper {
	
}

impl JsonDeserializerHelper for JsonRequestDeserializerHelper {
	
	fn new_request_deserialization_error(&self) -> JsonRpcError {
		return error_JSON_RPC_InvalidRequest();
	}
	
}

use serde;
use serde::Serialize;

impl serde::Serialize for RpcId {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer,
    {
    	match self {
			&RpcId::Number(number) => serializer.serialize_u64(number), 
			&RpcId::String(ref string) => serializer.serialize_str(string),
		}
    }
}



impl serde::Serialize for JsonRpcRequest {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer
    {
    	// TODO: need to investigate if struct count = 4 is actually valid when id is missing
    	// serializing to JSON seems to not be a problem, but there might be other issues
        let mut state = try!(serializer.serialize_struct("JsonRpcRequest", 4));
        try!(serializer.serialize_struct_elt(&mut state, "jsonrpc", "2.0"));
        if let Some(ref id) = self.id {
        	try!(serializer.serialize_struct_elt(&mut state, "id", id));
		}
        try!(serializer.serialize_struct_elt(&mut state, "method", &self.method));
        try!(serializer.serialize_struct_elt(&mut state, "params", &self.params));
        serializer.serialize_struct_end(state)
    }
}

// TODO: review code below, probably a way to shorten this
impl RpcId {
	pub fn to_value(&self) -> Value {
		serde_json::to_value(&self)
	}
}
impl JsonRpcRequest {
	pub fn to_value(&self) -> Value {
		serde_json::to_value(&self)
	}
}


pub fn parse_jsonrpc_request(message: &str) -> JsonRpcResult<JsonRpcRequest> {
	let mut json_result : Value = match 
		serde_json::from_str(message) 
	{
		Ok(ok) => { ok } 
		Err(error) => { 
			return Err(error_JSON_RPC_ParseError());
		}
	};
	
	parse_jsonrpc_request_json(&mut json_result)
}

pub fn parse_jsonrpc_request_json(request_json: &mut Value) -> JsonRpcResult<JsonRpcRequest> {    
	
	let mut json_request_map : &mut Map<String, Value> =
	match request_json {
		&mut Value::Object(ref mut map) => map ,
		_ => { return Err(error_JSON_RPC_InvalidRequest()) },
	};
	parse_jsonrpc_request_jsonObject(&mut json_request_map)
}

pub fn parse_jsonrpc_request_jsonObject(mut request_map: &mut Map<String, Value>) -> JsonRpcResult<JsonRpcRequest> {
	
	let mut helper = JsonRequestDeserializerHelper { };
	
	let jsonrpc = try!(helper.obtain_String(&mut request_map, "jsonrpc"));
	if jsonrpc != "2.0" {
		return Err(error_JSON_RPC_InvalidRequest())
	}
	let id = try!(parse_jsonrpc_request_id(request_map.remove("id")));
	let method = try!(helper.obtain_String(&mut request_map, "method"));
	let params = try!(helper.obtain_Map_or(&mut request_map, "params", &|| unwrap_object(ObjectBuilder::new())));
	
	let jsonrpc_request = JsonRpcRequest { id : id, method : method, params : params}; 
	
	Ok(jsonrpc_request)
}

pub fn parse_jsonrpc_request_id(id: Option<Value>) -> JsonRpcResult<Option<RpcId>> {
	let id : Value = match id {
		None => return Ok(None),
		Some(id) => id,
	};
	match id {
		Value::I64(number) => Ok(Some(RpcId::Number(number as u64))), // FIXME truncation
		Value::U64(number) => Ok(Some(RpcId::Number(number))),
		Value::String(string) => Ok(Some(RpcId::String(string))),
		Value::Null => Ok(None),
		_ => Err(error_JSON_RPC_InvalidRequest()),
	}
}


use std::io;
use serde_json::builder::ObjectBuilder;


impl JsonRpcError {
	
	pub fn to_string(&self) -> String {
		let value = ObjectBuilder::new()
			.insert("code", self.code)
			.insert("message", &self.message)
			.build()
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
	pub dispatcher_map : HashMap<String, Box<Fn(Map<String, Value>)>>, 
}

impl JsonRpcDispatcher {
	
	pub fn new() -> JsonRpcDispatcher {
		JsonRpcDispatcher { dispatcher_map : HashMap::new() }
	}
	
	pub fn dispatch(&self, request: JsonRpcRequest) -> JsonRpcResult<()> {
		match 
			self.dispatcher_map.get(&request.method) 
		{
			Some(dispatcher_fn) => { 
				dispatcher_fn(request.params);
				Ok(())
			}
			None => { 
				Err(error_JSON_RPC_MethodNotFound())
			}
		}
	}
	
}

/* ----------------- Test ----------------- */

#[test]
fn parse_jsonrpc_request_json_Test() {
	
	use serde_json::builder::ObjectBuilder;
	
	let sample_params = unwrap_object(ObjectBuilder::new()
		.insert("param", "2.0")
		.insert("foo", 123)
	);
	
	// Test invalid JSON
	assert_eq!(parse_jsonrpc_request("{" ).unwrap_err(), error_JSON_RPC_ParseError());
	
	// Test invalid JsonRpcRequest
	let mut invalid_request = ObjectBuilder::new()
		.insert("jsonrpc", "2.0")
		.insert("id", 1)
		.insert("params", sample_params.clone())
		.build();
	
	let result = parse_jsonrpc_request_json(&mut invalid_request).unwrap_err();    
	assert_eq!(result, error_JSON_RPC_InvalidRequest());
	
	// Test invalid JsonRpcRequest 2 - jsonrpc 1.0
	let mut invalid_request = ObjectBuilder::new()
		.insert("jsonrpc", "1.0")
		.insert("id", 1)
		.insert("method", "my method")
		.insert("params", sample_params.clone())
		.build();
	
	let result = parse_jsonrpc_request_json(&mut invalid_request).unwrap_err();    
	assert_eq!(result, error_JSON_RPC_InvalidRequest());
	
	// Test basic JsonRpcRequest
	let request = JsonRpcRequest { 
		id : Some(RpcId::Number(1)), 
		method: "myMethod".to_string(), 
		params: sample_params.clone() 
	}; 
	
	let result = parse_jsonrpc_request_json(&mut request.to_value()).unwrap();
	assert_eq!(request, result);
	
	// Test basic JsonRpcRequest, no params
	let mut request = ObjectBuilder::new()
		.insert("jsonrpc", "2.0")
		.insert("id", 1)
		.insert("method", "myMethod")
//		.insert("params", sample_params.clone())
		.build();
	
	let result = parse_jsonrpc_request_json(&mut request).unwrap();
	assert_eq!(result, JsonRpcRequest { 
			id : Some(RpcId::Number(1)), 
			method : "myMethod".to_string(), 
			params : unwrap_object(ObjectBuilder::new())
	});
	
}