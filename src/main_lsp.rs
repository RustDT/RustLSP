extern crate rust_lsp;


use rust_lsp::ls::*;
use rust_lsp::lsp::*;

struct DummyLanguageServer {
	
}
impl LanguageServer for DummyLanguageServer {
	
	fn initialize(&self, params: InitializeParams) -> Result<InitializeResult, InitializeError> {
		Ok(InitializeResult { capabilities : ServerCapabilities::default() })
	}
}

fn main() {
	let stdin = std::io::stdin();
	let stdout = std::io::stdout();
	
	let mut ls = std::rc::Rc::new(DummyLanguageServer{ }); 
	
	rust_lsp::rust_lsp_server::RustLSPServer::start_new(ls, &mut stdin.lock(), &mut stdout.lock());
}