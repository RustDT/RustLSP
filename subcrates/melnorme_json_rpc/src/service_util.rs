// Copyright 2016 Bruno Medeiros
//
// Licensed under the Apache License, Version 2.0 
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>. 
// This file may not be copied, modified, or distributed
// except according to those terms.

use std::result::Result;

use util::core::*;

pub trait Provider<VALUE, ERR> {
	fn obtain_next(&mut self) -> Result<VALUE, ERR>;
}

pub trait MessageWriter {
	
	fn write_message(&mut self, msg: &str) -> Result<(), GError>;
	
}

pub struct ServiceError<DATA> {
	pub code : u32,
	pub message : String,
	pub data : DATA
}

pub type ServiceResult<RETURN_VALUE, ERROR_DATA> = Result<RETURN_VALUE, ServiceError<ERROR_DATA>>;

pub type ServiceHandler<PARAMS, RETURN_VALUE, ERROR_DATA> = Fn(PARAMS) -> ServiceResult<RETURN_VALUE, ERROR_DATA>;
