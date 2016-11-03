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
	fn workspaceChangeConfiguration(&self, params: DidChangeConfigurationParams);
	fn didOpenTextDocument(&self, params: DidOpenTextDocumentParams);
	fn didChangeTextDocument(&self, params: DidChangeTextDocumentParams);
	fn didCloseTextDocument(&self, params: DidCloseTextDocumentParams);
	fn didSaveTextDocument(&self, params: DidSaveTextDocumentParams);
	fn didChangeWatchedFiles(&self, params: DidChangeWatchedFilesParams);
	
	fn completion(&self, params: TextDocumentPositionParams) -> LSResult<CompletionList, ()>;
	fn resolveCompletionItem(&self, params: CompletionItem) -> LSResult<CompletionItem, ()>;
	fn hover(&self, params: TextDocumentPositionParams) -> LSResult<Hover, ()>;
	fn signatureHelp(&self, params: TextDocumentPositionParams) -> LSResult<SignatureHelp, ()>;
	fn gotoDefinition(&self, params: TextDocumentPositionParams) -> LSResult<Vec<Location>, ()>;
	fn references(&self, params: ReferenceParams) -> LSResult<Vec<Location>, ()>;
	fn documentHighlight(&self, params: TextDocumentPositionParams) -> LSResult<DocumentHighlight, ()>;
	fn documentSymbols(&self, params: DocumentSymbolParams) -> LSResult<Vec<SymbolInformation>, ()>;
	fn workspaceSymbols(&self, params: WorkspaceSymbolParams) -> LSResult<Vec<SymbolInformation>, ()>;
	fn codeAction(&self, params: CodeActionParams) -> LSResult<Vec<Command>, ()>;
	fn codeLens(&self, params: CodeLensParams) -> LSResult<Vec<CodeLens>, ()>;
	fn codeLensResolve(&self, params: CodeLens) -> LSResult<CodeLens, ()>;
	fn formatting(&self, params: DocumentFormattingParams) -> LSResult<Vec<TextEdit>, ()>;
	fn rangeFormatting(&self, params: DocumentRangeFormattingParams) -> LSResult<Vec<TextEdit>, ()>;
	fn onTypeFormatting(&self, params: DocumentOnTypeFormattingParams) -> LSResult<Vec<TextEdit>, ()>;
	fn rename(&self, params: RenameParams) -> LSResult<WorkspaceEdit, ()>;
	
}


pub trait LanguageClientEndpoint {
	
	fn showMessage(&self, params: ShowMessageParams) -> GResult<()>;
	fn showMessageRequest(&self, params: ShowMessageRequestParams) -> GResult<LSResult<MessageActionItem, ()>>;
	fn logMessage(&self, params: LogMessageParams) -> GResult<()>;
	fn telemetryEvent(&self, params: Value) -> GResult<()>;
	
