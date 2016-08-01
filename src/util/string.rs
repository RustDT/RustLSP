// Note: Rust newbie code ahead (-_-)'

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