// Copyright 2016 Bruno Medeiros
//
// Licensed under the Apache License, Version 2.0 
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>. 
// This file may not be copied, modified, or distributed
// except according to those terms.


#![allow(non_camel_case_types)]

use std::io::{self, Write};

use util::core::*;
use json_rpc::service_util::Provider;
use json_rpc::service_util::Handler;

use json_rpc::*;

use lsp;
use lsp_transport;
use lsp::*;

use std::rc::Rc;

/* -----------------  ----------------- */

struct LSPMessageProvider<'a>(&'a mut io::BufRead);

impl<'a> Provider<String, GError> for LSPMessageProvider<'a> {
	fn obtain_next(&mut self) -> GResult<String> {
		lsp_transport::parse_transport_message(&mut self.0)
	}
}

struct LSPMessageWriter<T: io::Write>(pub T);

impl<T: io::Write> Handler<String, GError> for LSPMessageWriter<T> {
	fn supply(&mut self, msg: &str) -> Result<(), GError> {
		lsp_transport::write_transport_message(msg, &mut self.0)
	}
}


pub struct LSPServer {
	
	pub ls: Rc<LanguageServer>,
	pub json_rpc : JsonRpcEndpoint,
	
}

impl LSPServer {
	
	pub fn start_new<OUT, OUT_P>(
		ls: Rc<LanguageServer>, input: &mut io::BufRead, out_stream_provider: OUT_P
	) 
	where 
		OUT: io::Write + 'static, 
		OUT_P : FnOnce() -> OUT + Send + 'static 
	{
		let mut handler = new(MapRpcRequestHandler::new());
		initialize_methods(ls.clone(), &mut handler);
		
		let jsonrpc_endpoint = JsonRpcEndpoint::start_with_provider(|| {
			LSPMessageWriter(out_stream_provider())
		}, handler);
		Self::start_with_endpoint(ls, input, jsonrpc_endpoint)
	}
	
	pub fn start_with_endpoint(
		ls: Rc<LanguageServer>, input: &mut io::BufRead, jsonrpc_endpoint: JsonRpcEndpoint
	) {
		let mut server = LSPServer { ls : ls, json_rpc : jsonrpc_endpoint };
		
		let result = server.json_rpc.run_message_read_loop(LSPMessageProvider(input));
		match result {
			Err(error) => { 
				writeln!(&mut io::stderr(), "Error handling incoming the connection streams: {}", error)
					.expect("Failed writing to stderr");
			}
			Ok(_) => { } 
		}
	}
	
}

pub fn initialize_methods(ls: Rc<LanguageServer>, handler: &mut MapRpcRequestHandler) {
	
	let (name, mh) = lsp::request__Initialize(ls.clone()); handler.add_request(name, mh);
	let (name, mh) = lsp::request__Shutdown(ls.clone()); handler.add_request(name, mh);
	
	let (name, mh) = lsp::notification__Exit(ls.clone()); handler.add_notification(name, mh);
	
	let (name, mh) = lsp::notification__ShowMessage(ls.clone()); handler.add_notification(name, mh);
	let (name, mh) = lsp::request__ShowMessageRequest(ls.clone()); handler.add_request(name, mh);
	let (name, mh) = lsp::notification__LogMessage(ls.clone()); handler.add_notification(name, mh);
	let (name, mh) = lsp::notification__TelemetryEvent(ls.clone()); handler.add_notification(name, mh);
	let (name, mh) = lsp::notification__WorkspaceChangeConfiguration(ls.clone()); handler.add_notification(name, mh);
	let (name, mh) = lsp::notification__DidOpenTextDocument(ls.clone()); handler.add_notification(name, mh);
	let (name, mh) = lsp::notification__DidChangeTextDocument(ls.clone()); handler.add_notification(name, mh);
	let (name, mh) = lsp::notification__DidCloseTextDocument(ls.clone()); handler.add_notification(name, mh);
	let (name, mh) = lsp::notification__DidSaveTextDocument(ls.clone()); handler.add_notification(name, mh);
	let (name, mh) = lsp::notification__DidChangeWatchedFiles(ls.clone()); handler.add_notification(name, mh);
	let (name, mh) = lsp::notification__PublishDiagnostics(ls.clone()); handler.add_notification(name, mh);
	
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
