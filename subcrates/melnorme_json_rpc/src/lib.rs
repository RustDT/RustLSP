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
use service_util::ServiceResult;
use service_util::Provider;
use service_util::Handler;

use json_util::*;
use jsonrpc_objects::*;


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
	
	pub fn run_message_read_loop<PROVIDER : Provider<String, GError>>(&mut self, mut input: PROVIDER) 
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
		let completable = JsonRpcResponseCompletable::new(request.id, self.output_agent.clone());
		self.request_handler.handle_request(&request.method, request.params, completable); 
	}
	
}

/* ----------------- Response handling ----------------- */

pub trait RpcRequestHandler {
	
	fn handle_request(&mut self, request_method: &String, request_params: JsonObject, 
		completable: JsonRpcResponseCompletable); 
}

pub struct JsonRpcResponseCompletable {
	completion_flag: FinishedFlag,
	id: Option<RpcId>,
	on_response: Box<FnMut(JsonRpcResponse)>,
}

impl JsonRpcResponseCompletable {
	
	pub fn new(id: Option<RpcId>, output_agent: Arc<Mutex<OutputAgent>>) -> JsonRpcResponseCompletable {
		
		let on_response : Box<FnMut(JsonRpcResponse)> = new(move |response| { 
			post_response(&output_agent, response); 
		});
		
		JsonRpcResponseCompletable { 
			completion_flag : FinishedFlag(false), id : id, on_response: on_response 
		}
	}
	
	pub fn complete(mut self, rpc_result: Option<JsonRpcResult_Or_Error>) {
		self.completion_flag.finish();
		
		// From the spec: `A Notification is a Request object without an "id" member.`
		if let Some(rpc_result) = rpc_result {
			
			let response =
			if let Some(id) = self.id {
				JsonRpcResponse{ id : id, result_or_error : rpc_result }
			} else {
				JsonRpcResponse::new_error(RpcId::Null, 
					error_JSON_RPC_InvalidRequest("Property `id` not provided for request."))
			};
			
			(self.on_response)(response);
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
	pub method_handlers : HashMap<String, Box<RpcMethodHandler>>,
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
		let handler = new(move |p| { method_fn(p); None });
		let request : FnRpcMethodHandler<PARAMS, (), ()> = FnRpcMethodHandler { method_fn : handler };
		self.add_rpc_handler(method_name, request);
	}
	
	pub fn add_request<
		PARAMS : serde::Deserialize + 'static, 
		RET : serde::Serialize + 'static, 
		RET_ERROR : serde::Serialize + 'static
	>(
		&mut self,
		method_name: &'static str, 
		method_fn: Box<Fn(PARAMS) -> ServiceResult<RET, RET_ERROR>>
	) {
		let handler = new(move |p| { Some(method_fn(p)) });
		self.add_rpc_handler(method_name, FnRpcMethodHandler { method_fn : handler });
	}
	
	pub fn add_rpc_handler<METHOD_HANDLER: RpcMethodHandler + 'static>(
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
		completable: JsonRpcResponseCompletable) 
	{
		let method_result = self.invoke_method(request_method, request_params);
		completable.complete(method_result);
	}
	
}

pub trait RpcMethodHandler {
	
	fn handle_invocation(&self, params_obj: JsonObject) -> Option<JsonRpcResult_Or_Error>;
	
}

pub struct FnRpcMethodHandler<
	PARAMS : serde::Deserialize, 
	RET: serde::Serialize,
	RET_ERROR : serde::Serialize,
> {
	pub method_fn: Box<Fn(PARAMS) -> Option<ServiceResult<RET, RET_ERROR>>>
}

impl<
	PARAMS : serde::Deserialize, 
	RET : serde::Serialize,
	RET_ERROR : serde::Serialize,
