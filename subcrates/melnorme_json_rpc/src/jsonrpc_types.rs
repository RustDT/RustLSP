// Copyright 2016 Bruno Medeiros
//
// Licensed under the Apache License, Version 2.0 
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>. 
// This file may not be copied, modified, or distributed
// except according to those terms.


extern crate serde_json;
extern crate serde;


use std::fmt;

use serde::de::Visitor;
use serde::Error;
use serde_json::Value;

use util::core::GResult;
use json_util::*;


/* ----------------- Id ----------------- */

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// A JSON RPC Id
/// Note: only supports u64 numbers
pub enum Id { Number(u64), String(String), Null, }

impl serde::Serialize for Id {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
		where S: serde::Serializer,
	{
		match *self {
			Id::Null => serializer.serialize_none(),
			Id::Number(number) => serializer.serialize_u64(number), 
			Id::String(ref string) => serializer.serialize_str(string),
		}
	}
}

impl serde::Deserialize for Id {
	fn deserialize<DE>(deserializer: &mut DE) -> Result<Self, DE::Error>
		where DE: serde::Deserializer 
	{
		deserializer.deserialize(IdVisitor)
	}
}

struct IdVisitor;

impl Visitor for IdVisitor {
	type Value = Id;
	
    fn visit_unit<E>(&mut self) -> Result<Self::Value, E> where E: Error,
    {
		Ok(Id::Null)
    }	
	
	fn visit_u64<E>(&mut self, value: u64) -> Result<Self::Value, E> where E: Error,
    {
        Ok(Id::Number(value))
    }
	
    fn visit_str<E>(&mut self, value: &str) -> Result<Self::Value, E> where E: Error,
    {
        Ok(Id::String(value.to_string()))
    }
}

#[test]
fn test_Id() {
    use json_util::test_util::*;
	
	check_deser(Id::Null);
	check_deser(Id::Number(123));
	check_deser(Id::String("123".into()));
    check_deser(Id::String("".into()));
    check_deser(Id::String("foo".into()));
    
    // FIXME better handling of non-u64 numbers?
//    assert_eq!(from_json::<Id>("-123"), Id::Number(123)); 
}

/* -----------------  Request  ----------------- */

/// A JSON RPC request, version 2.0
#[derive(Debug, PartialEq, Clone)]
pub struct Request {
	// ommited jsonrpc field, must be "2.0" when serialized
	//pub jsonrpc : String, 
	pub id : Option<Id>,
	pub method : String,
	pub params : RequestParams,
}

impl Request {
	pub fn new(id_number: u64, method: String, params: JsonObject) -> Request {
		Request { 	
			id : Some(Id::Number(id_number)),
			method : method,
			params : RequestParams::Object(params),
		} 
	}
	
	pub fn to_message(self) -> Message {
		Message::Request(self)
	}
}

