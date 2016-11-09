// Copyright 2016 Bruno Medeiros
//
// Licensed under the Apache License, Version 2.0 
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>. 
// This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]


extern crate rust_lsp;


use rust_lsp::ls_types::*;
use rust_lsp::lsp_server::*;
use rust_lsp::jsonrpc::service_util::ServiceError;
use rust_lsp::jsonrpc::EndpointHandle;
use rust_lsp::jsonrpc::MethodCompletable;

use std::io;

pub struct DummyLanguageServer {
	server_endpoint : EndpointHandle,
}

pub fn run_lsp_server<OUT, OUT_P>(input: &mut io::BufRead, out_stream_provider: OUT_P)
where 
	OUT: io::Write + 'static, 
	OUT_P : FnOnce() -> OUT + Send + 'static
{
	let endpoint = LSPEndpoint::new_with_output_stream(out_stream_provider);
	
	let ls = DummyLanguageServer{ server_endpoint : endpoint.clone() };
	
	LSPEndpoint::run_server_from_input(ls, input, endpoint);
}

/**
 * A no-op language server
 */ 
impl DummyLanguageServer {
	
	// FIXME: user general error
	pub fn error_not_available<DATA>(data : DATA) -> ServiceError<DATA> {
		let msg = "Functionality not implemented.".to_string();
		ServiceError::<DATA> { code : 1, message : msg, data : data }
	}
	
}

impl LanguageServer for DummyLanguageServer {
	
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
		completable.complete(Err(Self::error_not_available(())))
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
	fn document_highlight(&self, _: TextDocumentPositionParams, completable: LSCompletable<Vec<DocumentHighlight>>) {
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
