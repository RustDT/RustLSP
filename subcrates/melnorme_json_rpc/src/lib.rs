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
pub mod jsonrpc_objects;
pub mod service_util;
pub mod output_agent;

/* -----------------  ----------------- */

use util::core::*;

use serde_json::Value;

use std::io;
use std::io::Write;
use std::collections::HashMap;
use std::result::Result;

use std::sync::Arc;
use std::sync::Mutex;

use service_util::ServiceError;
use service_util::ServiceHandler;
use service_util::Provider;
use service_util::Handler;

use json_util::*;
use jsonrpc_objects::*;

/* -----------------  JSON-RPC custom deserialization  ----------------- */

struct JsonRequestDeserializerHelper;

impl JsonDeserializerHelper<JsonRpcError> for JsonRequestDeserializerHelper {
	
	fn new_request_deserialization_error(&self) -> JsonRpcError {
		return error_JSON_RPC_InvalidRequest();
	}
	
}

pub type JsonRpcResult<T> = Result<T, JsonRpcError>;

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

/* -----------------  JsonRpcEndpoint  ----------------- */

use output_agent::OutputAgent;
use output_agent::OutputAgentTask;
use output_agent::AgentLoopRunner;

pub struct JsonRpcEndpoint {
	pub request_handler : Box<RpcRequestHandler>,
	pub output_agent : Arc<Mutex<OutputAgent>>,
}



impl JsonRpcEndpoint {
	
	pub fn start<AGENT_RUNNER>(agent_runner: AGENT_RUNNER, request_handler: Box<RpcRequestHandler>) 
		-> JsonRpcEndpoint
	where 
		AGENT_RUNNER : FnOnce(AgentLoopRunner),
		AGENT_RUNNER : Send + 'static,
	{
		let output_agent = OutputAgent::start(agent_runner);
		JsonRpcEndpoint { request_handler: request_handler, output_agent : newArcMutex(output_agent) }
	}
	
	pub fn start_with_provider<OUT, OUT_P>(out_stream_provider: OUT_P, request_handler: Box<RpcRequestHandler>) 
		-> JsonRpcEndpoint
	where 
		OUT: Handler<String, GError> + 'static, 
		OUT_P : FnOnce() -> OUT + Send + 'static 
	{
		let output_agent = OutputAgent::start_with_provider(out_stream_provider);
		JsonRpcEndpoint { request_handler: request_handler, output_agent : newArcMutex(output_agent) }
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
		self.request_handler.handle_request(&request.method, request.params, completable); 
	}
	
}

/* ----------------- Response handling ----------------- */

pub trait RpcRequestHandler {
	
	fn handle_request(&mut self, request_method: &String, request_params: JsonObject, 
		completable: JsonRpcRequestCompletable); 
}

pub struct JsonRpcRequestCompletable {
	completion_flag: FinishedFlag,
	id: Option<RpcId>,
	output_agent: Arc<Mutex<OutputAgent>>,
}

impl JsonRpcRequestCompletable {
	
	pub fn new(id: Option<RpcId>, output_agent: Arc<Mutex<OutputAgent>>) -> JsonRpcRequestCompletable {
		
		// From the spec: `A Notification is a Request object without an "id" member.`
		
		JsonRpcRequestCompletable{ completion_flag : FinishedFlag(false), id : id, output_agent: output_agent } 
	}
	
