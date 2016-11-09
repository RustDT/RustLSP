/* ----------------- Tests ----------------- */


use lsp_server::*;
use jsonrpc::service_util::ServiceError;
use jsonrpc::*;
use ls_types::*;

use std::io;
use std::thread;
use std::net::TcpListener;
use std::net::TcpStream;

pub struct TestsLanguageServer;

impl TestsLanguageServer {
	
	pub fn error_not_available<DATA>(data : DATA) -> ServiceError<DATA> {
		let msg = "Functionality not implemented.".to_string();
		ServiceError::<DATA> { code : 1, message : msg, data : data }
	}
	
}

impl LanguageServer for TestsLanguageServer {
	
	fn initialize(&self, _: InitializeParams, completable: MethodCompletable<InitializeResult, InitializeError>) {
		let capabilities = ServerCapabilities::default();
		completable.complete(Ok(InitializeResult { capabilities : capabilities }))
	}
	fn shutdown(&self, _: (), completable: LSCompletable<()>) {
		completable.complete(Ok(()))
	}
	fn exit(&self, _: ()) {
	}
	
	fn workspace_change_configuration(&self, _: DidChangeConfigurationParams) {}
	fn did_open_text_document(&self, _: DidOpenTextDocumentParams) {}
	fn did_change_text_document(&self, _: DidChangeTextDocumentParams) {}
	fn did_close_text_document(&self, _: DidCloseTextDocumentParams) {}
	fn did_save_text_document(&self, _: DidSaveTextDocumentParams) {}
	fn did_change_watched_files(&self, _: DidChangeWatchedFilesParams) {}
	
	fn completion(&self, _: TextDocumentPositionParams, completable: LSCompletable<CompletionList>) {
		completable.complete(Err(Self::error_not_available(())))
	}
	fn resolve_completion_item(&self, _: CompletionItem, completable: LSCompletable<CompletionItem>) {
		
		completable.complete(Err(Self::error_not_available(())))
	}
	fn hover(&self, _: TextDocumentPositionParams, completable: LSCompletable<Hover>) {
		thread::spawn(|| {
			completable.complete(Err(Self::error_not_available(())))
		});
	}
	fn signature_help(&self, _: TextDocumentPositionParams, completable: LSCompletable<SignatureHelp>) {
		completable.complete(Err(Self::error_not_available(())))
	}
	fn goto_definition(&self, _: TextDocumentPositionParams, completable: LSCompletable<Vec<Location>>) {
		completable.complete(Err(Self::error_not_available(())))
	}
	fn references(&self, _: ReferenceParams, completable: LSCompletable<Vec<Location>>) {
		completable.complete(Err(Self::error_not_available(())))
	}
	fn document_highlight(&self, _: TextDocumentPositionParams, completable: LSCompletable<DocumentHighlight>) {
		completable.complete(Err(Self::error_not_available(())))
	}
	fn document_symbols(&self, _: DocumentSymbolParams, completable: LSCompletable<Vec<SymbolInformation>>) {
		completable.complete(Err(Self::error_not_available(())))
	}
	fn workspace_symbols(&self, _: WorkspaceSymbolParams, completable: LSCompletable<Vec<SymbolInformation>>) {
		completable.complete(Err(Self::error_not_available(())))
	}
	fn code_action(&self, _: CodeActionParams, completable: LSCompletable<Vec<Command>>) {
		completable.complete(Err(Self::error_not_available(())))
	}
	fn code_lens(&self, _: CodeLensParams, completable: LSCompletable<Vec<CodeLens>>) {
		completable.complete(Err(Self::error_not_available(())))
	}
	fn code_lens_resolve(&self, _: CodeLens, completable: LSCompletable<CodeLens>) {
		completable.complete(Err(Self::error_not_available(())))
	}
	fn formatting(&self, _: DocumentFormattingParams, completable: LSCompletable<Vec<TextEdit>>) {
		completable.complete(Err(Self::error_not_available(())))
	}
	fn range_formatting(&self, _: DocumentRangeFormattingParams, completable: LSCompletable<Vec<TextEdit>>) {
		completable.complete(Err(Self::error_not_available(())))
	}
	fn on_type_formatting(&self, _: DocumentOnTypeFormattingParams, completable: LSCompletable<Vec<TextEdit>>) {
		completable.complete(Err(Self::error_not_available(())))
	}
	fn rename(&self, _: RenameParams, completable: LSCompletable<WorkspaceEdit>) {
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
	let ls = TestsLanguageServer{ };
	
	let out_stream = stream.try_clone().expect("Failed to clone stream");
	let endpoint = LSPEndpoint::new_with_output_stream(|| { out_stream });
	
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
	let endpoint = LSPEndpoint::new_with_output_stream(|| { out_stream });
	
	// TODO LSP client
	
	handle.join().unwrap();
}