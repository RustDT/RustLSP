// Copyright 2016 Bruno Medeiros
//
// Licensed under the Apache License, Version 2.0 
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>. 
// This file may not be copied, modified, or distributed
// except according to those terms.


use std::io;

use util::core::*;

use jsonrpc;
use jsonrpc::*;
use jsonrpc::service_util::MessageReader;
use jsonrpc::service_util::MessageWriter;
use jsonrpc::service_util::ServiceError;

use jsonrpc::output_agent::OutputAgent;

use jsonrpc::jsonrpc_objects::RequestParams;

use lsp_transport;
use ls_types::*;
use serde_json::Value;

/* -----------------  ----------------- */

pub struct LSPMessageReader<'a>(pub &'a mut io::BufRead);

impl<'a> MessageReader for LSPMessageReader<'a> {
	fn read_next(&mut self) -> GResult<String> {
		lsp_transport::parse_transport_message(&mut self.0)
	}
}

pub struct LSPMessageWriter<T: io::Write>(pub T);

impl<T: io::Write> MessageWriter for LSPMessageWriter<T> {
	fn write_message(&mut self, msg: &str) -> Result<(), GError> {
		lsp_transport::write_transport_message(msg, &mut self.0)
	}
}

/* -----------------  ----------------- */

pub struct LSPEndpoint {
	
}

impl LSPEndpoint {
	
	pub fn new_with_output_stream<OUT, OUT_P>(output_stream_provider: OUT_P) 
		-> EndpointHandle
	where 
		OUT: io::Write + 'static, 
		OUT_P : FnOnce() -> OUT + Send + 'static
	{
		Self::new(|| {
			LSPMessageWriter(output_stream_provider())
		})
	}
	
	pub fn new<OUT, OUT_P>(msg_writer_provider: OUT_P) 
		-> EndpointHandle
	where 
		OUT : MessageWriter + 'static, 
		OUT_P : FnOnce() -> OUT + Send + 'static 
	{
		let output_agent = OutputAgent::start_with_provider(msg_writer_provider);
		let endpoint = Endpoint::start_with_output_agent(output_agent, new(MapRequestHandler::new()));
		newArcMutex(endpoint)
	}

	pub fn run_server_from_input<LS>(ls: LS, input: &mut io::BufRead, endpoint: EndpointHandle) 
	where 
		LS: LanguageServer + 'static,
	{
		Self::run_server(ls, &mut LSPMessageReader(input), endpoint)
	}
	
	pub fn run_server<LS, MSG_READER: ?Sized>(ls: LS, msg_reader: &mut MSG_READER, endpoint: EndpointHandle) 
	where 
		LS: LanguageServer + 'static,
		MSG_READER : MessageReader,
	{
		info!("Starting LSP server");
		
		let req_handler : Box<RequestHandler> = Box::new(LSRequestHandler(ls));
		endpoint.lock().unwrap().request_handler = req_handler;
		
		let result = jsonrpc::run_message_read_loop(endpoint, msg_reader);
		
		if let Err(error) = result {
			error!("Error handling the incoming stream: {}", error);
		}
	}
	
}

pub type LSResult<RET, ERR_DATA> = Result<RET, ServiceError<ERR_DATA>>;
pub type LSCompletable<RET> = MethodCompletable<RET, ()>;

pub trait LanguageServer {
	
	fn initialize(&self, params: InitializeParams, completable: MethodCompletable<InitializeResult, InitializeError>);
	fn shutdown(&self, params: (), completable: LSCompletable<()>);
	fn exit(&self, params: ());
	fn workspace_change_configuration(&self, params: DidChangeConfigurationParams);
	fn did_open_text_document(&self, params: DidOpenTextDocumentParams);
	fn did_change_text_document(&self, params: DidChangeTextDocumentParams);
	fn did_close_text_document(&self, params: DidCloseTextDocumentParams);
	fn did_save_text_document(&self, params: DidSaveTextDocumentParams);
	fn did_change_watched_files(&self, params: DidChangeWatchedFilesParams);
	
