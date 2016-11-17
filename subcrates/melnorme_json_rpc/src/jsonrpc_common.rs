// Copyright 2016 Bruno Medeiros
//
// Licensed under the Apache License, Version 2.0 
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>. 
// This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

use serde;
use serde::de::Visitor;
use serde::Error;

use serde_json::Value;
use serde_json;
use json_util::*;



pub struct JsonRequestDeserializerHelper;

impl JsonDeserializerHelper<RequestError> for JsonRequestDeserializerHelper {
    
    fn new_error(&self, error_message: &str) -> RequestError {
        error_JSON_RPC_InvalidRequest(error_message.to_string())
    }
    
}

pub type JsonRpcParseResult<T> = Result<T, RequestError>;

pub fn parse_jsonrpc_id(id: Value) -> JsonRpcParseResult<Option<Id>> {
    serde_json::from_value(id)
        .map_err(|err| error_JSON_RPC_InvalidRequest(format!("Invalid id: {}", err)))
}


/* ----------------- Id ----------------- */

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// A JSON RPC Id
/// Note: only supports u64 numbers
pub enum Id { Number(u64), String(String), Null, }

impl serde::Serialize for Id {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer,
    {
        match *self {
            Id::Null => serializer.serialize_none(),
            Id::Number(number) => serializer.serialize_u64(number), 
            Id::String(ref string) => serializer.serialize_str(string),
        }
    }
}

impl serde::Deserialize for Id {
    fn deserialize<DE>(deserializer: &mut DE) -> Result<Self, DE::Error>
        where DE: serde::Deserializer 
    {
        deserializer.deserialize(IdDeserializeVisitor)
    }
}

struct IdDeserializeVisitor;

impl Visitor for IdDeserializeVisitor {
    type Value = Id;
    
    fn visit_unit<E>(&mut self) -> Result<Self::Value, E> where E: Error,
    {
        Ok(Id::Null)
    }
    
    fn visit_u64<E>(&mut self, value: u64) -> Result<Self::Value, E> where E: Error,
    {
        Ok(Id::Number(value))
    }
    
    fn visit_str<E>(&mut self, value: &str) -> Result<Self::Value, E> where E: Error,
    {
        Ok(Id::String(value.to_string()))
    }
}

#[test]
fn test_Id() {
    use json_util::test_util::*;
    
    test_serde(&Id::Null);
    test_serde(&Id::Number(123));
    test_serde(&Id::String("123".into()));
    test_serde(&Id::String("".into()));
    test_serde(&Id::String("foo".into()));
    
    // FIXME better handling of non-u64 numbers?
//    assert_eq!(from_json::<Id>("-123"), Id::Number(123)); 
}


/* -----------------  Error  ----------------- */

#[derive(Debug, PartialEq, Clone)]
pub struct RequestError {
    pub code : i64,
    pub message : String,
    pub data : Option<Value>,
}

impl RequestError {
    pub fn new(code: i64, message: String) -> RequestError {
        RequestError { code : code, message : message, data : None }
    }
}

pub fn error_JSON_RPC_ParseError<T: fmt::Display>(error: T) -> RequestError { 
    RequestError::new(-32700, format!("Invalid JSON was received by the server: {}", error).to_string())
}
pub fn error_JSON_RPC_InvalidRequest<T: fmt::Display>(error: T) -> RequestError { 
    RequestError::new(-32600, format!("The JSON sent is not a valid Request object: {}", error).to_string())
}
pub fn error_JSON_RPC_MethodNotFound() -> RequestError { 
    RequestError::new(-32601, "The method does not exist / is not available.".to_string())
}
pub fn error_JSON_RPC_InvalidParams<T: fmt::Display>(error: T) -> RequestError { 
    RequestError::new(-32602, format!("Invalid method parameter(s): {}", error).to_string())
}
pub fn error_JSON_RPC_InternalError() -> RequestError { 
    RequestError::new(-32603, "Internal JSON-RPC error.".to_string())
}

pub fn error_JSON_RPC_InvalidResponse<T: fmt::Display>(error: T) -> RequestError { 
    RequestError::new(-32000, format!("Invalid method response: {}", error).to_string())
}

impl serde::Serialize for RequestError {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer
    {
        let elem_count = 3;
        let mut state = try!(serializer.serialize_struct("RequestError", elem_count)); 
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

impl serde::Deserialize for RequestError {
    fn deserialize<DE>(deserializer: &mut DE) -> Result<Self, DE::Error>
        where DE: serde::Deserializer 
    {
        let mut helper = SerdeJsonDeserializerHelper(deserializer);
        let value : Value = try!(Value::deserialize(helper.0));
        let mut json_obj = try!(helper.as_Object(value));
        
        let code = try!(helper.obtain_i64(&mut json_obj, "code"));
        let message = try!(helper.obtain_String(&mut json_obj, "message"));
        
        let data = json_obj.remove("data"); 
        
        Ok(RequestError{ code : code, message : message, data : data }) 
    }
}

#[test]
fn test_RequestError() {
    use json_util::test_util::*;
    
    test_serde(&RequestError::new(12, "asd".into()));
    test_serde(&RequestError{ code : -123, message : "abc".into(), data : None });
    
    test_serde(&RequestError{ code : 1, message : "xxx".into(), data : Some(Value::Null) });
    test_serde(&RequestError{ code : 1, message : "xxx".into(), data : Some(Value::String("asdf".into())) });
    
    test_error_de::<RequestError>("{}", "Property `code` is missing");
}
