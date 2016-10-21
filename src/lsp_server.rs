// Copyright 2016 Bruno Medeiros
//
// Licensed under the Apache License, Version 2.0 
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>. 
// This file may not be copied, modified, or distributed
// except according to those terms.


#![allow(non_camel_case_types)]

extern crate serde;

use std::io;

use util::core::*;

use jsonrpc;
use jsonrpc::*;
use jsonrpc::service_util::Provider;
use jsonrpc::service_util::MessageWriter;
use jsonrpc::service_util::ServiceError;

use jsonrpc::output_agent::OutputAgent;

use jsonrpc::jsonrpc_objects::RequestParams;

use lsp;
use lsp_transport;
use lsp::*;

use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

/* -----------------  ----------------- */

struct LSPMessageReader<'a>(&'a mut io::BufRead);

impl<'a> Provider<String, GError> for LSPMessageReader<'a> {
	fn obtain_next(&mut self) -> GResult<String> {
		lsp_transport::parse_transport_message(&mut self.0)
	}
}

struct LSPMessageWriter<T: io::Write>(pub T);

impl<T: io::Write> MessageWriter for LSPMessageWriter<T> {
	fn write_message(&mut self, msg: &str) -> Result<(), GError> {
		lsp_transport::write_transport_message(msg, &mut self.0)
	}
}

/* -----------------  ----------------- */


pub struct LSPServer {
	
}

impl LSPServer {
	
