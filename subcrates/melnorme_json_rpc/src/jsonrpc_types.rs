// Copyright 2016 Bruno Medeiros
//
// Licensed under the Apache License, Version 2.0 
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>. 
// This file may not be copied, modified, or distributed
// except according to those terms.


extern crate serde_json;
extern crate serde;

use serde_json::Value;

use jsonrpc_request::*;
use jsonrpc_response::*;
use json_util::*;

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

//impl serde::Deserialize for Message {
//    fn deserialize<DE>(deserializer: &mut DE) -> Result<Self, DE::Error>
//        where DE: serde::Deserializer 
//    {
//        use serde::Error;
//        
//        let mut helper = SerdeJsonDeserializerHelper(deserializer);
//        let value = try!(Value::deserialize(helper.0));
//        let mut json_obj = try!(helper.as_Object(value));
//        
//        if json_obj.contains_key("method") {
//            let request = serde_json::from_value::<Request>(value);
//            Message::Request(request.map_err(to_de_error))
//        } else {
//            let response = serde_json::from_value::<Response>(value);
//            Message::Response(response.map_err(|_err| DE::Error::custom("Could not parse either Request or Response")))
//        }
//    }
//}


/* ----------------- Tests ----------------- */

#[cfg(test)]
pub mod message_tests {
    
    use super::*;
    use jsonrpc_common::*;
    
    use json_util::*;
    use json_util::test_util::*;
    
    use serde_json::Value;
    use serde_json::builder::ObjectBuilder;

    fn sample_json_obj(foo: u32) -> Value {
        ObjectBuilder::new().insert("foo", foo).build()
    }
    
    #[test]
    fn test_Message() {
        
//        test_error_de::<Response>("{}", "Property `id` is missing");
//
//        test_error_de::<Response>(r#"{ "id":123 }"#, "Missing property `result` or `error`");
//
//        
//        let response = Response::new_result(Id::Null, sample_json_obj(100));
//        test_serde_expecting(&response, &ObjectBuilder::new()
//            .insert("jsonrpc", "2.0")
//            .insert("id", Id::Null)
//            .insert("result", sample_json_obj(100))
//            .build()
//        ); 
//        
//        let response = Response::new_result(Id::Number(123), sample_json_obj(200));
//        test_serde_expecting(&response, &ObjectBuilder::new()
//            .insert("jsonrpc", "2.0")
//            .insert("id", 123)
//            .insert("result", sample_json_obj(200))
//            .build()
//        );
//        
//        let response = Response::new_result(Id::Null, sample_json_obj(200));
//        test_serde_expecting(&response, &ObjectBuilder::new()
//            .insert("jsonrpc", "2.0")
//            .insert("id", Value::Null)
//            .insert("result", sample_json_obj(200))
//            .build()
//        );
//        
//        let response = Response::new_error(Id::String("321".to_string()), RequestError{
//            code: 5, message: "msg".to_string(), data: Some(sample_json_obj(300))
//        });
//        test_serde_expecting(&response, &ObjectBuilder::new()
//            .insert("jsonrpc", "2.0")
//            .insert("id", "321")
//            .insert("error", unwrap_object_builder(ObjectBuilder::new()
//                .insert("code", 5)
//                .insert("message", "msg")
//                .insert("data", sample_json_obj(300))
//            ))
//            .build()
//        );
        
    }
}