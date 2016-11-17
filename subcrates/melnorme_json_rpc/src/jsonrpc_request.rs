// Copyright 2016 Bruno Medeiros
//
// Licensed under the Apache License, Version 2.0 
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>. 
// This file may not be copied, modified, or distributed
// except according to those terms.


use serde;
use serde::de::Visitor;
use serde::de;
use serde::Error;

use serde_json;
use serde_json::Value;

use util::core::GResult;

use jsonrpc_common::*;
use json_util::*;

/* -----------------  ----------------- */

pub fn check_jsonrpc_field<ERR, HELPER>(helper: &mut HELPER, json_obj: &mut JsonObject) -> Result<(), ERR>
where 
    HELPER: JsonDeserializerHelper<ERR>, 
{
    let jsonrpc = try!(helper.obtain_String(json_obj, "jsonrpc"));
    if jsonrpc != "2.0" {
        return Err(helper.new_error(r#"Property `jsonrpc` is not "2.0". "#))
    };
    Ok(())
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

pub fn parse_jsonrpc_request(message: &str) -> JsonRpcParseResult<Request> {
    serde_json::from_str(message).map_err(error_JSON_RPC_InvalidRequest) 
}

impl serde::Deserialize for Request {
    fn deserialize<DE>(deserializer: &mut DE) -> Result<Self, DE::Error>
        where DE: serde::Deserializer 
    {
        let mut helper = SerdeJsonDeserializerHelper(deserializer);
        let value = try!(Value::deserialize(helper.0));
        let mut json_obj = try!(helper.as_Object(value));
        
        try!(check_jsonrpc_field(&mut helper, &mut json_obj));
        
        let id = json_obj.remove("id");
        let id = try!(id.map_or(Ok(None), |value| serde_json::from_value(value).map_err(to_de_error)));
        let method = try!(helper.obtain_String(&mut json_obj, "method"));
        let params = try!(helper.obtain_Value(&mut json_obj, "params"));
        
        let params = try!(to_jsonrpc_params(params).map_err(to_de_error));
        
        Ok(Request { id : id, method : method, params : params })
    }
}


/* -----------------  ----------------- */

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

pub fn to_jsonrpc_params(params: Value) -> GResult<RequestParams> {
    match params {
        Value::Object(object) => Ok(RequestParams::Object(object)),
        Value::Array(array) => Ok(RequestParams::Array(array)),
        Value::Null => Ok(RequestParams::None),
        _ => Err("Property `params` not an Object, Array, or null.".into()),
    }
}

impl serde::Deserialize for RequestParams {
    fn deserialize<DE>(deserializer: &mut DE) -> Result<Self, DE::Error>
        where DE: serde::Deserializer 
    {
        deserializer.deserialize(RequestParams_DeserializeVisitor)
    }
}

struct RequestParams_DeserializeVisitor;

impl Visitor for RequestParams_DeserializeVisitor {
    type Value = RequestParams;
    
    fn visit_unit<E>(&mut self) -> Result<Self::Value, E> 
        where E: Error,
    {
        Ok(RequestParams::None)
    }
    
    fn visit_seq<V>(&mut self, visitor: V) -> Result<Self::Value, V::Error>
        where V: de::SeqVisitor,
    {
        let values = try!(de::impls::VecVisitor::new().visit_seq(visitor));
        Ok(RequestParams::Array(values))
    }

    fn visit_map<V>(&mut self, visitor: V) -> Result<Self::Value, V::Error>
        where V: de::MapVisitor,
    {
        let values = try!(de::impls::BTreeMapVisitor::new().visit_map(visitor));
        Ok(RequestParams::Object(values))
    }
    
}



#[cfg(test)]
pub mod request_tests {

    use super::*;
    
    use util::tests::*;
    use json_util::*;
    use json_util::test_util::*;
    use jsonrpc_common::*;
    
    use serde_json::Value;
    use serde_json::builder::ObjectBuilder;


    #[test]
    fn test__RequestParams() {
        
        let sample_obj = unwrap_object_builder(ObjectBuilder::new().insert("xxx", 123));
        let sample_string = Value::String("blah".into());
        
        test_serde__RequestParams(RequestParams::Object(sample_obj.clone()));
        test_serde__RequestParams(RequestParams::Array(vec![sample_string.clone(), sample_string]));
        test_serde__RequestParams(RequestParams::None);
    }
    
    fn test_serde__RequestParams(params: RequestParams) {
        let params_reser = test_serde(&params).0;
        assert_equal(params_reser, params);
    }
    
    pub fn check_error(result: RequestError, expected: RequestError) {
        assert_starts_with(&result.message, &expected.message);
        assert_eq!(result, RequestError { message : result.message.clone(), .. expected }); 
    }
    
    #[test]
    fn test_Request() {
        
        let sample_params = unwrap_object_builder(ObjectBuilder::new()
            .insert("param", "2.0")
            .insert("foo", 123)
        );
        
        // Test invalid JSON
        check_error(parse_jsonrpc_request("{" ).unwrap_err(), error_JSON_RPC_InvalidRequest("EOF"));
        
        test_error_de::<Request>(
            "{ }",
            "Property `jsonrpc` is missing.",
        );
        
        test_error_de::<Request>(
            r#"{ "jsonrpc": "1.0" }"#,
            r#"Property `jsonrpc` is not "2.0". "#,
        );
        
        test_error_de::<Request>(
            r#"{ "jsonrpc": "2.0" }"#,
            "Property `method` is missing.",
        );
        test_error_de::<Request>(
            r#"{ "jsonrpc": "2.0", "method":null }"#,
            "Value `null` is not a String.",
        );
        
        test_error_de::<Request>(
            r#"{ "jsonrpc": "2.0", "method":"xxx" }"#,
            "Property `params` is missing.",
        );
        
        // Test valid request with params = null
        assert_equal(
            parse_jsonrpc_request(r#"{ "jsonrpc": "2.0", "method":"xxx", "params":null }"#),
            Ok(Request { id : None, method : "xxx".into(), params : RequestParams::None, }) 
        );
        
        // --- Test serialization ---
        
        // basic Request
        let request = Request::new(1, "myMethod".to_string(), sample_params.clone());
        test_serde(&request);
        
        // Test basic Request, no params
        let request = Request { id : None, method : "myMethod".to_string(), params : RequestParams::None, };
        test_serde(&request);
        
        // Test Request with no id
        let sample_array_params = RequestParams::Array(vec![]);
        let request = Request { id : None, method : "myMethod".to_string(), params : sample_array_params, };  
        test_serde(&request);
    }
    
}