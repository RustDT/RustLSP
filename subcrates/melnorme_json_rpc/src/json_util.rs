// Copyright 2016 Bruno Medeiros
//
// Licensed under the Apache License, Version 2.0 
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>. 
// This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::BTreeMap;
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


pub trait JsonDeserializerHelper<ERR> {
	
	fn new_request_deserialization_error(&self) -> ERR;
	
	fn obtain_Value(&mut self, mut json_map : &mut JsonObject, key: & str) 
		-> Result<Value, ERR> 
	{
		let value = json_map.remove(key);
		match value {
			Some(value) => { Ok(value) }, 
			None => { return Err(self.new_request_deserialization_error()) }
		}
	}
	
	fn obtain_Value_or(&mut self, mut json_map : &mut JsonObject, key: & str, default: & Fn() -> Value) 
		-> Value 
	{
		if let Some(value) = json_map.remove(key) {
			value
		} else {
			default()
		}
	}
	
	fn as_String(&mut self, value: Value) -> Result<String, ERR> {
		match value {
			Value::String(string) => Ok(string),
			_ => Err(self.new_request_deserialization_error()),
		}
	}
	
	fn as_Map(&mut self, value: Value) -> Result<JsonObject, ERR> {
		match value {
			Value::Object(map) => Ok(map),
			_ => Err(self.new_request_deserialization_error()),
		}
	}
	
	fn as_u32(&mut self, value: Value) -> Result<u32, ERR> {
		match value {
			Value::I64(num) => Ok(num as u32), // TODO: check for truncation
			Value::U64(num) => Ok(num as u32), // TODO: check for truncation
			_ => Err(self.new_request_deserialization_error()) ,
		}
	}
	
	
	fn obtain_String(&mut self, json_map : &mut JsonObject, key: &str) 
		-> Result<String, ERR> 
	{
		let value = try!(self.obtain_Value(json_map, key));
		self.as_String(value)
	}
	
	fn obtain_Map(&mut self, json_map : &mut JsonObject, key: &str) 
		-> Result<JsonObject, ERR> 
	{
		let value = try!(self.obtain_Value(json_map, key));
		self.as_Map(value)
	}
	
	fn obtain_Map_or(&mut self, json_map : &mut JsonObject, key: &str, default: & Fn() -> JsonObject) 
		-> Result<JsonObject, ERR> 
	{
		let value = self.obtain_Value_or(json_map, key, &|| { Value::Object(default()) });
		self.as_Map(value)
	}
	
	fn obtain_u32(&mut self, json_map: &mut JsonObject, key: &str) 
		-> Result<u32, ERR> 
	{
		let value = try!(self.obtain_Value(json_map, key));
		self.as_u32(value)
	}

}