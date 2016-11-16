// Copyright 2016 Bruno Medeiros
//
// Licensed under the Apache License, Version 2.0 
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>. 
// This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::BTreeMap;

use serde;
use serde::Error;

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


pub struct SerdeJsonDeserializerHelper<DE>(pub DE);

impl<'a, DE : serde::Deserializer> 
    JsonDeserializerHelper<DE::Error> for SerdeJsonDeserializerHelper<&'a mut DE> 
{
    fn new_error(&self, error_message: &str) -> DE::Error {
        to_de_error::<DE>(error_message.into())
    }
}

pub fn to_de_error<DE>(message: String) 
    -> DE::Error  
    where DE: serde::Deserializer 
{
    DE::Error::custom(message)
}


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

#[cfg(test)]
pub mod test_util {
    
    use util::tests::*;
    use serde::Serialize;
    use serde::Deserialize;
    use serde_json;
    use std::fmt::Debug;
    
    pub fn to_json<T: Serialize>(value: &T) -> String {
        serde_json::to_string(value).unwrap()
    }
    
    pub fn from_json<T: Deserialize>(json: &str) -> T {
        serde_json::from_str(json).unwrap()
    }

    pub fn check_serde<T>(obj: T) 
        -> String
        where T : Serialize + Deserialize + PartialEq + Debug
    {
        let json = serde_json::to_string(&obj).unwrap();
        let reserialized : T = serde_json::from_str(&json).unwrap();
        check_equal(reserialized, obj);
        json
    }
    
}