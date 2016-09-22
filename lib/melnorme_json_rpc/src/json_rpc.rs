// Copyright 2016 Bruno Medeiros
//
// Licensed under the Apache License, Version 2.0 
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>. 
// This file may not be copied, modified, or distributed
// except according to those terms.


#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

extern crate serde_json;
extern crate serde;
extern crate melnorme_util as util;

pub mod json_util;
pub mod service_util;
pub mod output_agent;

/* -----------------  ----------------- */

use util::core::*;

use serde_json::Map;
use serde_json::Value;
use serde_json::builder::ObjectBuilder;

use std::io;
use std::io::Write;
use std::collections::HashMap;
use std::result::Result;

use service_util::ServiceError;
use service_util::ServiceHandler;
use service_util::Provider;

use json_util::*;


/* ----------------- JSON RPC ----------------- */

#[derive(Debug, PartialEq, Clone)]
pub enum RpcId { Number(u64), String(String), }

#[derive(Debug, PartialEq, Clone)]
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
#[derive(Debug, PartialEq, Clone)]
pub struct JsonRpcResponse {
	pub id : Option<RpcId>,
//	pub result : Option<Value>,
//	pub error: Option<JsonRpcError>,
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

impl JsonRpcError {
	
	pub fn new(code: i64, message: String) -> JsonRpcError {
		JsonRpcError { code : code, message : message, data : None }
	}
	
}

impl JsonRpcResponse {
	
	pub fn new_result(id: Option<RpcId>, result: Value) -> JsonRpcResponse {
		JsonRpcResponse { id : id, result_or_error : JsonRpcResult_Or_Error::Result(result) }
	}
	
	pub fn new_error(id: Option<RpcId>, error: JsonRpcError) -> JsonRpcResponse {
		JsonRpcResponse { id : id, result_or_error : JsonRpcResult_Or_Error::Error(error) }
	}
	
}

/* -----------------  ----------------- */

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

impl JsonDeserializerHelper<JsonRpcError> for JsonRequestDeserializerHelper {
	
	fn new_request_deserialization_error(&self) -> JsonRpcError {
		return error_JSON_RPC_InvalidRequest();
	}
	
}

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


// TODO: review code below, probably a way to shorten this
impl RpcId {
	pub fn to_value(&self) -> Value {
		serde_json::to_value(&self)
	}
}
impl JsonRpcRequest {
	
	pub fn new(id_number : u64, method : String, params : Map<String, Value>) -> JsonRpcRequest {
		JsonRpcRequest { 	
			id : Some(RpcId::Number(id_number)),
			method : method,
			params : params,
		} 
	}
	
