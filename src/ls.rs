// Copyright 2016 Bruno Medeiros
//
// Licensed under the Apache License, Version 2.0 
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>. 
// This file may not be copied, modified, or distributed
// except according to those terms.


// WARNING: Rust newbie code ahead (-_-)'

#![allow(non_camel_case_types)]

use ::util::core::*;

use lsp::*;

pub type LanguageServerMethod<METHOD_PARAMS, METHOD_RESULT, METHOD_ERROR> = 
	Fn(&LanguageServer, METHOD_PARAMS) -> Result<METHOD_RESULT, METHOD_ERROR>; 

pub type Method_Initialize = LanguageServerMethod<InitializeParams, InitializeResult, InitializeError>;


pub trait LanguageServer {
	
	fn initialize(&self, params: InitializeParams) -> Result<InitializeResult, InitializeError>;
	
}

pub const FN_INITIALIZE : &'static Method_Initialize = &|ls, params| { ls.initialize(params) };

