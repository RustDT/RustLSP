// Copyright 2016 Bruno Medeiros
//
// Licensed under the Apache License, Version 2.0 
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>. 
// This file may not be copied, modified, or distributed
// except according to those terms.


#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

#[macro_use] extern crate log;
extern crate serde_json;
extern crate serde;

extern crate melnorme_util as util;
pub extern crate futures;

pub mod json_util;
pub mod jsonrpc_common;
pub mod jsonrpc_message;
pub mod jsonrpc_request;
pub mod jsonrpc_response;
pub mod method_types;
pub mod service_util;
pub mod output_agent;

/* -----------------  ----------------- */

use util::core::*;

use std::collections::HashMap;
use std::result::Result;

use std::sync::Arc;
use std::sync::Mutex;
 
use futures::Future;
use futures::BoxFuture;
use futures::Complete;

use service_util::MessageReader;
use jsonrpc_common::*;
use jsonrpc_message::*;
use jsonrpc_request::*;
use jsonrpc_response::*;
use method_types::*;

/* -----------------  Endpoint  ----------------- */

use output_agent::OutputAgent;
use output_agent::OutputAgentTask;


/// A JSON-RPC Server-role than can send responses to requests.
/// TODO: Client role (send requests as well)
/// TODO: review and clarify shutdown semantics
#[derive(Clone)]
pub struct EndpointOutput {
    id_counter : Arc<Mutex<u64>>,
    pending_requests : Arc<Mutex<HashMap<Id, Complete<ResponseResult>>>>,
    output_agent : Arc<Mutex<OutputAgent>>,
}

impl EndpointOutput {
    
    pub fn start_with(output_agent: OutputAgent) 
        -> EndpointOutput
    {
        EndpointOutput {
            id_counter : newArcMutex(0),
            pending_requests : newArcMutex(HashMap::new()),
            output_agent : newArcMutex(output_agent) 
        }
    }
    
    pub fn is_shutdown(& self) -> bool {
        self.output_agent.lock().unwrap().is_shutdown()
    }
    
    pub fn shutdown(&self) {
        self.output_agent.lock().unwrap().shutdown_and_join();
    }
    
    pub fn next_id(&self) -> Id {
           let id_num : &mut u64 = &mut *self.id_counter.lock().unwrap();
        *id_num += 1;
        Id::Number(*id_num)
    }
}

/// Combine an EndpointOutput with a request handler, 
/// to provide a full Endpoint capable of handling incoming requests from a message reader.
///
/// See also: EndpointOutput
pub struct EndpointHandler {
    pub output : EndpointOutput,
    pub request_handler : Box<RequestHandler>,
}

impl EndpointHandler {
    
    pub fn create_with_output_agent(output_agent: OutputAgent, request_handler: Box<RequestHandler>) 
        -> EndpointHandler
    {
        let output = EndpointOutput::start_with(output_agent);
        Self::create(output, request_handler)
    }
    
    pub fn create(output: EndpointOutput, request_handler: Box<RequestHandler>) 
        -> EndpointHandler
    {
        EndpointHandler { output : output, request_handler: request_handler }
    }
    
    /// Run a message read loop with given message reader.
    /// Loop will be terminated only when there is an error reading a message.
    ///
    /// TODO: also provide a way for message handling to terminate the loop? 
    pub fn run_message_read_loop<MSG_READER : ?Sized>(self, input: &mut MSG_READER) 
        -> GResult<()>
    where
        MSG_READER : MessageReader
    {
        let mut endpoint = self;
        loop {
            let message = match input.read_next() {
                Ok(ok) => { ok } 
                Err(error) => { 
                    endpoint.output.shutdown();
                    return Err(error);
                }
            };
            
            endpoint.handle_incoming_message(&message);
        }
    }
    
    /// Handle an incoming message
    pub fn handle_incoming_message(&mut self, message_json: &str) {
        
        let message = serde_json::from_str::<Message>(message_json);
         
        match message {
            Ok(message) => {
                match message {
                	Message::Request(request) => self.handle_incoming_request(request),  
                	Message::Response(response) => self.output.handle_incoming_response(response),
                }
            } 
            Err(error) => {
                let error = error_JSON_RPC_InvalidRequest(error);
                submit_error_write_task(&self.output.output_agent, error); 
            }
        }
    }

