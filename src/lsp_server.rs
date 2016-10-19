// Copyright 2016 Bruno Medeiros
//
// Licensed under the Apache License, Version 2.0 
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>. 
// This file may not be copied, modified, or distributed
// except according to those terms.


#![allow(non_camel_case_types)]

use std::io::{self, Write};

use util::core::*;

use json_rpc;
use json_rpc::*;
use json_rpc::service_util::Provider;
use json_rpc::service_util::MessageWriter;
use json_rpc::output_agent::OutputAgent;

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
		let mut jsonrpc_endpoint = JsonRpcEndpoint::start_with_output_agent(output_agent, new(MapRpcRequestHandler::new()));
		
		let mut request_handler = new(MapRpcRequestHandler::new());
		initialize_methods(ls.clone(), &mut request_handler);
		jsonrpc_endpoint.request_handler = request_handler;
		
		let jsonrpc_endpoint = newArcMutex(jsonrpc_endpoint);
		
		let ls_client = EndpointLSClient { jsonrpc_endpoint : jsonrpc_endpoint.clone() };
		// FIXME: todo LanguageServerEndpoint + LS
		ls.connect(ls_client);
		
		let result = json_rpc::run_message_read_loop(jsonrpc_endpoint, LSPMessageReader(input));
		
		if let Err(error) = result {
			error!("Error handling the incoming stream: {}", error);
		}
	}
	
}

pub fn initialize_methods(ls: Rc<LanguageServer>, handler: &mut MapRpcRequestHandler) {
	
	let (name, mh) = lsp::request__Initialize(ls.clone()); handler.add_request(name, mh);
	let (name, mh) = lsp::request__Shutdown(ls.clone()); handler.add_request(name, mh);
	
	let (name, mh) = lsp::notification__Exit(ls.clone()); handler.add_notification(name, mh);
	
	let (name, mh) = lsp::notification__WorkspaceChangeConfiguration(ls.clone()); handler.add_notification(name, mh);
	let (name, mh) = lsp::notification__DidOpenTextDocument(ls.clone()); handler.add_notification(name, mh);
	let (name, mh) = lsp::notification__DidChangeTextDocument(ls.clone()); handler.add_notification(name, mh);
	let (name, mh) = lsp::notification__DidCloseTextDocument(ls.clone()); handler.add_notification(name, mh);
	let (name, mh) = lsp::notification__DidSaveTextDocument(ls.clone()); handler.add_notification(name, mh);
	let (name, mh) = lsp::notification__DidChangeWatchedFiles(ls.clone()); handler.add_notification(name, mh);
	
	let (name, mh) = lsp::request__Completion(ls.clone()); handler.add_request(name, mh);
	let (name, mh) = lsp::request__ResolveCompletionItem(ls.clone()); handler.add_request(name, mh);
	let (name, mh) = lsp::request__Hover(ls.clone()); handler.add_request(name, mh);
	let (name, mh) = lsp::request__SignatureHelp(ls.clone()); handler.add_request(name, mh);
	let (name, mh) = lsp::request__GotoDefinition(ls.clone()); handler.add_request(name, mh);
	let (name, mh) = lsp::request__References(ls.clone()); handler.add_request(name, mh);
	let (name, mh) = lsp::request__DocumentHighlight(ls.clone()); handler.add_request(name, mh);
	let (name, mh) = lsp::request__DocumentSymbols(ls.clone()); handler.add_request(name, mh);
	let (name, mh) = lsp::request__WorkspaceSymbols(ls.clone()); handler.add_request(name, mh);
	let (name, mh) = lsp::request__CodeAction(ls.clone()); handler.add_request(name, mh);
	let (name, mh) = lsp::request__CodeLens(ls.clone()); handler.add_request(name, mh);
	let (name, mh) = lsp::request__CodeLensResolve(ls.clone()); handler.add_request(name, mh);
	let (name, mh) = lsp::request__Formatting(ls.clone()); handler.add_request(name, mh);
	let (name, mh) = lsp::request__RangeFormatting(ls.clone()); handler.add_request(name, mh);
	let (name, mh) = lsp::request__OnTypeFormatting(ls.clone()); handler.add_request(name, mh);
	let (name, mh) = lsp::request__Rename(ls.clone()); handler.add_request(name, mh);
	
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
	jsonrpc_endpoint: Arc<Mutex<JsonRpcEndpoint>>,
}

impl LanguageClient for EndpointLSClient {
	
    fn showMessage(&self, _params: ShowMessageParams) {
//    	let (name, mh) = lsp::notification__ShowMessage(ls);
    	panic!("not implemented")
    }
    fn showMessageRequest(&self, _params: ShowMessageRequestParams) -> LSResult<MessageActionItem, ()> {
    	panic!("not implemented")
    }
    fn logMessage(&self, _params: LogMessageParams) {
    	panic!("not implemented")
    }
    fn telemetryEvent(&self, _params: any) {
    	panic!("not implemented")
    }
    fn publishDiagnostics(&self, _params: PublishDiagnosticsParams) {
    	panic!("not implemented")
    }
	
}