	fn completion(&self, params: TextDocumentPositionParams, completable: LSCompletable<CompletionList>);
	fn resolve_completion_item(&self, params: CompletionItem, completable: LSCompletable<CompletionItem>);
	fn hover(&self, params: TextDocumentPositionParams, completable: LSCompletable<Hover>);
	fn signature_help(&self, params: TextDocumentPositionParams, completable: LSCompletable<SignatureHelp>);
	fn goto_definition(&self, params: TextDocumentPositionParams, completable: LSCompletable<Vec<Location>>);
	fn references(&self, params: ReferenceParams, completable: LSCompletable<Vec<Location>>);
	fn document_highlight(&self, params: TextDocumentPositionParams, completable: LSCompletable<DocumentHighlight>);
	fn document_symbols(&self, params: DocumentSymbolParams, completable: LSCompletable<Vec<SymbolInformation>>);
	fn workspace_symbols(&self, params: WorkspaceSymbolParams, completable: LSCompletable<Vec<SymbolInformation>>);
	fn code_action(&self, params: CodeActionParams, completable: LSCompletable<Vec<Command>>);
	fn code_lens(&self, params: CodeLensParams, completable: LSCompletable<Vec<CodeLens>>);
	fn code_lens_resolve(&self, params: CodeLens, completable: LSCompletable<CodeLens>);
	fn formatting(&self, params: DocumentFormattingParams, completable: LSCompletable<Vec<TextEdit>>);
	fn range_formatting(&self, params: DocumentRangeFormattingParams, completable: LSCompletable<Vec<TextEdit>>);
	fn on_type_formatting(&self, params: DocumentOnTypeFormattingParams, completable: LSCompletable<Vec<TextEdit>>);
	fn rename(&self, params: RenameParams, completable: LSCompletable<WorkspaceEdit>);
	
}


pub trait LanguageClientEndpoint {
	
	fn show_message(&self, params: ShowMessageParams) -> GResult<()>;
	fn show_message_request(&self, params: ShowMessageRequestParams) -> GResult<LSResult<MessageActionItem, ()>>;
	fn log_message(&self, params: LogMessageParams) -> GResult<()>;
	fn telemetry_event(&self, params: Value) -> GResult<()>;
	
	fn publish_diagnostics(&self, params: PublishDiagnosticsParams) -> GResult<()>;

}

pub struct LSRequestHandler<LS : LanguageServer>(LS);

impl<LS : LanguageServer> RequestHandler for LSRequestHandler<LS> {
	