	fn publishDiagnostics(&self, params: PublishDiagnosticsParams) -> GResult<()>;

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
				|params| self.0.workspaceChangeConfiguration(params)) 
			}
			NOTIFICATION__DidOpenTextDocument => { completable.sync_handle_notification(params, 
				|params| self.0.didOpenTextDocument(params)) 
			}
			NOTIFICATION__DidChangeTextDocument => { completable.sync_handle_notification(params, 
				|params| self.0.didChangeTextDocument(params)) 
			}
			NOTIFICATION__DidCloseTextDocument => { completable.sync_handle_notification(params, 
				|params| self.0.didCloseTextDocument(params)) 
			}
			NOTIFICATION__DidSaveTextDocument => { completable.sync_handle_notification(params, 
				|params| self.0.didSaveTextDocument(params)) 
			}
			NOTIFICATION__DidChangeWatchedFiles => { completable.sync_handle_notification(params, 
				|params| self.0.didChangeWatchedFiles(params)) 
			}
			REQUEST__Completion => { completable.sync_handle_request(params, 
				|params| self.0.completion(params)) 
			}
			REQUEST__ResolveCompletionItem => { completable.sync_handle_request(params, 
				|params| self.0.resolveCompletionItem(params)) 
			}
			REQUEST__Hover => { completable.sync_handle_request(params, 
				|params| self.0.hover(params)) 
			}
			REQUEST__SignatureHelp => { completable.sync_handle_request(params, 
				|params| self.0.signatureHelp(params)) 
			}
			REQUEST__GotoDefinition => { completable.sync_handle_request(params, 
				|params| self.0.gotoDefinition(params)) 
			}
			REQUEST__References => { completable.sync_handle_request(params, 
				|params| self.0.references(params)) 
			}
			REQUEST__DocumentHighlight => { completable.sync_handle_request(params, 
				|params| self.0.documentHighlight(params)) 
			}
			REQUEST__DocumentSymbols => { completable.sync_handle_request(params, 
				|params| self.0.documentSymbols(params)) 
			}
			REQUEST__WorkspaceSymbols => { completable.sync_handle_request(params, 
				|params| self.0.workspaceSymbols(params)) 
			}
			REQUEST__CodeAction => { completable.sync_handle_request(params, 
				|params| self.0.codeAction(params)) 
			}
			REQUEST__CodeLens => { completable.sync_handle_request(params, 
				|params| self.0.codeLens(params)) 
			}
			REQUEST__CodeLensResolve => { completable.sync_handle_request(params, 
				|params| self.0.codeLensResolve(params)) 
			}
			REQUEST__Formatting => { completable.sync_handle_request(params, 
				|params| self.0.formatting(params)) 
			}
			REQUEST__RangeFormatting => { completable.sync_handle_request(params, 
				|params| self.0.rangeFormatting(params)) 
			}
			REQUEST__OnTypeFormatting => { completable.sync_handle_request(params, 
				|params| self.0.onTypeFormatting(params)) 
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
	
    fn showMessage(&self, params: ShowMessageParams) -> GResult<()> {
    	let mut endpoint = self.lock().unwrap();
    	try!(endpoint.send_notification(NOTIFICATION__ShowMessage, params));
    	Ok(())
    }
    
    fn showMessageRequest(&self, _params: ShowMessageRequestParams) -> GResult<LSResult<MessageActionItem, ()>> {
    	let endpoint = self.lock().unwrap();
//    	endpoint.send_request(NOTIFICATION__ShowMessageRequest, params);
    	panic!("not implemented")
    }
    
    fn logMessage(&self, params: LogMessageParams) -> GResult<()> {
    	let mut endpoint = self.lock().unwrap();
    	try!(endpoint.send_notification(NOTIFICATION__LogMessage, params));
    	Ok(())
    }
    
    fn telemetryEvent(&self, params: Value) -> GResult<()> {
    	let mut endpoint = self.lock().unwrap();
    	try!(endpoint.send_notification(NOTIFICATION__TelemetryEvent, params));
    	Ok(())
    }
    
    fn publishDiagnostics(&self, params: PublishDiagnosticsParams) -> GResult<()> {
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
    	
    	fn workspaceChangeConfiguration(&self, _: DidChangeConfigurationParams) {}
    	fn didOpenTextDocument(&self, _: DidOpenTextDocumentParams) {}
    	fn didChangeTextDocument(&self, _: DidChangeTextDocumentParams) {}
    	fn didCloseTextDocument(&self, _: DidCloseTextDocumentParams) {}
    	fn didSaveTextDocument(&self, _: DidSaveTextDocumentParams) {}
    	fn didChangeWatchedFiles(&self, _: DidChangeWatchedFilesParams) {}
    	
    	fn completion(&self, _: TextDocumentPositionParams) -> LSResult<CompletionList, ()> {
    		Err(Self::error_not_available(()))
    	}
    	fn resolveCompletionItem(&self, _: CompletionItem) -> LSResult<CompletionItem, ()> {
    		Err(Self::error_not_available(()))
    	}
    	fn hover(&self, _: TextDocumentPositionParams) -> LSResult<Hover, ()> {
    		Err(Self::error_not_available(()))
    	}
    	fn signatureHelp(&self, _: TextDocumentPositionParams) -> LSResult<SignatureHelp, ()> {
    		Err(Self::error_not_available(()))
    	}
    	fn gotoDefinition(&self, _: TextDocumentPositionParams) -> LSResult<Vec<Location>, ()> {
    		Err(Self::error_not_available(()))
    	}
    	fn references(&self, _: ReferenceParams) -> LSResult<Vec<Location>, ()> {
    		Err(Self::error_not_available(()))
    	}
    	fn documentHighlight(&self, _: TextDocumentPositionParams) -> LSResult<DocumentHighlight, ()> {
    		Err(Self::error_not_available(()))
    	}
    	fn documentSymbols(&self, _: DocumentSymbolParams) -> LSResult<Vec<SymbolInformation>, ()> {
    		Err(Self::error_not_available(()))
    	}
    	fn workspaceSymbols(&self, _: WorkspaceSymbolParams) -> LSResult<Vec<SymbolInformation>, ()> {
    		Err(Self::error_not_available(()))
    	}
    	fn codeAction(&self, _: CodeActionParams) -> LSResult<Vec<Command>, ()> {
    		Err(Self::error_not_available(()))
    	}
    	fn codeLens(&self, _: CodeLensParams) -> LSResult<Vec<CodeLens>, ()> {
    		Err(Self::error_not_available(()))
    	}
    	fn codeLensResolve(&self, _: CodeLens) -> LSResult<CodeLens, ()> {
    		Err(Self::error_not_available(()))
    	}
    	fn formatting(&self, _: DocumentFormattingParams) -> LSResult<Vec<TextEdit>, ()> {
    		Err(Self::error_not_available(()))
    	}
    	fn rangeFormatting(&self, _: DocumentRangeFormattingParams) -> LSResult<Vec<TextEdit>, ()> {
    		Err(Self::error_not_available(()))
    	}
    	fn onTypeFormatting(&self, _: DocumentOnTypeFormattingParams) -> LSResult<Vec<TextEdit>, ()> {
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