    /// Handle a well-formed incoming JsonRpc request object
    pub fn handle_incoming_request(&mut self, request: Request) {
        let output_agent = self.output.output_agent.clone();
        
        let on_response = new(move |response: Option<Response>| {
            if let Some(response) = response {
                submit_message_write_task(&output_agent, response.into()); 
            } else {
                let method_name = ""; // TODO
                info!("JSON-RPC notification complete. {:?}", method_name);
            } 
        });
        let completable = ResponseCompletable::new(request.id, on_response);
        
        self.request_handler.handle_request(&request.method, request.params, completable); 
    }

}

/* ----------------- Response handling ----------------- */

pub trait RequestHandler {
    fn handle_request(
        &mut self, request_method: &str, request_params: RequestParams, completable: ResponseCompletable
    );
}

/// A completable for a JSON-RPC request. This is an object that must be "completed", 
/// that is, a result must be provided. (this is the inverse of a future)
/// 
/// Must be completed once and only once, otherwise a panic is generated upon drop.
/// 
/// On completion, the on_response callback is invoked. 
/// Typically: this will write an appropriate JSON-RPC response to the endpoint output.
pub struct ResponseCompletable {
    completion_flag: FinishedFlag,
    id: Option<Id>,
    on_response: Box<FnMut(Option<Response>) + Send>,
}

impl ResponseCompletable {
    
    pub fn new(id: Option<Id>, on_response: Box<FnMut(Option<Response>) + Send>) -> ResponseCompletable {
        ResponseCompletable { 
            completion_flag : FinishedFlag(false), id : id, on_response: on_response
        }
    }
    
    pub fn complete(mut self, response_result: Option<ResponseResult>) {
        self.completion_flag.finish();
        
        // From the spec: `A Notification is a Request object without an "id" member.`
        if let Some(response_result) = response_result {
            
            let response =
            if let Some(id) = self.id {
                Response{ id : id, result_or_error : response_result }
            } else {
                Response::new_error(Id::Null, 
                    error_JSON_RPC_InvalidRequest("Property `id` not provided for request."))
            };
            
            (self.on_response)(Some(response));
        } else {
            (self.on_response)(None)
        }
    }
    
    pub fn complete_with_error(self, error: RequestError) {
        self.complete(Some(ResponseResult::Error(error)));
    }
    
    pub fn handle_request_with<PARAMS, RET, RET_ERROR, METHOD>(
        self, params: RequestParams, method_handler: METHOD
    ) 
    where 
        PARAMS : serde::Deserialize, 
        RET : serde::Serialize, 
        RET_ERROR : serde::Serialize,
        METHOD : FnOnce(PARAMS, MethodCompletable<RET, RET_ERROR>),
    {
        let mc = MethodCompletable::<RET, RET_ERROR>::new(self);
        mc.parse_params_and_complete_with(params, method_handler);
    }
    
    pub fn sync_handle_request<PARAMS, RET, RET_ERROR, METHOD>(
        self, params: RequestParams, sync_method_handler: METHOD
    ) 
    where 
        PARAMS : serde::Deserialize, 
        RET : serde::Serialize, 
        RET_ERROR : serde::Serialize ,
        METHOD : FnOnce(PARAMS) -> MethodResult<RET, RET_ERROR>,
    {
        self.handle_request_with(params, |params, completable| {
            let result = sync_method_handler(params);
            completable.complete(result);
        })
    }
    
    pub fn handle_notification_with<PARAMS, METHOD>(
        self, params: RequestParams, method_handler: METHOD
    ) 
    where 
        PARAMS : serde::Deserialize, 
        METHOD : FnOnce(PARAMS),
    {
        let mc = MethodCompletable::<(), ()>::new(self);
        mc.parse_params_and_complete_with(params, |params, completable| {
            // early completion for notification
            completable.completable.complete(None);
            method_handler(params)
        });
    }
    
