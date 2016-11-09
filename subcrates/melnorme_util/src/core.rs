// Copyright 2016 Bruno Medeiros
//
// Licensed under the Apache License, Version 2.0 
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>. 
// This file may not be copied, modified, or distributed
// except according to those terms.


use std::result;
use std::error::Error;

pub fn new<T>(x: T) -> Box<T> {
	Box::new(x)
}

/* -----------------  Error handling  ----------------- */

pub type GError = Box<Error>;
pub type GResult<T> = result::Result<T, GError>;
pub type Void = GResult<()>;


/* -----------------  lifecycle / dispose  ----------------- */


pub struct FinishedFlag(pub bool);

impl FinishedFlag {
	
	pub fn is_finished(&self) -> bool {
		return self.0
	}
	
	/// Set this flag as finished. Can only be invoked once.
	pub fn finish(&mut self) {
		assert!(!self.is_finished());
		self.set_finished();
	}
	
	pub fn set_finished(&mut self) {
		self.0 = true;
	}
	
}

impl Drop for FinishedFlag {
	
	fn drop(&mut self) {
		assert!(self.is_finished());
	}
}

/* -----------------  Sync and Rc util ----------------- */


use std::sync::Arc;
use std::sync::Mutex;

pub fn newArcMutex<T>(x: T) -> Arc<Mutex<T>> {
	Arc::new(Mutex::new(x))
}


use std::rc::Rc;
use std::cell::RefCell;

pub fn unwrap_Rc_RefCell<T>(this: Rc<RefCell<T>>) -> T {
	let ures : result::Result<RefCell<_>, _> = Rc::try_unwrap(this);
	match ures {
		Ok(refCell) => return refCell.into_inner(),
		Err(_) => panic!("std::Rc unwrap failed")
	}
}