extern crate dummy_lsp;


use dummy_lsp::*;
use std::env;
use std::io;
use std::io::Write;
use std::rc::Rc;

fn main() {
	let ls = Rc::new(DummyLanguageServer{ });

	if env::args().len() == 1  {
		// Use stdin/stdout
		
		let stdin = std::io::stdin();
		let out_provider = move || std::io::stdout();
		rust_lsp::lsp_server::LSPServer::start_new(ls, &mut stdin.lock(), out_provider);
	} else {
		let mut args = env::args();
		args.next();
		let port = args.next().unwrap();
		tcp_server(port);
	}
	
}

use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;


fn tcp_server(port_str: String) {
	let port : u16 = port_str.parse::<u16>().expect(&format!("Invalid port number: {}", port_str));
	let listener = TcpListener::bind(("127.0.0.1", port)).unwrap();
	println!("listening on port: {}", port);
	
	for stream in listener.incoming() {
		match stream {
			Ok(stream) => {
				thread::spawn(move|| {
					handle_client(stream)
				});
			}
			Err(err) => {
				writeln!(&mut io::stderr(), "TCP listen error : {:?}", err).expect("Failed writing to stderr");
			}
		}
	}
	
	drop(listener);
}

fn handle_client(stream: TcpStream) {
	//FIXME use same server for each connection
	let ls = Rc::new(DummyLanguageServer{ });
	
	let mut input = io::BufReader::new(stream.try_clone().expect("Failed to clone stream"));
	
	rust_lsp::lsp_server::LSPServer::start_new(ls, &mut input, || {
		stream
	});
}
