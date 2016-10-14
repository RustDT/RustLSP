// Copyright 2016 Bruno Medeiros
//
// Licensed under the Apache License, Version 2.0 
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>. 
// This file may not be copied, modified, or distributed
// except according to those terms.


extern crate serde_json;
extern crate serde;


use json_util::*;

use std::fmt;
//use util::core::*;
use serde_json::Value;


/* ----------------- JSON-RPC 2.0 object types ----------------- */

#[derive(Debug, PartialEq, Clone)]
pub enum RpcId { Number(u64), String(String), Null, }

#[derive(Debug, PartialEq, Clone)]
/// A JSON RPC request, version 2.0
pub struct JsonRpcRequest {
	// ommited jsonrpc field, must be "2.0" when serialized
	//pub jsonrpc : String, 
	pub id : Option<RpcId>,
	pub method : String,
	pub params : JsonObject,
}

/// A JSON RPC response, version 2.0
/// Only one of 'result' or 'error' is defined
#[derive(Debug, PartialEq, Clone)]
pub struct JsonRpcResponse {
	// Rpc id. Note: spec requires key `id` to be present
	pub id : RpcId, 
	
	// field `result` or field `error`:
	pub result_or_error: JsonRpcResult_Or_Error,
}

#[derive(Debug, PartialEq, Clone)]
pub enum JsonRpcResult_Or_Error {
	Result(Value),
	Error(JsonRpcError)
}

#[derive(Debug, PartialEq, Clone)]
pub struct JsonRpcError {
	pub code : i64,
	pub message : String,
	pub data : Option<Value>,
}

/* ----------------- Impls ----------------- */

impl JsonRpcRequest {
	
	pub fn new(id_number : u64, method : String, params : JsonObject) -> JsonRpcRequest {
		JsonRpcRequest { 	
			id : Some(RpcId::Number(id_number)),
			method : method,
			params : params,
		} 
	}
	
}

impl JsonRpcResponse {
	
	pub fn new_result(id: RpcId, result: Value) -> JsonRpcResponse {
		JsonRpcResponse { id : id, result_or_error : JsonRpcResult_Or_Error::Result(result) }
	}
	
	pub fn new_error(id: RpcId, error: JsonRpcError) -> JsonRpcResponse {
		JsonRpcResponse { id : id, result_or_error : JsonRpcResult_Or_Error::Error(error) }
	}
	
}

impl JsonRpcError {
	
	pub fn new(code: i64, message: String) -> JsonRpcError {
		JsonRpcError { code : code, message : message, data : None }
	}
	
}

pub fn error_JSON_RPC_ParseError<T: fmt::Display>(error: T) -> JsonRpcError { 
	JsonRpcError::new(-32700, format!("Invalid JSON was received by the server: {}", error).to_string())
}
pub fn error_JSON_RPC_InvalidRequest() -> JsonRpcError { 
	JsonRpcError::new(-32600, "The JSON sent is not a valid Request object.".to_string())
}
pub fn error_JSON_RPC_MethodNotFound() -> JsonRpcError { 
	JsonRpcError::new(-32601, "The method does not exist / is not available.".to_string())
}
pub fn error_JSON_RPC_InvalidParams<T: fmt::Display>(error: T) -> JsonRpcError { 
	JsonRpcError::new(-32602, format!("Invalid method parameter(s): {}", error).to_string())
}
pub fn error_JSON_RPC_InternalError() -> JsonRpcError { 
	JsonRpcError::new(-32603, "Internal JSON-RPC error.".to_string())
}


/* -----------------  ----------------- */


impl serde::Serialize for RpcId {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
		where S: serde::Serializer,
	{
		match self {
			&RpcId::Null => serializer.serialize_none(),
			&RpcId::Number(number) => serializer.serialize_u64(number), 
			&RpcId::String(ref string) => serializer.serialize_str(string),
		}
	}
}




impl serde::Serialize for JsonRpcRequest {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
		where S: serde::Serializer
	{
		// TODO: need to investigate if elem_count = 4 is actually valid when id is missing
		// serializing to JSON seems to not be a problem, but there might be other issues
		let elem_count = 4;
		let mut state = try!(serializer.serialize_struct("JsonRpcRequest", elem_count)); 
		{
			try!(serializer.serialize_struct_elt(&mut state, "jsonrpc", "2.0"));
			if let Some(ref id) = self.id {
				try!(serializer.serialize_struct_elt(&mut state, "id", id));
			}
			try!(serializer.serialize_struct_elt(&mut state, "method", &self.method));
			try!(serializer.serialize_struct_elt(&mut state, "params", &self.params));
		}
		serializer.serialize_struct_end(state)
	}
}

impl serde::Serialize for JsonRpcResponse {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
		where S: serde::Serializer
	{
		let elem_count = 3;
		let mut state = try!(serializer.serialize_struct("JsonRpcResponse", elem_count));
		{
			try!(serializer.serialize_struct_elt(&mut state, "jsonrpc", "2.0"));
			try!(serializer.serialize_struct_elt(&mut state, "id", &self.id));
			
			match self.result_or_error {
				JsonRpcResult_Or_Error::Result(ref value) => {
					try!(serializer.serialize_struct_elt(&mut state, "result", &value));
				}
				JsonRpcResult_Or_Error::Error(ref json_rpc_error) => {
					try!(serializer.serialize_struct_elt(&mut state, "error", &json_rpc_error)); 
				}
			}
		}
		serializer.serialize_struct_end(state)
	}
}