    pub fn sync_handle_notification<PARAMS, METHOD>(
        self, params: RequestParams, sync_method_handler: METHOD
    ) 
    where 
        PARAMS : serde::Deserialize, 
        METHOD : FnOnce(PARAMS),
    {
        self.handle_notification_with(params, |params| {
            sync_method_handler(params);
        })
    }
    
}

use std::marker::PhantomData;

/// Helper type that wraps a ResponseCompletable, 
/// and binds the possible completion to a result `MethodResult<RET, RET_ERROR>` 
pub struct MethodCompletable
<
    RET : serde::Serialize, 
    RET_ERROR : serde::Serialize,
>
{
    completable: ResponseCompletable,
    p1: PhantomData<RET>,
    p2: PhantomData<RET_ERROR>,
}

impl<
    RET : serde::Serialize, 
    RET_ERROR : serde::Serialize,
> 
    MethodCompletable<RET, RET_ERROR>
{
    pub fn new(completable: ResponseCompletable) -> MethodCompletable<RET, RET_ERROR> {
        MethodCompletable { completable : completable, p1 : PhantomData, p2 : PhantomData}
    }
    
    pub fn parse_params_and_complete_with<PARAMS, METHOD>(
        self,
        params: RequestParams,
        method_fn: METHOD
    )
    where 
        PARAMS : serde::Deserialize, 
        RET : serde::Serialize, 
        RET_ERROR : serde::Serialize,
        METHOD : FnOnce(PARAMS, Self),
    {
        let params_value = params.into_value();
        
        let params_result : Result<PARAMS, _> = serde_json::from_value(params_value);
        
        match params_result {
            Ok(params) => { 
                method_fn(params, self);
            }
            Err(error) => {
                self.completable.complete_with_error(error_JSON_RPC_InvalidParams(error));
            }
        }
    }
    
    pub fn complete(self, result: MethodResult<RET, RET_ERROR>) {
        self.completable.complete(Some(ResponseResult::from(result)));
    }
}

pub fn submit_message_write_task(output_agent: &Arc<Mutex<OutputAgent>>, jsonrpc_message: Message) {
    
    let write_task : OutputAgentTask = Box::new(move |mut response_handler| {
        info!("JSON-RPC message: {:?}", jsonrpc_message);
        
        let response_str = serde_json::to_string(&jsonrpc_message).unwrap_or_else(|error| -> String { 
            panic!("Failed to serialize to JSON object: {}", error);
        });
        
        let write_res = response_handler.write_message(&response_str);
        if let Err(error) = write_res {
            // FIXME handle output stream write error by shutting down
            error!("Error writing JSON-RPC message: {}", error);
        };
    });
    
    let res = {
        output_agent.lock().unwrap().try_submit_task(write_task)
    }; 
    // If res is error, panic here, outside of thread lock
    res.expect("Output agent is shutdown or thread panicked!");
}

pub fn submit_error_write_task(output_agent: &Arc<Mutex<OutputAgent>>, error: RequestError) {
    let id = Id::Null;
    let response = Response::new_error(id, error);
    submit_message_write_task(output_agent, response.into()); 
}

/* -----------------  Request sending  ----------------- */

pub type RequestFuture<RET, RET_ERROR> = BoxFuture<RequestResult<RET, RET_ERROR>, futures::Canceled>;


impl EndpointOutput {
    
    /// Send a (non-notification) request
    pub fn send_request<
        PARAMS : serde::Serialize, 
        RET: serde::Deserialize, 
        RET_ERROR : serde::Deserialize, 
    >(&mut self, method_name: &str, params: PARAMS) 
        -> GResult<RequestFuture<RET, RET_ERROR>> 
    {
        let (completable, future) = futures::oneshot::<ResponseResult>();
        let future : futures::Oneshot<ResponseResult> = future;
        
        let id = self.next_id();
        
        self.pending_requests.lock().unwrap().insert(id.clone(), completable);
        
        self.write_request(Some(id), method_name, params)?;
        
        let future = future.map(|response_result : ResponseResult| {
            RequestResult::<RET, RET_ERROR>::from(response_result)
        });
        
        Ok(new(future))
    }
    
    
    /// Send a notification
    pub fn send_notification<
        PARAMS : serde::Serialize, 
    >(&self, method_name: &str, params: PARAMS) 
        -> GResult<()> 
    {
        let id = None;
        self.write_request::<_>(id, method_name, params)
    }
    
