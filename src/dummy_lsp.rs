extern crate rust_lsp;


use rust_lsp::lsp::*;

fn main() {
	let stdin = std::io::stdin();
	let stdout = std::io::stdout();
	
	let mut ls = std::rc::Rc::new(DummyLanguageServer{ }); 
	
	rust_lsp::rust_lsp_server::LSPServer::start_new(ls, &mut stdin.lock(), &mut stdout.lock());
}