impl serde::Serialize for Request {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
		where S: serde::Serializer
	{
		// TODO: need to investigate if elem_count = 4 is actually valid when id is missing
		// serializing to JSON seems to not be a problem, but there might be other issues
		let elem_count = 4;
		let mut state = try!(serializer.serialize_struct("Request", elem_count)); 
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

#[derive(Debug, PartialEq, Clone)]
pub enum RequestParams {
	Object(JsonObject),
	Array(Vec<Value>),
	None,
}

impl RequestParams {
	pub fn into_value(self) -> Value {
		// Note, we could use serde_json::to_value(&params) but that is less efficient:
		// it reserializes the value, instead of just obtaining the underlying one 
		
		match self {
			RequestParams::Object(object) => Value::Object(object),
			RequestParams::Array(array) => Value::Array(array),
			RequestParams::None => Value::Null,
		}
	}
}

impl serde::Serialize for RequestParams {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
		where S: serde::Serializer
	{
		match *self {
			RequestParams::Object(ref object) => object.serialize(serializer),
			RequestParams::Array(ref array) => array.serialize(serializer),
			RequestParams::None => serializer.serialize_none(),
		}
	}
}

/* ----------------- Response ----------------- */

/// A JSON RPC response, version 2.0.
/// Only one of 'result' or 'error' is defined.
#[derive(Debug, PartialEq, Clone)]
pub struct Response {
	// Rpc id. Note: spec requires key `id` to be present
	pub id : Id, 
	// field `result` or field `error`:
	pub result_or_error: ResponseResult,
}

/// The result-or-error part of JSON RPC response.
#[derive(Debug, PartialEq, Clone)]
pub enum ResponseResult {
	Result(Value),
	Error(RequestError)
}


impl Response {
	pub fn new_result(id: Id, result: Value) -> Response {
		Response { id : id, result_or_error : ResponseResult::Result(result) }
	}
	
	pub fn new_error(id: Id, error: RequestError) -> Response {
		Response { id : id, result_or_error : ResponseResult::Error(error) }
	}
	
	pub fn to_message(self) -> Message {
		Message::Response(self)
	}
}


impl serde::Serialize for Response {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
		where S: serde::Serializer
	{
		let elem_count = 3;
		let mut state = try!(serializer.serialize_struct("Response", elem_count));
		{
			try!(serializer.serialize_struct_elt(&mut state, "jsonrpc", "2.0"));
			try!(serializer.serialize_struct_elt(&mut state, "id", &self.id));
			
			match self.result_or_error {
				ResponseResult::Result(ref value) => {
					try!(serializer.serialize_struct_elt(&mut state, "result", &value));
				}
				ResponseResult::Error(ref json_rpc_error) => {
					try!(serializer.serialize_struct_elt(&mut state, "error", &json_rpc_error)); 
				}
			}
		}
		serializer.serialize_struct_end(state)
	}
}

/* -----------------  Error  ----------------- */

#[derive(Debug, PartialEq, Clone)]
pub struct RequestError {
	pub code : i64,
	pub message : String,
	pub data : Option<Value>,
}

impl RequestError {
	pub fn new(code: i64, message: String) -> RequestError {
		RequestError { code : code, message : message, data : None }
	}
}

pub fn error_JSON_RPC_ParseError<T: fmt::Display>(error: T) -> RequestError { 
	RequestError::new(-32700, format!("Invalid JSON was received by the server: {}", error).to_string())
}
pub fn error_JSON_RPC_InvalidRequest<T: fmt::Display>(error: T) -> RequestError { 
	RequestError::new(-32600, format!("The JSON sent is not a valid Request object: {}", error).to_string())
}
pub fn error_JSON_RPC_MethodNotFound() -> RequestError { 
	RequestError::new(-32601, "The method does not exist / is not available.".to_string())
}
pub fn error_JSON_RPC_InvalidParams<T: fmt::Display>(error: T) -> RequestError { 
	RequestError::new(-32602, format!("Invalid method parameter(s): {}", error).to_string())
}
pub fn error_JSON_RPC_InternalError() -> RequestError { 
	RequestError::new(-32603, "Internal JSON-RPC error.".to_string())
}

pub fn error_JSON_RPC_InvalidResponse<T: fmt::Display>(error: T) -> RequestError { 
	RequestError::new(-32000, format!("Invalid method response: {}", error).to_string())
}

impl serde::Serialize for RequestError {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
		where S: serde::Serializer
	{
		let elem_count = 3;
		let mut state = try!(serializer.serialize_struct("RequestError", elem_count)); 
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

/* -----------------  JSON-RPC custom deserialization  ----------------- */

struct JsonRequestDeserializerHelper;

impl JsonDeserializerHelper<RequestError> for JsonRequestDeserializerHelper {
	
	fn new_error(&self, error_message: &str) -> RequestError {
		return error_JSON_RPC_InvalidRequest(error_message.to_string());
	}
	
}

pub type JsonRpcParseResult<T> = Result<T, RequestError>;


pub fn parse_jsonrpc_request(message: &str) -> JsonRpcParseResult<Request> {
	let json_value : Value = 
	match serde_json::from_str(message) 
	{
		Ok(ok) => { ok } 
		Err(error) => { 
			return Err(error_JSON_RPC_ParseError(error));
		}
	};
	
	let mut helper = JsonRequestDeserializerHelper { };
	let mut json_request_obj : JsonObject = try!(helper.as_Object(json_value));
		
	parse_jsonrpc_request_jsonObject(&mut json_request_obj)
}

pub fn parse_jsonrpc_request_jsonObject(mut request_map: &mut JsonObject) -> JsonRpcParseResult<Request> {
	let mut helper = JsonRequestDeserializerHelper { };
	
	let jsonrpc = try!(helper.obtain_String(&mut request_map, "jsonrpc"));
	if jsonrpc != "2.0" {
		return Err(error_JSON_RPC_InvalidRequest(r#"Property `jsonrpc` is not "2.0". "#))
	}
	let id = request_map.remove("id");
	let id = try!(id.map_or(Ok(None), parse_jsonrpc_id));
	let method = try!(helper.obtain_String(&mut request_map, "method"));
	let params = try!(helper.obtain_Value(&mut request_map, "params"));
	
	let params : RequestParams = match to_jsonrpc_params(params) {
		Ok(ok) => ok,
		Err(error) => return Err(error_JSON_RPC_InvalidRequest(error)),
	};
	
	let jsonrpc_request = Request { id : id, method : method, params : params }; 
	
	Ok(jsonrpc_request)
}

pub fn to_jsonrpc_params(params: Value) -> GResult<RequestParams> {
	match params {
		Value::Object(object) => Ok(RequestParams::Object(object)),
		Value::Array(array) => Ok(RequestParams::Array(array)),
		Value::Null => Ok(RequestParams::None),
		_ => Err("Property `params` not an Object, Array, or null.".into()),
	}
}

pub fn parse_jsonrpc_id(id: Value) -> JsonRpcParseResult<Option<Id>> {
   	serde_json::from_value(id)
    	.map_err(|err| error_JSON_RPC_InvalidRequest(format!("Invalid id: {}", err)))
}

/* -----------------  Message  ----------------- */

#[derive(Debug, PartialEq, Clone)]
pub enum Message {
	Request(Request),
	Response(Response),
}

impl serde::Serialize for Message {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
		where S: serde::Serializer
	{
		match *self {
			Message::Request(ref request) => request.serialize(serializer),
			Message::Response(ref response) => response.serialize(serializer),
		}
	}
}

/* ----------------- Tests ----------------- */

#[cfg(test)]
pub mod tests {
	
	use super::*;
	use util::tests::*;
	
	use serde;
	use serde_json;
	use serde_json::Value;
	use serde_json::builder::ObjectBuilder;
	use json_util::*;
	use json_util::test_util::*;

	pub fn check_error(result: RequestError, expected: RequestError) {
		assert_starts_with(&result.message, &expected.message);
		assert_eq!(result, RequestError { message : result.message.clone(), .. expected }); 
	}
	
	#[test]
	fn test__jsonrpc_params() {
		
		let sample_obj = unwrap_object_builder(ObjectBuilder::new().insert("xxx", 123));
		let sample_string = Value::String("blah".into());
		
		test__jsonrpc_params_serde(RequestParams::Object(sample_obj.clone()));
		test__jsonrpc_params_serde(RequestParams::Array(vec![sample_string.clone(), sample_string]));
		test__jsonrpc_params_serde(RequestParams::None);
	}
	
	fn test__jsonrpc_params_serde(params: RequestParams) {
		let params_string = to_json(&params);
		let params2 = to_jsonrpc_params(serde_json::from_str(&params_string).unwrap()).unwrap();
		
		assert_equal(params, params2);
	}
	
	#[test]
	fn test__parse_jsonrpc_request() {
		
		let sample_params = unwrap_object_builder(ObjectBuilder::new()
			.insert("param", "2.0")
			.insert("foo", 123)
		);
		
		// Test invalid JSON
		check_error(parse_jsonrpc_request("{" ).unwrap_err(), error_JSON_RPC_ParseError(""));
		
		assert_equal(
			parse_jsonrpc_request("{ }"),
			Err(error_JSON_RPC_InvalidRequest("Property `jsonrpc` is missing."))
		);
		assert_equal(
			parse_jsonrpc_request(r#"{ "jsonrpc": "1.0" }"#),
			Err(error_JSON_RPC_InvalidRequest(r#"Property `jsonrpc` is not "2.0". "#))
		);
		
		assert_equal(
			parse_jsonrpc_request(r#"{ "jsonrpc": "2.0" }"#),
			Err(error_JSON_RPC_InvalidRequest("Property `method` is missing."))
		);
		assert_equal(
			parse_jsonrpc_request(r#"{ "jsonrpc": "2.0", "method":null }"#),
			Err(error_JSON_RPC_InvalidRequest("Value `null` is not a String."))
		);
		
		assert_equal(
			parse_jsonrpc_request(r#"{ "jsonrpc": "2.0", "method":"xxx" }"#),
			Err(error_JSON_RPC_InvalidRequest("Property `params` is missing."))
		);
		
		// Test valid request with params = null
		assert_equal(
			parse_jsonrpc_request(r#"{ "jsonrpc": "2.0", "method":"xxx", "params":null }"#),
			Ok(Request { id : None, method : "xxx".into(), params : RequestParams::None, }) 
		);
		
		// --- Test serialization ---
		 
		// basic Request
		let request = Request::new(1, "myMethod".to_string(), sample_params.clone()); 
		let result = parse_jsonrpc_request(&to_json(&request)).unwrap();
		assert_eq!(request, result);
		
		// Test basic Request, no params
		let request = Request { id : None, method : "myMethod".to_string(), params : RequestParams::None, };
		let result = parse_jsonrpc_request(&to_json(&request)).unwrap();
		assert_eq!(result, request);
		
		// Test Request with no id
		let sample_array_params = RequestParams::Array(vec![]);
		let request = Request { id : None, method : "myMethod".to_string(), params : sample_array_params, };  
		let result = parse_jsonrpc_request(&to_json(&request)).unwrap();
		assert_eq!(result, request);
	}

	#[test]
	fn test_Response_serialize() {
		
		fn sample_json_obj(foo: u32) -> Value {
			ObjectBuilder::new().insert("foo", foo).build()
		}
		
		let response = Response::new_result(Id::Null, sample_json_obj(100));
		let response = unwrap_object(serde_json::from_str(&to_json(&response)).unwrap());
		assert_equal(response, unwrap_object_builder(ObjectBuilder::new()
			.insert("jsonrpc", "2.0")
			.insert("id", Id::Null)
			.insert("result", sample_json_obj(100))
		));
		
		
		let response = Response::new_result(Id::Number(123), sample_json_obj(200));
		let response = unwrap_object(serde_json::from_str(&to_json(&response)).unwrap());
		assert_equal(response, unwrap_object_builder(ObjectBuilder::new()
			.insert("jsonrpc", "2.0")
			.insert("id", 123)
			.insert("result", sample_json_obj(200))
		));
		
		let response = Response::new_result(Id::Null, sample_json_obj(200));
		let response = unwrap_object(serde_json::from_str(&to_json(&response)).unwrap());
		assert_equal(response, unwrap_object_builder(ObjectBuilder::new()
			.insert("jsonrpc", "2.0")
			.insert("id", Value::Null)
			.insert("result", sample_json_obj(200))
		));
		
		let response = Response::new_error(Id::String("321".to_string()), RequestError{
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