    pub fn write_request<
        PARAMS : serde::Serialize, 
    >(&self, id: Option<Id>, method_name: &str, params: PARAMS) 
        -> GResult<()> 
    {
        let params_value = serde_json::to_value(&params);
        let params = jsonrpc_request::to_jsonrpc_params(params_value)?;
        
        let rpc_request = Request { id: id.clone(), method : method_name.into(), params : params };
        
        submit_message_write_task(&self.output_agent, Message::Request(rpc_request));
        Ok(())
    }
    
    
    /// Handle a well-formed incoming JsonRpc request object
    pub fn handle_incoming_response(&mut self, response: Response) {
        let id = response.id;
        let result_or_error = response.result_or_error;
        
        let entry = self.pending_requests.lock().unwrap().remove(&id);
        
        match entry {
        	Some(entry) => { 
        	    entry.complete(result_or_error) 
        	} 
        	None => { 
                let id = Id::Null;
                let error = error_JSON_RPC_InvalidResponse(format!("id `{}` not found", id));
                submit_error_write_task(&self.output_agent, error); 
        	}
        }
    }
    
}

pub mod map_request_handler;


/* ----------------- Tests ----------------- */

mod tests_sample_types;

#[cfg(test)]
mod tests_ {
    
    use super::*;
    use util::core::*;
    use util::tests::*;
    use tests_sample_types::*;
    use map_request_handler::MapRequestHandler;
    
    use std::thread;
    
    use serde_json::Value;
    use serde_json;
    
    use jsonrpc_common::*;
    use jsonrpc_response::*;
    use jsonrpc_request::*;
    use jsonrpc_request::request_tests::check_error;
    use method_types::*;
    
    use json_util::JsonObject;
    use json_util::test_util::to_json;
    use output_agent::IoWriteHandler;
    use output_agent::OutputAgent;
    
    use futures::task::Unpark;
    use futures::Async;
    use std::sync::Arc;
    
    
    pub fn sample_fn(params: Point) -> MethodResult<String, ()> {
        let x_str : String = params.x.to_string();
        let y_str : String = params.y.to_string();
        Ok(x_str + &y_str)
    }
    pub fn no_params_method(_params: ()) -> Result<String, MethodError<()>> {
        Ok("okay".into())
    }
    
    pub fn check_request(result: ResponseResult, expected: ResponseResult) {
        if let ResponseResult::Error(ref error) = result {
            
            if let ResponseResult::Error(expected_error) = expected {
                check_error(error.clone(), expected_error.clone());
                return;
            }
            
        }
        
        assert_equal(&result, &expected);
    }
    
    pub fn async_method(request_params: RequestParams, completable: ResponseCompletable) {
        thread::spawn(move || {
            completable.sync_handle_request(request_params, sample_fn);
        });
    }
        
    fn invoke_method<FN>(
        req_handler: &mut RequestHandler, 
        method_name: &str, 
        request_params: RequestParams, 
        mut and_then: FN
    ) 
    where 
        FN : FnMut(Option<ResponseResult>) + 'static + Send
    {
        let on_response : Box<FnMut(Option<Response>) + Send> = new(move |response: Option<Response>| {
            and_then(response.and_then(|e| Some(e.result_or_error)));
        });
        
        let completable = ResponseCompletable::new(Some(Id::Number(123)), on_response);
        req_handler.handle_request(method_name, request_params, completable);
    }
    
