// Copyright 2016 Bruno Medeiros
//
// Licensed under the Apache License, Version 2.0 
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>. 
// This file may not be copied, modified, or distributed
// except according to those terms.


extern crate serde_json;
extern crate serde;


use jsonrpc_request::*;
use jsonrpc_response::*;

/* -----------------  Deserialize helper  ----------------- */



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
//        let value : Value = try!(Value::deserialize(deserializer));
//        if value.find("method").is_some() {
//            let request = serde_json::from_value::<Request>(value));
//            Message::Request(request.map_err(|_err| DE::Error::custom("Could not parse either Request or Response"))
//        } else {
//            let response = serde_json::from_value::<Response>(value));
//            Message::Response(response.map_err(|_err| DE::Error::custom("Could not parse either Request or Response"))
//        }
//    }
//}


/* ----------------- Tests ----------------- */

#[cfg(test)]
pub mod message_tests {
    
}