	pub fn start_new<OUT, OUT_P>(
		ls: Rc<LanguageServer>, input: &mut io::BufRead, out_stream_provider: OUT_P
	) 
	where 
		OUT: io::Write + 'static, 
		OUT_P : FnOnce() -> OUT + Send + 'static 
	{
		
		let output_agent = OutputAgent::start_with_provider(|| {
			LSPMessageWriter(out_stream_provider())
		});
		let mut jsonrpc_endpoint = Endpoint::start_with_output_agent(output_agent, new(MapRequestHandler::new()));
		
		let mut request_handler = new(MapRequestHandler::new());
		//FIXME
		//initialize_methods(ls.clone(), &mut request_handler);
		jsonrpc_endpoint.request_handler = request_handler;
		
		let jsonrpc_endpoint = newArcMutex(jsonrpc_endpoint);
		
		let ls_client = EndpointLSClient { jsonrpc_endpoint : jsonrpc_endpoint.clone() };
		// FIXME: todo LanguageServerEndpoint + LS
		ls.connect(ls_client);
		
		let result = jsonrpc::run_message_read_loop(jsonrpc_endpoint, LSPMessageReader(input));
		
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


pub trait LanguageClient {
	
	fn showMessage(&self, params: ShowMessageParams);
	fn showMessageRequest(&self, params: ShowMessageRequestParams) -> LSResult<MessageActionItem, ()>;
	fn logMessage(&self, params: LogMessageParams);
	fn telemetryEvent(&self, params: any);
	
	fn publishDiagnostics(&self, params: PublishDiagnosticsParams);

}



impl RequestHandler for LanguageServer {
	
	fn handle_request(&mut self, method_name: &str, params: RequestParams, 
		completable: ResponseCompletable) 
	{
		match method_name {
			lsp::Request__Initialize => { completable.sync_handle_request(params, 
				|params| self.initialize(params)) 
			}
			lsp::Request__Shutdown => { completable.sync_handle_request(params, 
				|params| self.shutdown(params)) 
			}
			lsp::Notification__Exit => { completable.sync_handle_notification(params, 
				|params| self.exit(params)) 
			}
			lsp::Notification__WorkspaceChangeConfiguration => { completable.sync_handle_notification(params, 
				|params| self.workspaceChangeConfiguration(params)) 
			}
			lsp::Notification__DidOpenTextDocument => { completable.sync_handle_notification(params, 
				|params| self.didOpenTextDocument(params)) 
			}
			lsp::Notification__DidChangeTextDocument => { completable.sync_handle_notification(params, 
				|params| self.didChangeTextDocument(params)) 
			}
			lsp::Notification__DidCloseTextDocument => { completable.sync_handle_notification(params, 
				|params| self.didCloseTextDocument(params)) 
			}
			lsp::Notification__DidSaveTextDocument => { completable.sync_handle_notification(params, 
				|params| self.didSaveTextDocument(params)) 
			}
			lsp::Notification__DidChangeWatchedFiles => { completable.sync_handle_notification(params, 
				|params| self.didChangeWatchedFiles(params)) 
			}
			lsp::Request__Completion => { completable.sync_handle_request(params, 
				|params| self.completion(params)) 
			}
			lsp::Request__ResolveCompletionItem => { completable.sync_handle_request(params, 
				|params| self.resolveCompletionItem(params)) 
			}
			lsp::Request__Hover => { completable.sync_handle_request(params, 
				|params| self.hover(params)) 
			}
			lsp::Request__SignatureHelp => { completable.sync_handle_request(params, 
				|params| self.signatureHelp(params)) 
			}
			lsp::Request__GotoDefinition => { completable.sync_handle_request(params, 
				|params| self.gotoDefinition(params)) 
			}
			lsp::Request__References => { completable.sync_handle_request(params, 
				|params| self.references(params)) 
			}
			lsp::Request__DocumentHighlight => { completable.sync_handle_request(params, 
				|params| self.documentHighlight(params)) 
			}
			lsp::Request__DocumentSymbols => { completable.sync_handle_request(params, 
				|params| self.documentSymbols(params)) 
			}
			lsp::Request__WorkspaceSymbols => { completable.sync_handle_request(params, 
				|params| self.workspaceSymbols(params)) 
			}
			lsp::Request__CodeAction => { completable.sync_handle_request(params, 
				|params| self.codeAction(params)) 
			}
			lsp::Request__CodeLens => { completable.sync_handle_request(params, 
				|params| self.codeLens(params)) 
			}
			lsp::Request__CodeLensResolve => { completable.sync_handle_request(params, 
				|params| self.codeLensResolve(params)) 
			}
			lsp::Request__Formatting => { completable.sync_handle_request(params, 
				|params| self.formatting(params)) 
			}
			lsp::Request__RangeFormatting => { completable.sync_handle_request(params, 
				|params| self.rangeFormatting(params)) 
			}
			lsp::Request__OnTypeFormatting => { completable.sync_handle_request(params, 
				|params| self.onTypeFormatting(params)) 
			}
			lsp::Request__Rename => { completable.sync_handle_request(params, 
				|params| self.rename(params)) 
			}
			_ => {
				completable.complete_with_error(jsonrpc_objects::error_JSON_RPC_MethodNotFound());
			}
		};
		
	}
	
}


pub trait LanguageServerEndpoint {
	fn connect(&self, client_endpoint: EndpointLSClient);	
}

impl LanguageServerEndpoint for LanguageServer {
	fn connect(&self, client_endpoint: EndpointLSClient) {
		// FIXME: todo
	}
}

pub struct EndpointLSClient {
	jsonrpc_endpoint: Arc<Mutex<Endpoint>>,
}

impl LanguageClient for EndpointLSClient {
	
    fn showMessage(&self, params: ShowMessageParams) {
    	let mut jsonrpc_endpoint = self.jsonrpc_endpoint.lock().unwrap();
    	jsonrpc_endpoint.send_notification(lsp::Notification__ShowMessage, params);
    }
    
    fn showMessageRequest(&self, _params: ShowMessageRequestParams) -> LSResult<MessageActionItem, ()> {
    	let jsonrpc_endpoint = self.jsonrpc_endpoint.lock().unwrap();
//    	jsonrpc_endpoint.send_request(lsp::Notification__ShowMessageRequest, params);
    	panic!("not implemented")
    }
    
    fn logMessage(&self, params: LogMessageParams) {
    	let mut jsonrpc_endpoint = self.jsonrpc_endpoint.lock().unwrap();
    	jsonrpc_endpoint.send_notification(lsp::Notification__LogMessage, params);
    }
    
    fn telemetryEvent(&self, params: any) {
    	let mut jsonrpc_endpoint = self.jsonrpc_endpoint.lock().unwrap();
    	jsonrpc_endpoint.send_notification(lsp::Notification__TelemetryEvent, params);
    }
    
    fn publishDiagnostics(&self, params: PublishDiagnosticsParams) {
    	let mut jsonrpc_endpoint = self.jsonrpc_endpoint.lock().unwrap();
    	jsonrpc_endpoint.send_notification(lsp::Notification__PublishDiagnostics, params);
    }
	
}