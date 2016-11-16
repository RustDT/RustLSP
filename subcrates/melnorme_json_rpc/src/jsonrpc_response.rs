// Copyright 2016 Bruno Medeiros
//
// Licensed under the Apache License, Version 2.0 
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>. 
// This file may not be copied, modified, or distributed
// except according to those terms.


use serde;

use serde_json::Value;

use jsonrpc_common::*;
use jsonrpc_types::Message;


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

#[cfg(test)]
mod response_tests {

    use super::*;
    use jsonrpc_common::*;
    
    use util::tests::*;
    use json_util::*;
    use json_util::test_util::*;
    
    use serde_json;
    use serde_json::Value;
    use serde_json::builder::ObjectBuilder;

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