impl serde::Serialize for JsonRpcError {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
		where S: serde::Serializer
	{
		let elem_count = 3;
		let mut state = try!(serializer.serialize_struct("JsonRpcError", elem_count)); 
		{
			try!(serializer.serialize_struct_elt(&mut state, "code", self.code));
			try!(serializer.serialize_struct_elt(&mut state, "message", &self.message));
			if let Some(ref data) = self.data {
				try!(serializer.serialize_struct_elt(&mut state, "data", data));
			}
		}
		serializer.serialize_struct_end(state)
	}
}

/* ----------------- Tests ----------------- */

#[cfg(test)]
pub mod tests {
	
	use super::*;
	use super::super::parse_jsonrpc_request;
	use util::tests::*;
	
	use serde;
	use serde_json;
	use serde_json::Value;
	use serde_json::builder::ObjectBuilder;
	use json_util::*;

	pub fn to_json<T: serde::Serialize>(value: &T) -> String {
		serde_json::to_string(value).unwrap()
	}
	
	pub fn check_error(result: JsonRpcError, expected: JsonRpcError) {
		assert_starts_with(&result.message, &expected.message);
		assert_eq!(result, JsonRpcError { message : result.message.clone(), .. expected }); 
	}
	
	#[test]
	fn test_parse_JsonRpcRequest() {
		
		let sample_params = unwrap_object_builder(ObjectBuilder::new()
			.insert("param", "2.0")
			.insert("foo", 123)
		);
		
		// Test invalid JSON
		check_error(parse_jsonrpc_request("{" ).unwrap_err(), error_JSON_RPC_ParseError(""));
		
		// Test invalid JsonRpcRequest
		let invalid_request = ObjectBuilder::new()
			.insert("jsonrpc", "2.0")
			.insert("id", 1)
			.insert("params", sample_params.clone())
			.build();
		
		let result = parse_jsonrpc_request(&serde_json::to_string(&invalid_request).unwrap()).unwrap_err();
		assert_eq!(result, error_JSON_RPC_InvalidRequest());
		
		// Test invalid JsonRpcRequest 2 - jsonrpc 1.0
		let invalid_request = ObjectBuilder::new()
			.insert("jsonrpc", "1.0")
			.insert("id", 1)
			.insert("method", "my method")
			.insert("params", sample_params.clone())
			.build();
		
		let result = parse_jsonrpc_request(&to_json(&invalid_request)).unwrap_err();
		assert_eq!(result, error_JSON_RPC_InvalidRequest());
		
		// Test basic JsonRpcRequest
		let request = JsonRpcRequest { 
			id : Some(RpcId::Number(1)), 
			method: "myMethod".to_string(), 
			params: sample_params.clone() 
		}; 
		
		let result = parse_jsonrpc_request(&to_json(&request)).unwrap();
		assert_eq!(request, result);
		
		// Test basic JsonRpcRequest, no params
		let request = ObjectBuilder::new()
			.insert("jsonrpc", "2.0")
			.insert("id", 1)
			.insert("method", "myMethod")
			.build();
		let result = parse_jsonrpc_request(&to_json(&request)).unwrap();
		assert_eq!(result, JsonRpcRequest { 
				id : Some(RpcId::Number(1)), 
				method : "myMethod".to_string(), 
				params : unwrap_object_builder(ObjectBuilder::new())
		});
		
		// Test JsonRpcRequest for notification
		let request = ObjectBuilder::new()
			.insert("jsonrpc", "2.0")
			.insert("method", "myNotification")
			.build();
		let result = parse_jsonrpc_request(&to_json(&request)).unwrap();
		assert_eq!(result, JsonRpcRequest { 
				id : None, // Test null id
				method : "myNotification".to_string(), 
				params : unwrap_object_builder(ObjectBuilder::new())
		});
		
	}

	#[test]
	fn test_JsonRpcResponse_serialize() {
		
		fn sample_json_obj(foo: u32) -> Value {
			ObjectBuilder::new().insert("foo", foo).build()
		}
		
		let response = JsonRpcResponse::new_result(RpcId::Null, sample_json_obj(100));
		let response = unwrap_object(serde_json::from_str(&to_json(&response)).unwrap());
		assert_equal(response, unwrap_object_builder(ObjectBuilder::new()
			.insert("jsonrpc", "2.0")
			.insert("id", RpcId::Null)
			.insert("result", sample_json_obj(100))
		));
		
		
		let response = JsonRpcResponse::new_result(RpcId::Number(123), sample_json_obj(200));
		let response = unwrap_object(serde_json::from_str(&to_json(&response)).unwrap());
		assert_equal(response, unwrap_object_builder(ObjectBuilder::new()
			.insert("jsonrpc", "2.0")
			.insert("id", 123)
			.insert("result", sample_json_obj(200))
		));
		
		let response = JsonRpcResponse::new_result(RpcId::Null, sample_json_obj(200));
		let response = unwrap_object(serde_json::from_str(&to_json(&response)).unwrap());
		assert_equal(response, unwrap_object_builder(ObjectBuilder::new()
			.insert("jsonrpc", "2.0")
			.insert("id", Value::Null)
			.insert("result", sample_json_obj(200))
		));
		
		let response = JsonRpcResponse::new_error(RpcId::String("321".to_string()), JsonRpcError{
			code: 5, message: "msg".to_string(), data: Some(sample_json_obj(300))
		});
		let response = unwrap_object(serde_json::from_str(&to_json(&response)).unwrap());
		assert_equal(response, unwrap_object_builder(ObjectBuilder::new()
			.insert("jsonrpc", "2.0")
			.insert("id", "321")
			.insert("error", unwrap_object_builder(ObjectBuilder::new()
				.insert("code", 5)
				.insert("message", "msg")
				.insert("data", sample_json_obj(300))
			))
		));
		
	}

}