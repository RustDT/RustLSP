// Copyright 2016 Bruno Medeiros
//
// Licensed under the Apache License, Version 2.0 
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>. 
// This file may not be copied, modified, or distributed
// except according to those terms.

#[allow(unused_imports)]
use core::*;

use std::io;
use std::fmt;

use std::io::Write;

pub struct StdoutWrite(pub io::Stdout);

impl fmt::Write for StdoutWrite {
	
	fn write_str(&mut self, s: &str) -> fmt::Result {
		match self.0.write_all(s.as_bytes()) {
			Ok(_) => Ok(()),
			Err(_) => Err(fmt::Error),
		}
	}
	
}

pub fn writeNTimes<OUT : ?Sized + fmt::Write>(out : &mut OUT, ch : char, count : u32) -> fmt::Result {
	for _ in 0 .. count {
		try!(out.write_char(ch))
	}
	Ok(())
}

#[test]
fn test_writeNTimes() {
	let mut s = String::new();
	writeNTimes(&mut s, 'a', 2).unwrap();
	assert_eq!(s, "aa");
}