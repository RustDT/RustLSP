extern crate rust_lsp;

use std::io;

fn main() {
	let stdin = io::stdin();
	let stdout = io::stdout();
	
    rust_lsp::rust_lsp_server::RustLSPServer::new().handle_streams(&mut stdin.lock(), &mut stdout.lock());
}