	pub fn complete(mut self, rpc_result: Option<JsonRpcResult_Or_Error>) {
		self.completion_flag.finish();
		
		if let Some(rpc_result) = rpc_result {
			
			let response =
			if let Some(id) = self.id {
				JsonRpcResponse{ id : id, result_or_error : rpc_result }
			} else {
				JsonRpcResponse::new_error(RpcId::Null, error_JSON_RPC_InvalidRequest())
			};
			
			post_response(&self.output_agent, response);
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

/* -----------------  ----------------- */

pub struct MapRpcRequestHandler {
	pub method_handlers : HashMap<String, Box<MethodHandler>>,
}

impl MapRpcRequestHandler {
	
	pub fn new() -> MapRpcRequestHandler {
		 MapRpcRequestHandler { method_handlers : HashMap::new() }
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
	
	pub fn add_handler<METHOD_HANDLER : MethodHandler + 'static>(
		&mut self,
		method: (&'static str, METHOD_HANDLER)
	) {
		self.add_rpc_handler(method.0, method.1);
	}
	
	pub fn add_rpc_handler<METHOD_HANDLER: MethodHandler + 'static>(
		&mut self,
		method_name: &'static str,
		method_handler: METHOD_HANDLER
	) {
		self.method_handlers.insert(method_name.to_string(), new(method_handler));
	}
	
	fn invoke_method(&mut self, request_method: &String, request_params: JsonObject) 
		-> Option<JsonRpcResult_Or_Error>
	{
		if let Some(dispatcher_fn) = self.method_handlers.get(request_method) 
		{
			// FIXME: asynchronous operation 
			return dispatcher_fn.handle_invocation(request_params);
		} else {
			return Some(JsonRpcResult_Or_Error::Error(error_JSON_RPC_MethodNotFound()));
		}
	}
	
}

impl RpcRequestHandler for MapRpcRequestHandler {
	
	fn handle_request(&mut self, request_method: &String, request_params: JsonObject, 
		completable: JsonRpcRequestCompletable) 
	{
		let method_result = self.invoke_method(request_method, request_params);
		completable.complete(method_result);
	}
	
}

pub trait MethodHandler {
	
	fn handle_invocation(&self, params_map: JsonObject) -> Option<JsonRpcResult_Or_Error>;
	
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
> MethodHandler for RpcRequest<PARAMS, RET, RET_ERROR> {
	
	fn handle_invocation(&self, params_map: JsonObject) -> Option<JsonRpcResult_Or_Error> {
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
> MethodHandler for RpcNotification<PARAMS> {
	
	fn handle_invocation(&self, params_map: JsonObject) -> Option<JsonRpcResult_Or_Error> {
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


/* ----------------- Tests ----------------- */

mod tests_sample_types;

#[cfg(test)]
mod _tests {
	
	use super::*;
	use util::core::*;
	use util::tests::*;
	use tests_sample_types::*;
	use ::jsonrpc_objects::tests::*;
	
	use service_util::*;
	use jsonrpc_objects::*;


	pub fn sample_fn(params: Point) -> Result<String, ServiceError<()>> {
		let x_str : String = params.x.to_string();
		let y_str : String = params.y.to_string();
		Ok(x_str + &y_str)
	}
	pub fn new_sample_params(x: i32, y: i32) -> Point {
		Point { x : x, y : y }
	}
	
	pub fn check_request(result: JsonRpcResult_Or_Error, expected: JsonRpcResult_Or_Error) {
		if let JsonRpcResult_Or_Error::Error(error) = result {
			
			if let JsonRpcResult_Or_Error::Error(expected_error) = expected {
				check_error(error.clone(), expected_error.clone());
			}
			
		} else {
			assert_equal(&result, &expected);
		}
	}


	#[test]
	fn test_JsonRpcEndpoint() {
		
		use std::collections::BTreeMap;
		use output_agent::IoWriteHandler;
		use serde_json::Value;
		use serde_json;
		
		{
			// Test handle unknown method
			let mut method_handler = new(MapRpcRequestHandler::new());
			
			let request = JsonRpcRequest::new(1, "my_method".to_string(), BTreeMap::new());
			let result = method_handler.invoke_method(&request.method, request.params).unwrap();
			
			check_request(result, JsonRpcResult_Or_Error::Error(error_JSON_RPC_MethodNotFound()));
		}
		
		let output = vec![];
		let mut method_handler = new(MapRpcRequestHandler::new());
		method_handler.add_request("my_method", Box::new(sample_fn));
		
		let request = JsonRpcRequest::new(1, "my_method".to_string(), BTreeMap::new());
		let result = method_handler.invoke_method(&request.method, request.params).unwrap();
		check_request(result, JsonRpcResult_Or_Error::Error(error_JSON_RPC_InvalidParams("missing field")));
		
		let params_value = match serde_json::to_value(&new_sample_params(10, 20)) {
			Value::Object(object) => object, 
			_ => panic!("Not serialized into Object") 
		};
		
		let request = JsonRpcRequest::new(1, "my_method".to_string(), params_value);
		let result = method_handler.invoke_method(&request.method, request.params.clone()).unwrap();
		check_request(result, JsonRpcResult_Or_Error::Result(
			Value::String("1020".to_string()))
		);
		
		
		let mut rpc = JsonRpcEndpoint::start_with_provider(move || IoWriteHandler(output), method_handler);
		
		// Test JsonRpcRequestCompletable - missing id for notification method
		let completable = JsonRpcRequestCompletable::new(None, rpc.output_agent.clone());
		completable.complete(None);
		
		// Test JsonRpcRequestCompletable - missing id for regular method
		let completable = JsonRpcRequestCompletable::new(None, rpc.output_agent.clone());
		completable.complete(Some(JsonRpcResult_Or_Error::Result(Value::String("1020".to_string()))));
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