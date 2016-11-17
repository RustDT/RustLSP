// Copyright 2016 Bruno Medeiros
//
// Licensed under the Apache License, Version 2.0 
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>. 
// This file may not be copied, modified, or distributed
// except according to those terms.


use serde;
use serde_json;
use serde_json::Value;

use jsonrpc_common::*;
use jsonrpc_request::check_jsonrpc_field;
use json_util::*;


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

impl serde::Deserialize for Response {
    fn deserialize<DE>(deserializer: &mut DE) -> Result<Self, DE::Error>
        where DE: serde::Deserializer 
    {
        let mut helper = SerdeJsonDeserializerHelper(deserializer);
        let value = try!(Value::deserialize(helper.0));
        let mut json_obj = try!(helper.as_Object(value));
        
        try!(check_jsonrpc_field(&mut helper, &mut json_obj));
        
        let id_value = try!(helper.obtain_Value(&mut json_obj, "id"));
        let id : Id = try!(serde_json::from_value(id_value).map_err(to_de_error));
        
        let result_or_error : ResponseResult = {
            if let Some(result) = json_obj.remove("result") {
                ResponseResult::Result(result)
            } else  
            if let Some(error_obj) = json_obj.remove("error") {
                let error : RequestError = try!(serde_json::from_value(error_obj).map_err(to_de_error));
                ResponseResult::Error(error)
            } else {
                return Err(new_de_error("Missing property `result` or `error`".to_string()));
            }
        };
        
        Ok(Response{ id : id, result_or_error : result_or_error }) 
    }
}

#[cfg(test)]
pub mod response_tests {

    use super::*;
    use jsonrpc_common::*;
    
    use json_util::*;
    use json_util::test_util::*;
    
    use serde_json::Value;
    use serde_json::builder::ObjectBuilder;

    pub fn sample_json_obj(foo: u32) -> Value {
        ObjectBuilder::new().insert("foo", foo).build()
    }
    
    #[test]
    fn test_Response() {
        
        test_error_de::<Response>(
            r#"{ "id":123, "result":null }"#, 
            "Property `jsonrpc` is missing.",
        );
        test_error_de::<Response>(
            r#"{ "jsonrpc":"1", "id":123, "result":null }"#, 
            r#"Property `jsonrpc` is not "2.0". "#
        );
        
        test_error_de::<Response>(
            r#"{ "jsonrpc":"2.0" }"#, 
            "Property `id` is missing"
        );
        test_error_de::<Response>(
            r#"{ "jsonrpc":"2.0", "id":123 }"#, 
            "Missing property `result` or `error`"
        );

        
        let response = Response::new_result(Id::Null, sample_json_obj(100));
        test_serde_expecting(&response, &ObjectBuilder::new()
            .insert("jsonrpc", "2.0")
            .insert("id", Id::Null)
            .insert("result", sample_json_obj(100))
            .build()
        ); 
        
        let response = Response::new_result(Id::Number(123), sample_json_obj(200));
        test_serde_expecting(&response, &ObjectBuilder::new()
            .insert("jsonrpc", "2.0")
            .insert("id", 123)
            .insert("result", sample_json_obj(200))
            .build()
        );
        
        let response = Response::new_result(Id::Null, sample_json_obj(200));
        test_serde_expecting(&response, &ObjectBuilder::new()
            .insert("jsonrpc", "2.0")
            .insert("id", Value::Null)
            .insert("result", sample_json_obj(200))
            .build()
        );
        
        let response = Response::new_error(Id::String("321".to_string()), RequestError{
            code: 5, message: "msg".to_string(), data: Some(sample_json_obj(300))
        });
        test_serde_expecting(&response, &ObjectBuilder::new()
            .insert("jsonrpc", "2.0")
            .insert("id", "321")
            .insert("error", unwrap_object_builder(ObjectBuilder::new()
                .insert("code", 5)
                .insert("message", "msg")
                .insert("data", sample_json_obj(300))
            ))
            .build()
        );
        
    }
}