> RpcMethodHandler for FnRpcMethodHandler<PARAMS, RET, RET_ERROR> {
	
	fn handle_invocation(&self, params_obj: JsonObject) -> Option<JsonRpcResult_Or_Error> {
		handle_request(params_obj, self.method_fn.as_ref())
	}
	
}

	pub fn handle_request<PARAMS, RET, RET_ERROR>(
		params_obj: JsonObject,
		method_fn: &Fn(PARAMS) -> Option<ServiceResult<RET, RET_ERROR>>
	) -> Option<JsonRpcResult_Or_Error>
		where 
		PARAMS : serde::Deserialize, 
		RET : serde::Serialize, 
		RET_ERROR : serde::Serialize
	{
		let params_value = if !params_obj.is_empty() {
			Value::Object(params_obj)
		} else {
			Value::Null
		};
		
		let params_result : Result<PARAMS, _> = serde_json::from_value(params_value);
		
		let result = 
		match params_result {
			Ok(params) => { 
				method_fn(params) 
			} 
			Err(error) => { 
				return Some(JsonRpcResult_Or_Error::Error(error_JSON_RPC_InvalidParams(error)));
			}
		};
		
		let result = 
		if let Some(result) = result {
			result
		} else {
			return None;
		};
		
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
mod tests_ {
	
	use super::*;
	use util::core::*;
	use util::tests::*;
	use tests_sample_types::*;
	use ::jsonrpc_objects::tests::*;
	
	use service_util::*;
	use jsonrpc_objects::*;
	
	use json_util::JsonObject;
	use output_agent::IoWriteHandler;
	use serde_json::Value;
	use serde_json;
	
	pub fn sample_fn(params: Point) -> Result<String, ServiceError<()>> {
		let x_str : String = params.x.to_string();
		let y_str : String = params.y.to_string();
		Ok(x_str + &y_str)
	}
	pub fn new_sample_params(x: i32, y: i32) -> Point {
		Point { x : x, y : y }
	}
	pub fn no_params_method(_params: ()) -> Result<String, ServiceError<()>> {
		Ok("okay".into())
	}
	
	pub fn check_request(result: JsonRpcResult_Or_Error, expected: JsonRpcResult_Or_Error) {
		if let JsonRpcResult_Or_Error::Error(ref error) = result {
			
			if let JsonRpcResult_Or_Error::Error(expected_error) = expected {
				check_error(error.clone(), expected_error.clone());
				return;
			}
			
		}
		
		assert_equal(&result, &expected);
	}
	
	
	#[test]
	fn test_JsonRpcEndpoint() {
		
		{
			// Test handle unknown method
			let mut method_handler = new(MapRpcRequestHandler::new());
			
			let request = JsonRpcRequest::new(1, "my_method".to_string(), JsonObject::new());
			let result = method_handler.invoke_method(&request.method, request.params).unwrap();
			
			check_request(result, JsonRpcResult_Or_Error::Error(error_JSON_RPC_MethodNotFound()));
		}
		
		let output = vec![];
		let mut method_handler = new(MapRpcRequestHandler::new());
		method_handler.add_request("my_method", Box::new(sample_fn));
		
		// test with invalid params = "{}" 
		let request = JsonRpcRequest::new(1, "my_method".to_string(), JsonObject::new());
		let result = method_handler.invoke_method(&request.method, request.params).unwrap();
		check_request(result, JsonRpcResult_Or_Error::Error(error_JSON_RPC_InvalidParams("invalid type: unit")));
		
		// test with valid params
		let params_value = match serde_json::to_value(&new_sample_params(10, 20)) {
			Value::Object(object) => object, 
			_ => panic!("Not serialized into Object") 
		};
		let request = JsonRpcRequest::new(1, "my_method".to_string(), params_value);
		let result = method_handler.invoke_method(&request.method, request.params.clone()).unwrap();
		assert_equal(result, JsonRpcResult_Or_Error::Result(
			Value::String("1020".to_string())
		));
		
		
		// Test valid request with params = "{}"
		method_handler.add_request("no_params_method", Box::new(no_params_method));
		
		let request = JsonRpcRequest::new(1, "no_params_method".to_string(), JsonObject::new());
		let result = method_handler.invoke_method(&request.method, request.params.clone()).unwrap();
		assert_equal(result, JsonRpcResult_Or_Error::Result(
			Value::String("okay".to_string())
		));
		
		
		// --- JsonRpcEndpoint:
		
		let mut rpc = JsonRpcEndpoint::start_with_provider(move || IoWriteHandler(output), method_handler);
		
		// Test JsonRpcResponseCompletable - missing id for notification method
		let completable = JsonRpcResponseCompletable::new(None, rpc.output_agent.clone());
		completable.complete(None);
		
		// Test JsonRpcResponseCompletable - missing id for regular method
		let completable = JsonRpcResponseCompletable::new(None, rpc.output_agent.clone());
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