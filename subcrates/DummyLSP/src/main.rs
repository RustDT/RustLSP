extern crate dummy_lsp;


use dummy_lsp::*;

fn main() {
	let ls = std::rc::Rc::new(DummyLanguageServer{ });
	
	let stdin = std::io::stdin();
	let out_provider = move || std::io::stdout();
	
	rust_lsp::lsp_server::LSPServer::start_new(ls, &mut stdin.lock(), out_provider);
}