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

pub trait LanguageServer {
	
	fn initialize(&self, params: InitializeParams) -> LSResult<InitializeResult, InitializeError>;
	fn shutdown(&self, params: ()) -> LSResult<(), ()>;
	fn exit(&self, params: ());
	fn workspace_change_configuration(&self, params: DidChangeConfigurationParams);
	fn did_open_text_document(&self, params: DidOpenTextDocumentParams);
	fn did_change_text_document(&self, params: DidChangeTextDocumentParams);
	fn did_close_text_document(&self, params: DidCloseTextDocumentParams);
	fn did_save_text_document(&self, params: DidSaveTextDocumentParams);
	fn did_change_watched_files(&self, params: DidChangeWatchedFilesParams);
	
	fn completion(&self, params: TextDocumentPositionParams) -> LSResult<CompletionList, ()>;
	fn resolve_completion_item(&self, params: CompletionItem) -> LSResult<CompletionItem, ()>;
	fn hover(&self, params: TextDocumentPositionParams) -> LSResult<Hover, ()>;
	fn signature_help(&self, params: TextDocumentPositionParams) -> LSResult<SignatureHelp, ()>;
	fn goto_definition(&self, params: TextDocumentPositionParams) -> LSResult<Vec<Location>, ()>;
	fn references(&self, params: ReferenceParams) -> LSResult<Vec<Location>, ()>;
	fn document_highlight(&self, params: TextDocumentPositionParams) -> LSResult<DocumentHighlight, ()>;
	fn document_symbols(&self, params: DocumentSymbolParams) -> LSResult<Vec<SymbolInformation>, ()>;
	fn workspace_symbols(&self, params: WorkspaceSymbolParams) -> LSResult<Vec<SymbolInformation>, ()>;
	fn code_action(&self, params: CodeActionParams) -> LSResult<Vec<Command>, ()>;
	fn code_lens(&self, params: CodeLensParams) -> LSResult<Vec<CodeLens>, ()>;
	fn code_lens_resolve(&self, params: CodeLens) -> LSResult<CodeLens, ()>;
	fn formatting(&self, params: DocumentFormattingParams) -> LSResult<Vec<TextEdit>, ()>;
	fn range_formatting(&self, params: DocumentRangeFormattingParams) -> LSResult<Vec<TextEdit>, ()>;
	fn on_type_formatting(&self, params: DocumentOnTypeFormattingParams) -> LSResult<Vec<TextEdit>, ()>;
	fn rename(&self, params: RenameParams) -> LSResult<WorkspaceEdit, ()>;
	
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
			REQUEST__Initialize => { completable.sync_handle_request(params, 
				|params| self.0.initialize(params)) 
			}
			REQUEST__Shutdown => { completable.sync_handle_request(params, 
				|params| self.0.shutdown(params)) 
			}
			NOTIFICATION__Exit => { completable.sync_handle_notification(params, 
				|params| self.0.exit(params)) 
			}
			NOTIFICATION__WorkspaceChangeConfiguration => { completable.sync_handle_notification(params, 
				|params| self.0.workspace_change_configuration(params)) 
			}
			NOTIFICATION__DidOpenTextDocument => { completable.sync_handle_notification(params, 
				|params| self.0.did_open_text_document(params)) 
			}
			NOTIFICATION__DidChangeTextDocument => { completable.sync_handle_notification(params, 
				|params| self.0.did_change_text_document(params)) 
			}
			NOTIFICATION__DidCloseTextDocument => { completable.sync_handle_notification(params, 
				|params| self.0.did_close_text_document(params)) 
			}
			NOTIFICATION__DidSaveTextDocument => { completable.sync_handle_notification(params, 
				|params| self.0.did_save_text_document(params)) 
			}
			NOTIFICATION__DidChangeWatchedFiles => { completable.sync_handle_notification(params, 
				|params| self.0.did_change_watched_files(params)) 
			}
			REQUEST__Completion => { completable.sync_handle_request(params, 
				|params| self.0.completion(params)) 
			}
			REQUEST__ResolveCompletionItem => { completable.sync_handle_request(params, 
				|params| self.0.resolve_completion_item(params)) 
			}
			REQUEST__Hover => { completable.sync_handle_request(params, 
				|params| self.0.hover(params)) 
			}
			REQUEST__SignatureHelp => { completable.sync_handle_request(params, 
				|params| self.0.signature_help(params)) 
			}
			REQUEST__GotoDefinition => { completable.sync_handle_request(params, 
				|params| self.0.goto_definition(params)) 
			}
			REQUEST__References => { completable.sync_handle_request(params, 
				|params| self.0.references(params)) 
			}
			REQUEST__DocumentHighlight => { completable.sync_handle_request(params, 
				|params| self.0.document_highlight(params)) 
			}
			REQUEST__DocumentSymbols => { completable.sync_handle_request(params, 
				|params| self.0.document_symbols(params)) 
			}
			REQUEST__WorkspaceSymbols => { completable.sync_handle_request(params, 
				|params| self.0.workspace_symbols(params)) 
			}
			REQUEST__CodeAction => { completable.sync_handle_request(params, 
				|params| self.0.code_action(params)) 
			}
			REQUEST__CodeLens => { completable.sync_handle_request(params, 
				|params| self.0.code_lens(params)) 
			}
			REQUEST__CodeLensResolve => { completable.sync_handle_request(params, 
				|params| self.0.code_lens_resolve(params)) 
			}
			REQUEST__Formatting => { completable.sync_handle_request(params, 
				|params| self.0.formatting(params)) 
			}
			REQUEST__RangeFormatting => { completable.sync_handle_request(params, 
				|params| self.0.range_formatting(params)) 
			}
			REQUEST__OnTypeFormatting => { completable.sync_handle_request(params, 
				|params| self.0.on_type_formatting(params)) 
			}
			REQUEST__Rename => { completable.sync_handle_request(params, 
				|params| self.0.rename(params)) 
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
//    	endpoint.send_request(NOTIFICATION__ShowMessageRequest, params);
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

/* ----------------- Tests ----------------- */

#[cfg(test)]
mod tests {
    
	use super::*;
    use jsonrpc::service_util::ServiceError;
    use ls_types::*;
    use std::io::BufReader;
    
    
    pub struct TestsLanguageServer;
    
    impl TestsLanguageServer {
    	
    	pub fn error_not_available<DATA>(data : DATA) -> ServiceError<DATA> {
    		let msg = "Functionality not implemented.".to_string();
    		ServiceError::<DATA> { code : 1, message : msg, data : data }
    	}
    	
    }

    impl LanguageServer for TestsLanguageServer {
    	
    	fn initialize(&self, _: InitializeParams) -> LSResult<InitializeResult, InitializeError> {
    		let capabilities = ServerCapabilities::default();
    		Ok(InitializeResult { capabilities : capabilities })
    	}
    	fn shutdown(&self, _: ()) -> LSResult<(), ()> {
    		Ok(())
    	}
    	fn exit(&self, _: ()) {
    	}
    	
    	fn workspace_change_configuration(&self, _: DidChangeConfigurationParams) {}
    	fn did_open_text_document(&self, _: DidOpenTextDocumentParams) {}
    	fn did_change_text_document(&self, _: DidChangeTextDocumentParams) {}
    	fn did_close_text_document(&self, _: DidCloseTextDocumentParams) {}
    	fn did_save_text_document(&self, _: DidSaveTextDocumentParams) {}
    	fn did_change_watched_files(&self, _: DidChangeWatchedFilesParams) {}
    	
    	fn completion(&self, _: TextDocumentPositionParams) -> LSResult<CompletionList, ()> {
    		Err(Self::error_not_available(()))
    	}
    	fn resolve_completion_item(&self, _: CompletionItem) -> LSResult<CompletionItem, ()> {
    		Err(Self::error_not_available(()))
    	}
    	fn hover(&self, _: TextDocumentPositionParams) -> LSResult<Hover, ()> {
    		Err(Self::error_not_available(()))
    	}
    	fn signature_help(&self, _: TextDocumentPositionParams) -> LSResult<SignatureHelp, ()> {
    		Err(Self::error_not_available(()))
    	}
    	fn goto_definition(&self, _: TextDocumentPositionParams) -> LSResult<Vec<Location>, ()> {
    		Err(Self::error_not_available(()))
    	}
    	fn references(&self, _: ReferenceParams) -> LSResult<Vec<Location>, ()> {
    		Err(Self::error_not_available(()))
    	}
    	fn document_highlight(&self, _: TextDocumentPositionParams) -> LSResult<DocumentHighlight, ()> {
    		Err(Self::error_not_available(()))
    	}
    	fn document_symbols(&self, _: DocumentSymbolParams) -> LSResult<Vec<SymbolInformation>, ()> {
    		Err(Self::error_not_available(()))
    	}
    	fn workspace_symbols(&self, _: WorkspaceSymbolParams) -> LSResult<Vec<SymbolInformation>, ()> {
    		Err(Self::error_not_available(()))
    	}
    	fn code_action(&self, _: CodeActionParams) -> LSResult<Vec<Command>, ()> {
    		Err(Self::error_not_available(()))
    	}
    	fn code_lens(&self, _: CodeLensParams) -> LSResult<Vec<CodeLens>, ()> {
    		Err(Self::error_not_available(()))
    	}
    	fn code_lens_resolve(&self, _: CodeLens) -> LSResult<CodeLens, ()> {
    		Err(Self::error_not_available(()))
    	}
    	fn formatting(&self, _: DocumentFormattingParams) -> LSResult<Vec<TextEdit>, ()> {
    		Err(Self::error_not_available(()))
    	}
    	fn range_formatting(&self, _: DocumentRangeFormattingParams) -> LSResult<Vec<TextEdit>, ()> {
    		Err(Self::error_not_available(()))
    	}
    	fn on_type_formatting(&self, _: DocumentOnTypeFormattingParams) -> LSResult<Vec<TextEdit>, ()> {
    		Err(Self::error_not_available(()))
    	}
    	fn rename(&self, _: RenameParams) -> LSResult<WorkspaceEdit, ()> {
    		Err(Self::error_not_available(()))
    	}
    }
    
    #[test]
    pub fn test_run_lsp_server() {
        let out_stream_provider = || { Vec::<u8>::new() };
        
        let endpoint = LSPEndpoint::new_with_output_stream(out_stream_provider);
        
       	let ls = TestsLanguageServer{ };
    	
    	let mut input = BufReader::new("".as_bytes());
    	LSPEndpoint::run_server_from_input(ls, &mut input, endpoint);
    }
    
}