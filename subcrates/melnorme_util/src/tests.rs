// Copyright 2016 Bruno Medeiros
//
// Licensed under the Apache License, Version 2.0 
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>. 
// This file may not be copied, modified, or distributed
// except according to those terms.


// WARNING: Rust newbie code ahead (-_-)'

//use core::*;

use std;

pub fn assert_equal<T>(left: T, right: T)
	where T : std::fmt::Debug + std::cmp::PartialEq, 
{
	if left != right {
		println!("\n========= Equals failed:");
		println!("{:?}", left);
		println!("{:?}", right);
		println!("====");
	}
	assert_eq!(left, right);
}


use std::sync::Arc;
use std::sync::Mutex; 

pub fn unwrap_Arc<T : std::fmt::Debug>(arc: Arc<T>) -> T {
	Arc::try_unwrap(arc).unwrap()
}

pub fn unwrap_ArcMutex<T : std::fmt::Debug>(arc: Arc<Mutex<T>>) -> T {
	unwrap_Arc(arc).into_inner().unwrap()
}