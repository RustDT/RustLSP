// Copyright 2015 Bruno Medeiros
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use ::util::core::*;

use std::io;
use std::fmt;

use std::io::Write;

impl CharOutput<fmt::Error> for StdoutWrite {
	
    fn write_str(&mut self, string: &str) -> fmt::Result {
    	fmt::Write::write_str(self, string)
    }
	
    fn write_char(&mut self, c: char) -> fmt::Result {
    	fmt::Write::write_char(self, c)
    }
	
}

//impl fmt::Write for CharOutput<fmt::Error> {
//	
//	fn write_str(&mut self, s: &str) -> fmt::Result {
//		CharOutput::<fmt::Error>::write_str(self, s)
//	}
//	
//}
//
//impl fmt::Debug for CharOutput<fmt::Error> {
//	
//	fn fmt(&self, fmt : &mut fmt::Formatter) -> fmt::Result {
//		fmt.write_str("[CharOutput<fmt::Error>]")
//	}
//	
//}

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