    #[test]
    fn test_Endpoint() {
        
        {
            // Test handle unknown method
            let mut request_handler = MapRequestHandler::new();
            
            let request = Request::new(1, "unknown_method".to_string(), JsonObject::new());
            invoke_method(&mut request_handler, &request.method, request.params,
                |result| 
                check_request(result.unwrap(), ResponseResult::Error(error_JSON_RPC_MethodNotFound())) 
            );
        }
        
        let mut request_handler = MapRequestHandler::new();
        request_handler.add_request("sample_fn", Box::new(sample_fn));
        request_handler.add_rpc_handler("async_method", Box::new(async_method));
        
        // test with invalid params = "{}" 
        let request = Request::new(1, "sample_fn".to_string(), JsonObject::new());
        invoke_method(&mut request_handler, &request.method, request.params, 
            |result| 
            check_request(result.unwrap(), ResponseResult::Error(error_JSON_RPC_InvalidParams(r#"missing field "x""#)))
        );
        
        // test with valid params
        let params_value = match serde_json::to_value(&new_sample_params(10, 20)) {
            Value::Object(object) => object, 
            _ => panic!("Not serialized into Object") 
        };
        let request = Request::new(1, "sample_fn".to_string(), params_value);
        invoke_method(&mut request_handler, &request.method, request.params.clone(),
            |result| 
            assert_equal(result.unwrap(), ResponseResult::Result(
                Value::String("1020".to_string())
            ))
        );
        
        
        // Test valid request with params = "null"
        request_handler.add_request("no_params_method", Box::new(no_params_method));
        
        let id1 = Some(Id::Number(1));
        let request = Request { id : id1, method : "no_params_method".into(), params : RequestParams::None, };
        invoke_method(&mut request_handler, &request.method, request.params.clone(), 
            |result| 
            assert_equal(result.unwrap(), ResponseResult::Result(
                Value::String("okay".to_string())
            ))
        );
        
        // --- Endpoint:
        let output = vec![];
        let output_agent = OutputAgent::start_with_provider(move || IoWriteHandler(output));
        let mut eh = EndpointHandler::create_with_output_agent(output_agent, new(request_handler));
        
        // Test ResponseCompletable - missing id for notification method
        let completable = ResponseCompletable::new(None, new(|_| {}));
        completable.complete(None);
        
        // Test ResponseCompletable - missing id for regular method
        let completable = ResponseCompletable::new(None, new(|_| {}));
        completable.complete(Some(ResponseResult::Result(Value::String("1020".to_string()))));
        
        // test again using handle_request
        // TODO review this code
        let request = Request {     
            id : None,
            method : "sample_fn".into(),
            params : request.params.clone(),
        }; 
        eh.handle_incoming_request(request);
        
        // Test send_request
        
        let params = new_sample_params(123, 66);
        eh.output.send_notification("sample_fn", params.clone()).unwrap();
        
        eh.output.send_notification("async_method", params.clone()).unwrap();
        
        assert_eq!(*eh.output.id_counter.lock().unwrap(), 0);
        
        let my_method = "sample_fn".to_string();
        let future : RequestFuture<String, ()> = eh.output.send_request(&my_method, params.clone()).unwrap();
        
        assert_eq!(*eh.output.id_counter.lock().unwrap(), 1);
        
        // Test future is not completed
        let mut spawn = futures::task::spawn(future);
        let poll = spawn.poll_future(noop_unpark());
        assert_eq!(poll, Ok(Async::NotReady));
        
        // Complete the request
        let expected_result = "sample_fn result".to_string();
        let id = Id::Number(1);
        let response = Response::new_result(id, Value::String(expected_result.clone())); 
        eh.handle_incoming_message(&to_json(&response));

        // ...check future was completed.
        let result : Result<RequestResult<String, ()>, _> = spawn.wait_future();
        assert_eq!(result.unwrap(), RequestResult::MethodResult(Ok(expected_result)));
        
        eh.output.shutdown();
    }
    
    pub fn noop_unpark() -> Arc<Unpark> {
        struct Foo;
        
        impl Unpark for Foo {
            fn unpark(&self) {}
        }
        
        Arc::new(Foo)
    }
    
}

// TODO: investigate: only necessary because of compiler bug?
#[cfg(test)]
fn wait_future<ITEM, ERROR>(future : BoxFuture<ITEM, ERROR>)
    -> Result<ITEM, ERROR> 
{
    future.wait()
}