	fn handle_request(&mut self, method_name: &str, params: RequestParams, 
		completable: ResponseCompletable) 
	{
		match method_name {
			REQUEST__Initialize => {
				completable.handle_request_with(params, 
				    |params, completable| self.0.initialize(params, completable)
				) 
			}
			REQUEST__Shutdown => {
				completable.handle_request_with(params, 
				    |params, completable| self.0.shutdown(params, completable)
				) 
			}
			NOTIFICATION__Exit => { 
			    completable.handle_notification_with(params, 
			        |params| self.0.exit(params)) 
			}
			NOTIFICATION__WorkspaceChangeConfiguration => {
				completable.handle_notification_with(params, 
				    |params| self.0.workspace_change_configuration(params)
				) 
			}
			NOTIFICATION__DidOpenTextDocument => {
				completable.handle_notification_with(params, 
				    |params| self.0.did_open_text_document(params)
				) 
			}
			NOTIFICATION__DidChangeTextDocument => {
				completable.handle_notification_with(params, 
				    |params| self.0.did_change_text_document(params)
				) 
			}
			NOTIFICATION__DidCloseTextDocument => {
				completable.handle_notification_with(params, 
				    |params| self.0.did_close_text_document(params)
				) 
			}
			NOTIFICATION__DidSaveTextDocument => {
				completable.handle_notification_with(params, 
				    |params| self.0.did_save_text_document(params)
				) 
			}
			NOTIFICATION__DidChangeWatchedFiles => {
				completable.handle_notification_with(params, 
				    |params| self.0.did_change_watched_files(params)) 
			}
			REQUEST__Completion => {
				completable.handle_request_with(params, 
				    |params, completable| self.0.completion(params, completable)
				) 
			}
			REQUEST__ResolveCompletionItem => {
				completable.handle_request_with(params, 
				    |params, completable| self.0.resolve_completion_item(params, completable)
				) 
			}
			REQUEST__Hover => {
				completable.handle_request_with(params, 
				    |params, completable| self.0.hover(params, completable)
				) 
			}
			REQUEST__SignatureHelp => {
				completable.handle_request_with(params, 
				    |params, completable| self.0.signature_help(params, completable)
				) 
			}
			REQUEST__GotoDefinition => {
				completable.handle_request_with(params, 
				    |params, completable| self.0.goto_definition(params, completable)
				) 
			}
			REQUEST__References => {
				completable.handle_request_with(params, 
				    |params, completable| self.0.references(params, completable)
				) 
			}
			REQUEST__DocumentHighlight => {
				completable.handle_request_with(params, 
				    |params, completable| self.0.document_highlight(params, completable)
				) 
			}
			REQUEST__DocumentSymbols => {
				completable.handle_request_with(params, 
				    |params, completable| self.0.document_symbols(params, completable)
				) 
			}
			REQUEST__WorkspaceSymbols => {
				completable.handle_request_with(params, 
				    |params, completable| self.0.workspace_symbols(params, completable)
				) 
			}
			REQUEST__CodeAction => {
				completable.handle_request_with(params, 
				    |params, completable| self.0.code_action(params, completable)
				) 
			}
			REQUEST__CodeLens => {
				completable.handle_request_with(params, 
				    |params, completable| self.0.code_lens(params, completable)
				) 
			}
			REQUEST__CodeLensResolve => {
				completable.handle_request_with(params, 
				    |params, completable| self.0.code_lens_resolve(params, completable)
				) 
			}
			REQUEST__Formatting => {
				completable.handle_request_with(params, 
				    |params, completable| self.0.formatting(params, completable)
				) 
			}
			REQUEST__RangeFormatting => {
				completable.handle_request_with(params, 
				    |params, completable| self.0.range_formatting(params, completable)
				) 
			}
			REQUEST__OnTypeFormatting => {
				completable.handle_request_with(params, 
				    |params, completable| self.0.on_type_formatting(params, completable)
				) 
			}
			REQUEST__Rename => {
				completable.handle_request_with(params, 
				    |params, completable| self.0.rename(params, completable)
				) 
			}
			_ => {
				completable.complete_with_error(jsonrpc_objects::error_JSON_RPC_MethodNotFound());
			}
		};
		
	}
	
}

impl LanguageClientEndpoint for EndpointHandle {
	
	fn show_message(&self, params: ShowMessageParams) -> GResult<()> {
		let mut endpoint = self.lock().unwrap();
		try!(endpoint.send_notification(NOTIFICATION__ShowMessage, params));
		Ok(())
	}
	
	fn show_message_request(&self, _params: ShowMessageRequestParams) -> GResult<LSResult<MessageActionItem, ()>> {
		let endpoint = self.lock().unwrap();
//		endpoint.send_request(NOTIFICATION__ShowMessageRequest, params);
		panic!("not implemented")
	}
	
	fn log_message(&self, params: LogMessageParams) -> GResult<()> {
		let mut endpoint = self.lock().unwrap();
		try!(endpoint.send_notification(NOTIFICATION__LogMessage, params));
		Ok(())
	}
	
	fn telemetry_event(&self, params: Value) -> GResult<()> {
		let mut endpoint = self.lock().unwrap();
		try!(endpoint.send_notification(NOTIFICATION__TelemetryEvent, params));
		Ok(())
	}
	
	fn publish_diagnostics(&self, params: PublishDiagnosticsParams) -> GResult<()> {
		let mut endpoint = self.lock().unwrap();
		try!(endpoint.send_notification(NOTIFICATION__PublishDiagnostics, params));
		Ok(())
	}
	
}

