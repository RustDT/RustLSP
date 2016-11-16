// Copyright 2016 Bruno Medeiros
//
// Licensed under the Apache License, Version 2.0 
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>. 
// This file may not be copied, modified, or distributed
// except according to those terms.


//use util::core::*;

use super::jsonrpc_types::*;
use serde;
use serde_json;

/* -----------------  ----------------- */

#[derive(Debug, PartialEq)]
pub struct MethodError<DATA> {
    pub code: u32,
    pub message: String,
    pub data: DATA
}

impl<DATA> MethodError<DATA> {
    pub fn new(code: u32, msg: String, data : DATA) -> Self {
        MethodError::<DATA> { code : code, message : msg, data : data }
    }
}

pub type MethodResult<RETURN_VALUE, ERROR_DATA> = Result<RETURN_VALUE, MethodError<ERROR_DATA>>;


impl<RET, RET_ERROR> From<MethodResult<RET, RET_ERROR>> for ResponseResult
where 
    RET : serde::Serialize, 
    RET_ERROR : serde::Serialize,
{
    fn from(method_result: MethodResult<RET, RET_ERROR>) -> Self 
    {
        match method_result {
            Ok(ret) => {
                ResponseResult::Result(serde_json::to_value(&ret)) 
            } 
            Err(error) => {
                let code : u32 = error.code;
                let request_error = RequestError { 
                    code : code as i64, // Safe convertion. TODO: use TryFrom when it's stable
                    message : error.message,
                    data : Some(serde_json::to_value(&error.data)),
                };
                ResponseResult::Error(request_error)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RequestResult<RET, RET_ERROR> {
    MethodResult(MethodResult<RET, RET_ERROR>),
    RequestError(RequestError),
}

impl<
    RET : serde::Deserialize, 
    RET_ERROR : serde::Deserialize, 
> From<ResponseResult> for RequestResult<RET, RET_ERROR> 
{
    fn from(response_result : ResponseResult) -> Self 
    {
        match response_result {
            ResponseResult::Result(result_value) => { 
                let result : Result<RET, _> = serde_json::from_value(result_value);
                match result {
                    Ok(ok) => { 
                        RequestResult::MethodResult(Ok(ok)) 
                    }
                    Err(error) => { 
                        RequestResult::RequestError(error_JSON_RPC_InvalidResponse(error))
                    }
                }
            } 
            ResponseResult::Error(error) => {
                RequestResult::RequestError(error)
            }
        }
    }
}

    #[test]
    fn test__RequestResult_from() {
        use tests_sample_types::*;
        
        // Test JSON RPC error
        let error = error_JSON_RPC_InvalidParams(r#"RPC_ERROR"#);
        let response_result = ResponseResult::Error(error.clone());
        assert_eq!(
            RequestResult::<Point, ()>::from(response_result), 
            RequestResult::RequestError(error)
        );
        
        // Test Ok
        let params = new_sample_params(10, 20);
        let response_result = ResponseResult::Result(serde_json::to_value(&params));
        assert_eq!(
            RequestResult::<Point, ()>::from(response_result), 
            RequestResult::MethodResult(Ok(params.clone()))
        );
        
        // Test invalid MethodResult response 
        let response_result = ResponseResult::Result(serde_json::to_value(&new_sample_params(10, 20)));
        assert_eq!(
            RequestResult::<String, ()>::from(response_result), 
            RequestResult::RequestError(error_JSON_RPC_InvalidResponse(
                r#"invalid type: map at line 0 column 0"#))
        );
    }
    