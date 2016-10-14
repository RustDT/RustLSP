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

use serde_json::Value;

use std::io;
use std::io::Write;
use std::collections::HashMap;
use std::result::Result;

use std::fmt;
use std::sync::Arc;
use std::sync::Mutex;

use service_util::ServiceError;
use service_util::ServiceHandler;
use service_util::Provider;
use service_util::Handler;

use json_util::*;

/* ----------------- JSON RPC ----------------- */

#[derive(Debug, PartialEq, Clone)]
pub enum RpcId { Number(u64), String(String), Null, }

#[derive(Debug, PartialEq, Clone)]
/// A JSON RPC request, version 2.0
pub struct JsonRpcRequest {
	// ommited jsonrpc field, must be "2.0"
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

impl JsonRpcError {
	
	pub fn new(code: i64, message: String) -> JsonRpcError {
		JsonRpcError { code : code, message : message, data : None }
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

/* -----------------  ----------------- */

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
			&RpcId::Null => serializer.serialize_none(),
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
	
	pub fn new(id_number : u64, method : String, params : JsonObject) -> JsonRpcRequest {
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


/* -----------------  ----------------- */


pub fn parse_jsonrpc_request(message: &str) -> JsonRpcResult<JsonRpcRequest> {
	let mut json_result : Value = 
	match serde_json::from_str(message) 
	{
		Ok(ok) => { ok } 
		Err(error) => { 
			return Err(error_JSON_RPC_ParseError(error));
		}
	};
	
	let mut json_request_map : &mut JsonObject =
	match json_result {
		Value::Object(ref mut map) => map ,
		_ => { return Err(error_JSON_RPC_InvalidRequest()) },
	};
	
	parse_jsonrpc_request_jsonObject(&mut json_request_map)
}

pub fn parse_jsonrpc_request_jsonObject(mut request_map: &mut JsonObject) -> JsonRpcResult<JsonRpcRequest> {
	
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

/* -----------------  ----------------- */

use output_agent::OutputAgent;
use output_agent::OutputAgentTask;
use output_agent::AgentLoopRunner;

pub type DispatcherFn = Fn(JsonObject) -> Option<JsonRpcResult_Or_Error>;

pub struct JsonRpcEndpoint {
	pub method_handler : Box<MethodHandler>,
	pub output_agent : Arc<Mutex<OutputAgent>>,
}


impl JsonRpcEndpoint {
	
	pub fn start<AGENT_RUNNER>(agent_runner: AGENT_RUNNER, method_handler: Box<MethodHandler>) 
		-> JsonRpcEndpoint
	where 
		AGENT_RUNNER : FnOnce(AgentLoopRunner),
		AGENT_RUNNER : Send + 'static,
	{
		let output_agent = OutputAgent::start(agent_runner);
		JsonRpcEndpoint { method_handler: method_handler, output_agent : newArcMutex(output_agent) }
	}
	
	pub fn start_with_provider<OUT, OUT_P>(out_stream_provider: OUT_P, method_handler: Box<MethodHandler>) 
		-> JsonRpcEndpoint
	where 
		OUT: Handler<String, GError> + 'static, 
		OUT_P : FnOnce() -> OUT + Send + 'static 
	{
		let output_agent = OutputAgent::start_with_provider(out_stream_provider);
		JsonRpcEndpoint { method_handler: method_handler, output_agent : newArcMutex(output_agent) }
	}
	
	pub fn is_shutdown(& self) -> bool {
		self.output_agent.lock().unwrap().is_shutdown()
	}
	
	pub fn shutdown(&mut self) {
		self.output_agent.lock().unwrap().shutdown_and_join();
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
				self.handle_request(rpc_request);
			} 
			Err(error) => {
				// If we can't parse JsonRpcRequest, send an error response with null id
				let id = RpcId::Null;
				post_response(&mut self.output_agent, JsonRpcResponse::new_error(id, error)); 
			}
		}
	}
	
	pub fn handle_request(&mut self, request: JsonRpcRequest) {
		let completable = JsonRpcRequestCompletable::new(request.id, self.output_agent.clone());
		let rpc_result = self.do_dispatch_request(&request.method, request.params); 
		completable.provide_result(rpc_result);
	}
	
	pub fn do_dispatch_request(&mut self, request_method: &String, request_params: JsonObject) 
		-> Option<JsonRpcResult_Or_Error> 
	{
		self.method_handler.handle_method(request_method, request_params)
	}
	
}

/* -----------------  ----------------- */

pub trait MethodHandler {
	
	fn handle_method(&mut self, request_method: &String, request_params: JsonObject) 
		-> Option<JsonRpcResult_Or_Error>; 
}

pub struct MapMethodHandler {
	pub method_handlers : HashMap<String, Box<DispatcherFn>>,
}

impl MapMethodHandler {
	
	pub fn new() -> MapMethodHandler {
		 MapMethodHandler { method_handlers : HashMap::new() }
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
	
	pub fn add_handler<REQUEST_HANDLER : HandleRpcRequest + 'static>(
		&mut self,
		method: (&'static str, REQUEST_HANDLER)
	) {
		self.add_rpc_handler(method.0, method.1);
	}
	
	pub fn add_rpc_handler<REQUEST_HANDLER: HandleRpcRequest + 'static>(
		&mut self,
		method_name: &'static str,
		request_method: REQUEST_HANDLER
	) {
		let handler_fn : Box<DispatcherFn> = Box::new(move |params_map| {
			request_method.handle_jsonrpc_request(params_map)
		});
		
		self.method_handlers.insert(method_name.to_string(), handler_fn);
	}
	
}

impl MethodHandler for MapMethodHandler {
	
	fn handle_method(&mut self, request_method: &String, request_params: JsonObject) 
		-> Option<JsonRpcResult_Or_Error>
	{
		if let Some(dispatcher_fn) = self.method_handlers.get(request_method) 
		{
			// FIXME: asynchronous operation 
			return dispatcher_fn(request_params);
		} else {
			return Some(JsonRpcResult_Or_Error::Error(error_JSON_RPC_MethodNotFound()));
		}
	}
	
}

/* -----------------  ----------------- */

pub trait HandleRpcRequest {
	
	fn handle_jsonrpc_request(&self, params_map: JsonObject) -> Option<JsonRpcResult_Or_Error>;
	
}

pub struct RpcRequest<
	PARAMS : serde::Deserialize, 
	RET: serde::Serialize,
	RET_ERROR : serde::Serialize,
> {
	pub method_fn: Box<ServiceHandler<PARAMS, RET, RET_ERROR>>
}

impl<
	PARAMS : serde::Deserialize, 
	RET : serde::Serialize,
	RET_ERROR : serde::Serialize,
> HandleRpcRequest for RpcRequest<PARAMS, RET, RET_ERROR> {
	
	fn handle_jsonrpc_request(&self, params_map: JsonObject) -> Option<JsonRpcResult_Or_Error> {
		handle_request(params_map, self.method_fn.as_ref())
	}
	
}


pub struct RpcNotification<
	PARAMS : serde::Deserialize, 
> {
	pub method_fn: Box<Fn(PARAMS)>
}

impl<
	PARAMS : serde::Deserialize, 
> HandleRpcRequest for RpcNotification<PARAMS> {
	
	fn handle_jsonrpc_request(&self, params_map: JsonObject) -> Option<JsonRpcResult_Or_Error> {
		let params_res : Result<PARAMS, _> = serde_json::from_value(Value::Object(params_map));
		match params_res {
			Ok(params) => { 
				(self.method_fn)(params);
				None
			} 
			Err(error) => {
				return Some(JsonRpcResult_Or_Error::Error(error_JSON_RPC_InvalidParams(error)));
			}
		}
	}
	
}


	pub fn handle_request<PARAMS, RET, RET_ERROR>(
		params_map: JsonObject,
		method_fn: &ServiceHandler<PARAMS, RET, RET_ERROR>
	) -> Option<JsonRpcResult_Or_Error>
		where 
		PARAMS : serde::Deserialize, 
		RET : serde::Serialize, 
		RET_ERROR : serde::Serialize
	{
		let params_result : Result<PARAMS, _> = serde_json::from_value(Value::Object(params_map));
		
		let params = 
		match params_result {
			Ok(params) => { 
				params 
			} 
			Err(error) => { 
				return Some(JsonRpcResult_Or_Error::Error(error_JSON_RPC_InvalidParams(error)));
			}
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

/* ----------------- Response ----------------- */

pub struct JsonRpcRequestCompletable {
	completion_flag: FinishedFlag,
	id: Option<RpcId>,
	output_agent: Arc<Mutex<OutputAgent>>,
}

impl JsonRpcRequestCompletable {
	
	pub fn new(id: Option<RpcId>, output_agent: Arc<Mutex<OutputAgent>>) -> JsonRpcRequestCompletable {
		
		// From the spec: `A Notification is a Request object without an "id" member.`
		
		let completable = id.is_some(); 
		
		JsonRpcRequestCompletable{ completion_flag : FinishedFlag(completable), id : id, output_agent: output_agent } 
	}
	
	pub fn provide_result(mut self, rpc_result: Option<JsonRpcResult_Or_Error>) {
		if let Some(rpc_result) = rpc_result {
			self.completion_flag.finish();
			
			let response =
			if let Some(id) = self.id {
				JsonRpcResponse{ id : id, result_or_error : rpc_result }
			} else {
				JsonRpcResponse::new_error(RpcId::Null, error_JSON_RPC_InvalidRequest())
			};
			
			post_response(&self.output_agent, response);
		} else {
			self.completion_flag.set_finished();
		}
	}
	
}

pub fn post_response(output_agent: &Arc<Mutex<OutputAgent>>, response: JsonRpcResponse) {
	
	let task : OutputAgentTask = Box::new(move |mut response_handler| {
		/* FIXME: log , review expect */ 
		writeln!(&mut io::stderr(), "Response: {:?}", response).expect("Failed writing to stderr");
		
		let response_str = serde_json::to_string(&response).unwrap_or_else(|error| -> String { 
			panic!("Failed to serialize to JSON, should be impossible: {}", error);
		});
		
		let write_res = response_handler.supply(&response_str);
		if let Err(error) = write_res {
			// TODO log
			// FIXME handle output stream write error by shutting down
			writeln!(&mut io::stderr(), "Error writing RPC response: {}", error)
				.expect("Failed writing to stderr");
		};
	});
	
	let res = {
		output_agent.lock().unwrap().try_submit_task(task)
	}; 
	// If res is error, panic here, outside of thread lock
	res.expect("Output agent is shutdown or thread panicked!");
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

/* ----------------- Test ----------------- */

mod tests_sample_types;

#[cfg(test)]
mod tests {
	use super::*;
	use util::core::*;
	use util::tests::*;
	use tests_sample_types::*;
	
	use serde;
	use serde_json;
	use serde_json::Value;
	use serde_json::builder::ObjectBuilder;
	use json_util::*;
	use service_util::*;


	pub fn sample_fn(params: Point) -> Result<String, ServiceError<()>> {
		let x_str : String = params.x.to_string();
		let y_str : String = params.y.to_string();
		Ok(x_str + &y_str)
	}
	pub fn new_sample_params(x: i32, y: i32) -> Point {
		Point { x : x, y : y }
	}
	
	fn to_json<T: serde::Serialize>(value: &T) -> String {
		serde_json::to_string(value).unwrap()
	}
	
	fn check_error(result: JsonRpcError, expected: JsonRpcError) {
		assert_starts_with(&result.message, &expected.message);
		assert_eq!(result, JsonRpcError { message : result.message.clone(), .. expected }); 
	}
	
	fn check_request(result: JsonRpcResult_Or_Error, expected: JsonRpcResult_Or_Error) {
		if let JsonRpcResult_Or_Error::Error(error) = result {
			
			if let JsonRpcResult_Or_Error::Error(expected_error) = expected {
				check_error(error.clone(), expected_error.clone());
			}
			
		} else {
			assert_equal(&result, &expected);
		}
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


	#[test]
	fn test_JsonRpcEndpoint() {
		
		use std::collections::BTreeMap;
		use output_agent::IoWriteHandler;
		use serde_json::Value;
		use serde_json;
		
		{
			let output = vec![];
			let mut rpc = JsonRpcEndpoint::start_with_provider(move || IoWriteHandler(output), new(MapMethodHandler::new()));
			
			let request = JsonRpcRequest::new(1, "my_method".to_string(), BTreeMap::new());
			let result = rpc.do_dispatch_request(&request.method, request.params).unwrap();
			
			check_request(result, JsonRpcResult_Or_Error::Error(error_JSON_RPC_MethodNotFound()));
			rpc.shutdown();
		}
	
		let output = vec![];
		let mut method_handler = new(MapMethodHandler::new());
		method_handler.add_request("my_method", Box::new(sample_fn));
		
		let mut rpc = JsonRpcEndpoint::start_with_provider(move || IoWriteHandler(output), method_handler);
		
		let request = JsonRpcRequest::new(1, "my_method".to_string(), BTreeMap::new());
		let result = rpc.do_dispatch_request(&request.method, request.params).unwrap();
		check_request(result, JsonRpcResult_Or_Error::Error(error_JSON_RPC_InvalidParams("missing field")));
		
		let params_value = match serde_json::to_value(&new_sample_params(10, 20)) {
			Value::Object(object) => object, 
			_ => panic!("Not serialized into Object") 
		};
		
		let request = JsonRpcRequest::new(1, "my_method".to_string(), params_value);
		let result = rpc.do_dispatch_request(&request.method, request.params.clone()).unwrap();
		check_request(result, JsonRpcResult_Or_Error::Result(
			Value::String("1020".to_string()))
		);
		
		// Test JsonRpcRequestCompletable - missing id for notification method
		let completable = JsonRpcRequestCompletable::new(None, rpc.output_agent.clone());
		completable.provide_result(None);
		
		// Test JsonRpcRequestCompletable - missing id for regular method
		let completable = JsonRpcRequestCompletable::new(None, rpc.output_agent.clone());
		completable.provide_result(Some(JsonRpcResult_Or_Error::Result(Value::String("1020".to_string()))));
		// test again using handle_request
		// TODO review this code
		let request = JsonRpcRequest { 	
			id : None,
			method : "my_method".into(),
			params : request.params.clone(),
		}; 
		rpc.handle_request(request);
		
		rpc.shutdown();
	}

}