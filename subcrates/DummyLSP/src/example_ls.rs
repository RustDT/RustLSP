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
use rust_lsp::jsonrpc::method_types::MethodError;
use rust_lsp::jsonrpc::EndpointOutput;
use rust_lsp::jsonrpc::MethodCompletable;

use std::io;

pub struct DummyLanguageServer {
	endpoint_output : EndpointOutput,
}

pub fn run_lsp_server<OUT, OUT_P>(input: &mut io::BufRead, out_stream_provider: OUT_P)
where 
	OUT: io::Write + 'static, 
	OUT_P : FnOnce() -> OUT + Send + 'static
{
	let endpoint_output = LSPEndpoint::create_lsp_output_with_output_stream(out_stream_provider);
	
	let ls = DummyLanguageServer{ endpoint_output : endpoint_output.clone() };
	
	LSPEndpoint::run_server_from_input(ls, input, endpoint_output);
}

/**
 * A no-op language server
 */ 
impl DummyLanguageServer {
	
	// FIXME: user general error
	pub fn error_not_available<DATA>(data : DATA) -> MethodError<DATA> {
		let msg = "Functionality not implemented.".to_string();
		MethodError::<DATA> { code : 1, message : msg, data : data }
	}
	
}

impl LanguageServer for DummyLanguageServer {
	
	fn initialize(&mut self, _: InitializeParams, completable: MethodCompletable<InitializeResult, InitializeError>) {
		let capabilities = ServerCapabilities::default();
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
		completable.complete(Err(Self::error_not_available(())))
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