	pub fn to_value(&self) -> Value {
		serde_json::to_value(&self)
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
		let elem_count = 2;
		let mut state = try!(serializer.serialize_struct("JsonRpcRequest", elem_count));
		{
			try!(serializer.serialize_struct_elt(&mut state, "jsonrpc", "2.0"));
			match self.result_or_error {
				//FIXME: test
				JsonRpcResult_Or_Error::Result(ref value) => {
					try!(serializer.serialize_struct_elt(&mut state, "result", &value));
				}
				JsonRpcResult_Or_Error::Error(ref json_rpc_error) => {
					/* FIXME: BUG here "result" */
					try!(serializer.serialize_struct_elt(&mut state, "result", &json_rpc_error)); 
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

/* -----------------  ----------------- */


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
	let params = try!(helper.obtain_Map_or(&mut request_map, "params", &|| new_object()));
	
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

use output_agent::OutputAgent;
use output_agent::OutputAgentTask;

pub type DispatcherFn = Fn(Map<String, Value>) -> Option<JsonRpcResult_Or_Error>;

pub struct JsonRpcEndpoint {
	pub dispatcher_map : HashMap<String, Box<DispatcherFn>>,
	pub output_agent : OutputAgent,
}

pub fn post_response(output_agent : &mut OutputAgent, response : JsonRpcResponse) {
	
	let task : OutputAgentTask = Box::new(move |mut out_stream| {
		/* FIXME: log */ 
		println!("Handle: {:?}", response);
		
		let res = serde_json::to_writer(&mut out_stream, &response);
		
		if let Err(error) = res {
			// TODO log
			// FIXME handle output stream write error by shutting down
			writeln!(&mut std::io::stderr(), "Error writing RPC response: {}", error).expect(
				"Failed writing to stderr");
		}
		
	});
	output_agent.submit_task(task);
}
	

impl JsonRpcEndpoint {
	
	pub fn new<T : io::Write + Send + 'static>(out_stream : Box<T>) -> JsonRpcEndpoint {
		
		let output_agent = OutputAgent::new(out_stream);
		
		JsonRpcEndpoint { dispatcher_map : HashMap::new() , output_agent : output_agent }
	}
	
	pub fn is_shutdown(& self) -> bool {
		self.output_agent.is_shutdown()
	}
	
	pub fn shutdown(&mut self) {
		self.output_agent.shutdown_and_join();
	}
	
	pub fn read_incoming_messages<PROVIDER : Provider<String, GError>>(&mut self, mut input: PROVIDER) 
		-> GResult<()> 
	{
		loop {
			let message = try!(input.obtain_next());
			self.handle_message(&message);
		}
	}
	
	pub fn handle_message(&mut self, message: &str) {
		match parse_jsonrpc_request(message) {
			Ok(rpc_request) => { 
				self.dispatch_request(rpc_request);
			} 
			Err(error) => {
				// If we can't parse JsonRpcRequest, send a sesponse with null id
				let id = None;
				post_response(&mut self.output_agent, JsonRpcResponse::new_error(id, error)); 
			}
		}
	}
	
	pub fn dispatch_request(&mut self, request: JsonRpcRequest) {
		let id = request.id;
		
		if let Some(result_or_error) = self.do_dispatch_request(&request.method, request.params) 
		{
			let response = JsonRpcResponse{ id: id, result_or_error : result_or_error };
			post_response(&mut self.output_agent, response);
		}
	}
	
	pub fn do_dispatch_request(&mut self, request_method: &String, request_params: Map<String, Value>) 
		-> Option<JsonRpcResult_Or_Error> 
	{
		if let Some(dispatcher_fn) = self.dispatcher_map.get(request_method) 
		{
			// FIXME: asynchronous operation 
			return dispatcher_fn(request_params);
		} else {
			return Some(JsonRpcResult_Or_Error::Error(error_JSON_RPC_MethodNotFound()));
		}
	}
	
	pub fn add_notification<
		PARAMS : serde::Deserialize + 'static,
	>(
		&mut self,
		method_name: &'static str, 
		method_fn: Box<Fn(PARAMS)>
	) {
		self.add_rpc_handler(method_name, RpcNotification { method_fn : method_fn });
	}
	
	pub fn add_request<
		PARAMS : serde::Deserialize + 'static, 
		RET : serde::Serialize + 'static, 
		RET_ERROR : serde::Serialize + 'static
	>(
		&mut self,
		method_name: &'static str, 
		method_fn: Box<ServiceHandler<PARAMS, RET, RET_ERROR>>
	) {
		self.add_rpc_handler(method_name, RpcRequest { method_fn : method_fn });
	}
	
	pub fn add_rpc_handler<REQUEST_HANDLER>(
		&mut self,
		method_name: &'static str,
		request_method: REQUEST_HANDLER
	)
		where REQUEST_HANDLER: HandleRpcRequest + 'static,
	{
		let handler_fn : Box<DispatcherFn> = Box::new(move |params_map| {
			request_method.handle_jsonrpc_request(params_map)
		});
		
		self.dispatcher_map.insert(method_name.to_string(), handler_fn);
	}
}

impl Drop for JsonRpcEndpoint {
	
	fn drop(&mut self) {
		assert!(self.is_shutdown());
		// We shutdown ourselves, but I don't that a good style to do in drop,
		// since shutdown should join with thread
	}
	
}

/* -----------------  ----------------- */

pub trait HandleRpcRequest {
	
	fn handle_jsonrpc_request(&self, params_map: Map<String, Value>) -> Option<JsonRpcResult_Or_Error>;
	
}

pub struct RpcRequest<
	PARAMS : serde::Deserialize + 'static, 
	RET: serde::Serialize + 'static, 
	RET_ERROR : serde::Serialize + 'static
> {
	pub method_fn: Box<ServiceHandler<PARAMS, RET, RET_ERROR>>
}

impl<
	PARAMS : serde::Deserialize + 'static, 
	RET : serde::Serialize + 'static, 
	RET_ERROR : serde::Serialize + 'static
> HandleRpcRequest for RpcRequest<PARAMS, RET, RET_ERROR> {
	
	fn handle_jsonrpc_request(&self, params_map: Map<String, Value>) -> Option<JsonRpcResult_Or_Error> {
		handle_request(params_map, self.method_fn.as_ref())
	}
	
}


pub struct RpcNotification<
	PARAMS : serde::Deserialize + 'static, 
> {
	pub method_fn: Box<Fn(PARAMS)>
}
impl<
	PARAMS : serde::Deserialize + 'static, 
> HandleRpcRequest for RpcNotification<PARAMS> {
	
	fn handle_jsonrpc_request(&self, params_map: Map<String, Value>) -> Option<JsonRpcResult_Or_Error> {
		let params_res : Result<PARAMS, _> = serde_json::from_value(Value::Object(params_map));
		match params_res {
			Ok(params) => { 
				(self.method_fn)(params);
				None
			} 
			Err(error) => {
				return Some(JsonRpcResult_Or_Error::Error(error_JSON_RPC_InvalidParams()));
			}
		}
	}
	
}

	pub fn handle_request<PARAMS, RET, RET_ERROR>(
		params_map: Map<String, Value>,
		method_fn: &ServiceHandler<PARAMS, RET, RET_ERROR>
	) -> Option<JsonRpcResult_Or_Error>
		where 
		PARAMS : serde::Deserialize, 
		RET : serde::Serialize, 
		RET_ERROR : serde::Serialize
	{
		let params_result : Result<PARAMS, _> = serde_json::from_value(Value::Object(params_map));
		
		let params = 
		if let Ok(params) = params_result {
			params
		} else {
			return Some(JsonRpcResult_Or_Error::Error(error_JSON_RPC_InvalidParams()));
		};
		
		let result = method_fn(params);
		
		match result {
			Ok(ret) => {
				let ret = serde_json::to_value(&ret);
				return Some(JsonRpcResult_Or_Error::Result(ret)); 
			} 
			Err(error) => {
				let error : ServiceError<RET_ERROR> = error; // FIXME cleanup syntax
				let json_rpc_error = JsonRpcError { 
					code : error.code as i64, // FIXME review truncation
					message : error.message,
					data : Some(serde_json::to_value(&error.data)),
				};
				
				return Some(JsonRpcResult_Or_Error::Error(json_rpc_error));
			}
		}
	}

/* ----------------- Test ----------------- */

#[test]
fn parse_jsonrpc_request_json_Test() {
	
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
		.build();
	
	let result = parse_jsonrpc_request_json(&mut request).unwrap();
	assert_eq!(result, JsonRpcRequest { 
			id : Some(RpcId::Number(1)), 
			method : "myMethod".to_string(), 
			params : unwrap_object(ObjectBuilder::new())
	});
	
}


#[cfg(test)]
mod tests_sample_types;

#[test]
fn test_JsonRpcEndpoint() {
	
	use std::collections::BTreeMap;
	use util::tests::*;
	use tests_sample_types::*;
	
	pub fn sample_fn(params: Point) -> Result<String, ServiceError<()>> {
		let x_str : String = params.x.to_string();
		let y_str : String = params.y.to_string();
		Ok(x_str + &y_str)
	}
	pub fn new_sample_params(x: i32, y: i32) -> Point {
		Point { x : x, y : y }
	}
	
	/* -----------------  ----------------- */
	
	{
		let mut output : Vec<u8> = vec![];
		let mut rpc = JsonRpcEndpoint::new(Box::new(output));
		
		let request = JsonRpcRequest::new(1, "my_method".to_string(), BTreeMap::new());
		let result = rpc.do_dispatch_request(&request.method, request.params);
		
//		let expected = JsonRpcResponse::new_error(None, error_JSON_RPC_MethodNotFound());
		assert_equal(&result, &Some(JsonRpcResult_Or_Error::Error(error_JSON_RPC_MethodNotFound())));
		rpc.shutdown();
	}
	
	{
		let mut output : Vec<u8> = vec![];
		let mut rpc = JsonRpcEndpoint::new(Box::new(output));
		let handler = Box::new(sample_fn);
		rpc.add_request("my_method", handler);
		
		let request = JsonRpcRequest::new(1, "my_method".to_string(), BTreeMap::new());
		let result = rpc.do_dispatch_request(&request.method, request.params);
		assert_equal(result, Some(JsonRpcResult_Or_Error::Error(error_JSON_RPC_InvalidParams())));
		
		// FIXME: review
//		assert_equal(String::new(), String::from_utf8(*output_).unwrap());
		
		let params_value = match serde_json::to_value(&new_sample_params(10, 20)) {
			Value::Object(object) => object, 
			_ => panic!("Not serialized into Object") 
		};
		
		let request = JsonRpcRequest::new(1, "my_method".to_string(), params_value);
		let result = rpc.do_dispatch_request(&request.method, request.params);
		assert_equal(result, Some(JsonRpcResult_Or_Error::Result(
					Value::String("1020".to_string()))));
		
		rpc.shutdown();
	}
}