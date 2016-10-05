// Copyright 2016 Bruno Medeiros
//
// Licensed under the Apache License, Version 2.0 
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>. 
// This file may not be copied, modified, or distributed
// except according to those terms.


#![allow(non_camel_case_types)]

use std::io::{self, Read, Write};

use util::core::*;
use json_rpc::service_util::Provider;

use json_rpc::*;

use lsp;
use lsp_transport;
use lsp::*;

use std::rc::Rc;

/* -----------------  ----------------- */

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
		let jsonrpc_endpoint = JsonRpcEndpoint::start_with_provider(|| {
			output_agent::IoWriteHandler(out_stream_provider())
		});
		Self::start_with_endpoint(ls, input, jsonrpc_endpoint)
	}
	
	pub fn start_with_endpoint(
		ls: Rc<LanguageServer>, input: &mut io::BufRead, jsonrpc_endpoint: JsonRpcEndpoint
	) {
		let mut server = LSPServer { ls : ls, json_rpc : jsonrpc_endpoint };
		
		initialize_methods(&mut server);
		
		let result = server.json_rpc.read_incoming_messages(LSPMessageProvider(input));
		match result {
			Err(error) => { 
				writeln!(&mut io::stderr(), "Error handling incoming the connection streams: {}", error)
					.expect("Failed writing to stderr");
			}
			Ok(_) => { } 
		}
	}
	
}

struct LSPMessageProvider<'a>(&'a mut io::BufRead);

impl<'a> Provider<String, GError> for LSPMessageProvider<'a> {
	fn obtain_next(&mut self) -> GResult<String> {
		lsp_transport::parse_transport_message::<&mut _>(self.0)
	}
}

pub fn initialize_methods(lsp_handler : &mut LSPServer) {
	let ls = &lsp_handler.ls;
	
	add_handler(&mut lsp_handler.json_rpc, lsp::request__Initialize(ls.clone()));
	add_handler(&mut lsp_handler.json_rpc, lsp::request__Shutdown(ls.clone()));
	
	add_handler(&mut lsp_handler.json_rpc, lsp::notification__Exit(ls.clone()));
	
	add_handler(&mut lsp_handler.json_rpc, lsp::notification__ShowMessage(ls.clone()));
	add_handler(&mut lsp_handler.json_rpc, lsp::notification__ShowMessageRequest(ls.clone()));
	add_handler(&mut lsp_handler.json_rpc, lsp::notification__LogMessage(ls.clone()));
	add_handler(&mut lsp_handler.json_rpc, lsp::notification__TelemetryEvent(ls.clone()));
	add_handler(&mut lsp_handler.json_rpc, lsp::notification__WorkspaceChangeConfiguration(ls.clone()));
	add_handler(&mut lsp_handler.json_rpc, lsp::notification__DidOpenTextDocument(ls.clone()));
	add_handler(&mut lsp_handler.json_rpc, lsp::notification__DidChangeTextDocument(ls.clone()));
	add_handler(&mut lsp_handler.json_rpc, lsp::notification__DidCloseTextDocument(ls.clone()));
	add_handler(&mut lsp_handler.json_rpc, lsp::notification__DidSaveTextDocument(ls.clone()));
	add_handler(&mut lsp_handler.json_rpc, lsp::notification__DidChangeWatchedFiles(ls.clone()));
	add_handler(&mut lsp_handler.json_rpc, lsp::notification__PublishDiagnostics(ls.clone()));
	
	add_handler(&mut lsp_handler.json_rpc, lsp::request__Completion(ls.clone()));
	add_handler(&mut lsp_handler.json_rpc, lsp::request__ResolveCompletionItem(ls.clone()));
	add_handler(&mut lsp_handler.json_rpc, lsp::request__Hover(ls.clone()));
	add_handler(&mut lsp_handler.json_rpc, lsp::request__SignatureHelp(ls.clone()));
	add_handler(&mut lsp_handler.json_rpc, lsp::request__GotoDefinition(ls.clone()));
	add_handler(&mut lsp_handler.json_rpc, lsp::request__References(ls.clone()));
	add_handler(&mut lsp_handler.json_rpc, lsp::request__DocumentHighlight(ls.clone()));
	add_handler(&mut lsp_handler.json_rpc, lsp::request__DocumentSymbols(ls.clone()));
	add_handler(&mut lsp_handler.json_rpc, lsp::request__WorkspaceSymbols(ls.clone()));
	add_handler(&mut lsp_handler.json_rpc, lsp::request__CodeAction(ls.clone()));
	add_handler(&mut lsp_handler.json_rpc, lsp::request__CodeLens(ls.clone()));
	add_handler(&mut lsp_handler.json_rpc, lsp::request__CodeLensResolve(ls.clone()));
	add_handler(&mut lsp_handler.json_rpc, lsp::request__Formatting(ls.clone()));
	add_handler(&mut lsp_handler.json_rpc, lsp::request__RangeFormatting(ls.clone()));
	add_handler(&mut lsp_handler.json_rpc, lsp::request__OnTypeFormatting(ls.clone()));
	add_handler(&mut lsp_handler.json_rpc, lsp::request__Rename(ls.clone()));
	
}

pub fn add_handler<REQUEST_HANDLER : HandleRpcRequest + 'static>(
	endpoint : &mut JsonRpcEndpoint,
	method: (&'static str, REQUEST_HANDLER)
){
	endpoint.add_rpc_handler(method.0, method.1);
}