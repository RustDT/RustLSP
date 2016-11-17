// Copyright 2016 Bruno Medeiros
//
// Licensed under the Apache License, Version 2.0 
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>. 
// This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::BTreeMap;
use std::fmt;

use serde;

use serde_json::Map;
use serde_json::Value;
use serde_json::builder::ObjectBuilder;

pub type JsonObject = Map<String, Value>;

/* ----------------- deserialize helpers ----------------- */

pub fn new_object() -> JsonObject {
    BTreeMap::new()
}

pub fn unwrap_object_builder(ob: ObjectBuilder) -> JsonObject {
    unwrap_object(ob.build())
}

pub fn unwrap_object(value: Value) -> JsonObject {
    match value {
        Value::Object(o) => o ,
        _ => { panic!() },
    }
}

/* -----------------  ----------------- */

pub trait JsonDeserializerHelper<ERR> {
    
    fn new_error(&self, error_message: &str) -> ERR;
    
    fn obtain_Value(&mut self, mut json_map : &mut JsonObject, key: & str) 
        -> Result<Value, ERR>
    {
        let value = json_map.remove(key);
        match value {
            Some(value) => { Ok(value) },
            None => { return Err(self.new_error(&format!("Property `{}` is missing.", key))) }
        }
    }
    
    fn obtain_Value_or(&mut self, mut json_map : &mut JsonObject, key: & str, default: & Fn() -> Value) 
        -> Value 
    {
        if let Some(value) = json_map.remove(key) {
            if let Value::Null = value {
                default()
            } else {
                value
            }
        } else {
            default()
        }
    }
    
    fn as_String(&mut self, value: Value) -> Result<String, ERR> {
        match value {
            Value::String(string) => Ok(string),
            _ => Err(self.new_error(&format!("Value `{}` is not a String.", value))),
        }
    }
    
    fn as_Object(&mut self, value: Value) -> Result<JsonObject, ERR> {
        match value {
            Value::Object(map) => Ok(map),
            _ => Err(self.new_error(&format!("Value `{}` is not an Object.", value))),
        }
    }
    
    fn as_u32(&mut self, value: Value) -> Result<u32, ERR> {
        match value {
            Value::I64(num) => Ok(num as u32), // FIXME: check for truncation
            Value::U64(num) => Ok(num as u32), // FIXME: check for truncation
            _ => Err(self.new_error(&format!("Value `{}` is not an Integer.", value))),
        }
    }
    
    fn as_i64(&mut self, value: Value) -> Result<i64, ERR> {
        match value {
            Value::I64(num) => Ok(num),
            Value::U64(num) => Ok(num as i64), // FIXME: check for truncation
            _ => Err(self.new_error(&format!("Value `{}` is not an Integer.", value))),
        }
    }
    
    
    fn obtain_String(&mut self, json_map : &mut JsonObject, key: &str) 
        -> Result<String, ERR> 
    {
        let value = try!(self.obtain_Value(json_map, key));
        self.as_String(value)
    }
    
    fn obtain_Object(&mut self, json_map : &mut JsonObject, key: &str) 
        -> Result<JsonObject, ERR> 
    {
        let value = try!(self.obtain_Value(json_map, key));
        self.as_Object(value)
    }
    
    fn obtain_Object_or(&mut self, json_map : &mut JsonObject, key: &str, default: & Fn() -> JsonObject) 
        -> Result<JsonObject, ERR> 
    {
        let value = self.obtain_Value_or(json_map, key, &|| { Value::Object(default()) });
        self.as_Object(value)
    }
    
    fn obtain_u32(&mut self, json_map: &mut JsonObject, key: &str) 
        -> Result<u32, ERR> 
    {
        let value = try!(self.obtain_Value(json_map, key));
        self.as_u32(value)
    }
    
    fn obtain_i64(&mut self, json_map: &mut JsonObject, key: &str) 
        -> Result<i64, ERR> 
    {
        let value = try!(self.obtain_Value(json_map, key));
        self.as_i64(value)
    }

}

pub struct SerdeJsonDeserializerHelper<DE>(pub DE);

impl<'a, DE : serde::Deserializer> 
    JsonDeserializerHelper<DE::Error> for SerdeJsonDeserializerHelper<&'a mut DE> 
{
    fn new_error(&self, error_message: &str) -> DE::Error {
        new_de_error(error_message.into())
    }
}

pub fn to_de_error<DISPLAY, DE_ERROR>(display: DISPLAY) 
    -> DE_ERROR   
where 
    DISPLAY: fmt::Display,
    DE_ERROR: serde::Error, 
{
    DE_ERROR::custom(format!("{}", display))
}

pub fn new_de_error<DE_ERROR>(message: String) 
    -> DE_ERROR
    where DE_ERROR: serde::Error 
{
    DE_ERROR::custom(message)
}

/* -----------------  ----------------- */

#[cfg(test)]
pub mod test_util {
    
    use util::tests::*;
    use serde::Serialize;
    use serde::Deserialize;
    use serde_json;
    use serde_json::Value;
    use std::fmt::Debug;
    
    pub fn to_json<T: Serialize>(value: &T) -> String {
        serde_json::to_string(value).unwrap()
    }
    
    pub fn from_json<T: Deserialize>(json: &str) -> T {
        serde_json::from_str(json).unwrap()
    }

    pub fn test_serde<T>(obj: &T) 
        -> (T, String)
        where T : Serialize + Deserialize + PartialEq + Debug
    {
        let json = to_json(obj);
        let reserialized : T = from_json(&json);
        check_equal(&reserialized, obj);
        (reserialized, json)
    }
    
    pub fn test_error_de<T>(json: &str, expected_err_contains: &str) 
        where T : Deserialize + PartialEq + Debug
    {
        let res = serde_json::from_str::<T>(json).unwrap_err();
        check_err_contains(res, expected_err_contains);
    }
    
    pub fn test_serde_expecting<T>(obj: &T, expected_value: &Value) 
        -> Value
        where T : Serialize + Deserialize + PartialEq + Debug
    {
        let json = test_serde(obj).1;
        
        let as_value : Value = serde_json::from_str(&json).unwrap();
        check_equal(&as_value, expected_value);
        as_value
    }
    
}