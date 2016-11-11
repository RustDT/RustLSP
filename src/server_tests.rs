/* ----------------- Tests ----------------- */


use lsp_server::*;
use jsonrpc::service_util::ServiceError;
use jsonrpc::*;
use ls_types::*;

use std::io;
use std::thread;
use std::net::TcpListener;
use std::net::TcpStream;

pub struct TestsLanguageServer {
    dummy: u32,
}

impl TestsLanguageServer {
	
	pub fn error_not_available<DATA>(data : DATA) -> ServiceError<DATA> {
		let msg = "Functionality not implemented.".to_string();
		ServiceError::<DATA> { code : 1, message : msg, data : data }
	}
	
}

impl LanguageServer for TestsLanguageServer {
	
	fn initialize(&mut self, _: InitializeParams, completable: MethodCompletable<InitializeResult, InitializeError>) {
		let capabilities = ServerCapabilities::default();
		self.dummy += 1;
		completable.complete(Ok(InitializeResult { capabilities : capabilities }))
	}
	fn shutdown(&mut self, _: (), completable: LSCompletable<()>) {
		completable.complete(Ok(()))
	}
	fn exit(&mut self, _: ()) {
	}
	
	fn workspace_change_configuration(&mut self, _: DidChangeConfigurationParams) {}
	fn did_open_text_document(&mut self, _: DidOpenTextDocumentParams) {}
	fn did_change_text_document(&mut self, _: DidChangeTextDocumentParams) {}
	fn did_close_text_document(&mut self, _: DidCloseTextDocumentParams) {}
	fn did_save_text_document(&mut self, _: DidSaveTextDocumentParams) {}
	fn did_change_watched_files(&mut self, _: DidChangeWatchedFilesParams) {}
	
	fn completion(&mut self, _: TextDocumentPositionParams, completable: LSCompletable<CompletionList>) {
		completable.complete(Err(Self::error_not_available(())))
	}
	fn resolve_completion_item(&mut self, _: CompletionItem, completable: LSCompletable<CompletionItem>) {
		
		completable.complete(Err(Self::error_not_available(())))
	}
	fn hover(&mut self, _: TextDocumentPositionParams, completable: LSCompletable<Hover>) {
		thread::spawn(|| {
			completable.complete(Err(Self::error_not_available(())))
		});
	}
	fn signature_help(&mut self, _: TextDocumentPositionParams, completable: LSCompletable<SignatureHelp>) {
		completable.complete(Err(Self::error_not_available(())))
	}
	fn goto_definition(&mut self, _: TextDocumentPositionParams, completable: LSCompletable<Vec<Location>>) {
		completable.complete(Err(Self::error_not_available(())))
	}
	fn references(&mut self, _: ReferenceParams, completable: LSCompletable<Vec<Location>>) {
		completable.complete(Err(Self::error_not_available(())))
	}
	fn document_highlight(&mut self, _: TextDocumentPositionParams, completable: LSCompletable<Vec<DocumentHighlight>>) {
		completable.complete(Err(Self::error_not_available(())))
	}
	fn document_symbols(&mut self, _: DocumentSymbolParams, completable: LSCompletable<Vec<SymbolInformation>>) {
		completable.complete(Err(Self::error_not_available(())))
	}
	fn workspace_symbols(&mut self, _: WorkspaceSymbolParams, completable: LSCompletable<Vec<SymbolInformation>>) {
		completable.complete(Err(Self::error_not_available(())))
	}
	fn code_action(&mut self, _: CodeActionParams, completable: LSCompletable<Vec<Command>>) {
		completable.complete(Err(Self::error_not_available(())))
	}
	fn code_lens(&mut self, _: CodeLensParams, completable: LSCompletable<Vec<CodeLens>>) {
		completable.complete(Err(Self::error_not_available(())))
	}
	fn code_lens_resolve(&mut self, _: CodeLens, completable: LSCompletable<CodeLens>) {
		completable.complete(Err(Self::error_not_available(())))
	}
	fn formatting(&mut self, _: DocumentFormattingParams, completable: LSCompletable<Vec<TextEdit>>) {
		completable.complete(Err(Self::error_not_available(())))
	}
	fn range_formatting(&mut self, _: DocumentRangeFormattingParams, completable: LSCompletable<Vec<TextEdit>>) {
		completable.complete(Err(Self::error_not_available(())))
	}
	fn on_type_formatting(&mut self, _: DocumentOnTypeFormattingParams, completable: LSCompletable<Vec<TextEdit>>) {
		completable.complete(Err(Self::error_not_available(())))
	}
	fn rename(&mut self, _: RenameParams, completable: LSCompletable<WorkspaceEdit>) {
		completable.complete(Err(Self::error_not_available(())))
	}
}

fn tcp_server(listener: TcpListener) {
	
	for stream in listener.incoming() {
		let stream = stream.expect("Failed to open incoming stream");
		thread::spawn(move|| {
			handle_client(stream)
		});
	}
	
	drop(listener);
}

fn handle_client(stream: TcpStream) {
	let ls = TestsLanguageServer{ dummy : 0 };
	
	let out_stream = stream.try_clone().expect("Failed to clone stream");
	let endpoint = LSPEndpoint::create_lsp_output_with_output_stream(|| { out_stream });
	
	let mut input = io::BufReader::new(stream);
	LSPEndpoint::run_server_from_input(ls, &mut input, endpoint);
}

#[test]
pub fn test_run_lsp_server() {
	// TODO
	if true { return };
	
	let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
	let local_addr = listener.local_addr().unwrap();
	
	let handle = thread::spawn(|| {
		tcp_server(listener)
	});
	
	let stream = TcpStream::connect(local_addr).unwrap();
	let out_stream = stream.try_clone().expect("Failed to clone stream");
	let endpoint = LSPEndpoint::create_lsp_output_with_output_stream(|| { out_stream });
	
	// TODO LSP client
	
	handle.join().unwrap();
}