// Copyright 2016 Bruno Medeiros
//
// Licensed under the Apache License, Version 2.0 
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>. 
// This file may not be copied, modified, or distributed
// except according to those terms.

use util::core::*;

use std::collections::HashMap;

use super::ResponseCompletable;
use super::RequestHandler;
use super::serde;

use method_types::*;
use jsonrpc_types::*;


/* -----------------  MapRequestHandler  ----------------- */

pub type RpcMethodHandler = Fn(RequestParams, ResponseCompletable);

pub struct MapRequestHandler {
	pub method_handlers : HashMap<String, Box<RpcMethodHandler>>,
}

impl MapRequestHandler {
	
	pub fn new() -> MapRequestHandler {
		 MapRequestHandler { method_handlers : HashMap::new() }
	}
	
	pub fn add_notification<
		PARAMS : serde::Deserialize + 'static,
	>(
		&mut self,
		method_name: &'static str, 
		method_fn: Box<Fn(PARAMS)>
	) {
		let req_handler : Box<RpcMethodHandler> = new(move |params, completable| {
			completable.sync_handle_notification(params, &*method_fn);
		});
		self.add_rpc_handler(method_name, req_handler);
	}
	
	pub fn add_request<
		PARAMS : serde::Deserialize + 'static, 
		RET : serde::Serialize + 'static, 
		RET_ERROR : serde::Serialize + 'static
	>(
		&mut self,
		method_name: &'static str, 
		method_fn: Box<Fn(PARAMS) -> MethodResult<RET, RET_ERROR>>
	) {
		let req_handler : Box<RpcMethodHandler> = new(move |params, completable| {
			completable.sync_handle_request(params, &*method_fn);
		});
		self.add_rpc_handler(method_name, req_handler);
	}
	
	pub fn add_rpc_handler(
		&mut self,
		method_name: &'static str,
		method_handler: Box<RpcMethodHandler>
	) {
		self.method_handlers.insert(method_name.to_string(), method_handler);
	}
	
	fn do_invoke_method(
		&mut self, 
		method_name: &str, 
		completable: ResponseCompletable,
		request_params: RequestParams,
	) {
		if let Some(method_fn) = self.method_handlers.get(method_name) 
		{
			let method_fn : &Box<RpcMethodHandler> = method_fn;
			method_fn(request_params, completable);
		} else {
			completable.complete_with_error(error_JSON_RPC_MethodNotFound());
		};
	}
	
}

impl RequestHandler for MapRequestHandler {
	
	fn handle_request(&mut self, request_method: &str, request_params: RequestParams, 
		completable: ResponseCompletable) 
	{
		self.do_invoke_method(request_method, completable, request_params);
	}
	
}
