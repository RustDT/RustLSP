// Copyright 2016 Bruno Medeiros
//
// Licensed under the Apache License, Version 2.0 
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>. 
// This file may not be copied, modified, or distributed
// except according to those terms.


// WARNING: Rust newbie code ahead (-_-)'

#![allow(non_camel_case_types)]

extern crate serde_json;

use std::io::{self, Read, Write};

use util::core::*;
use util::service::Provider;

use json_rpc;
use json_rpc::*;

use lsp;
use lsp::*;

use std::rc::Rc;

/* -----------------  ----------------- */

pub struct LSPServer<'a> {
	
	pub ls: Rc<LanguageServer>,
	pub rpc_dispatcher : JsonRpcDispatcher<'a>,
	
}

impl<'a> LSPServer<'a> {
	
	pub fn start_new(ls: Rc<LanguageServer>, input: &mut io::BufRead, output : &mut io::Write) {
		let mut server = LSPServer { ls : ls, rpc_dispatcher : JsonRpcDispatcher::new(output), };
		
		initialize_methods(&mut server);
		
		let result = server.read_incoming_messages(LSPMessageProvider(input));
		match result {
			Err(error) => { 
				writeln!(&mut io::stderr(), "Error reading/writing the connection streams: {}", error)
					.expect("Failed writing to stderr");
			}
			Ok(_) => { } 
		}
	}
	
}

struct LSPMessageProvider<'a>(&'a mut io::BufRead);

impl<'a> Provider<String> for LSPMessageProvider<'a> {
	fn obtain_next(&mut self) -> GResult<String> {
		parse_transport_message::<&mut _>(self.0)
	}
}

pub fn initialize_methods(lsp_handler : &mut LSPServer) {
	let ls = &lsp_handler.ls;
	
	lsp_handler.rpc_dispatcher.add_request(lsp::request__Initialize(ls.clone()));
	lsp_handler.rpc_dispatcher.add_request(lsp::request__Shutdown(ls.clone()));
	
	lsp_handler.rpc_dispatcher.add_notification(lsp::notification__Exit(ls.clone()));
	
	lsp_handler.rpc_dispatcher.add_notification(lsp::notification__ShowMessage(ls.clone()));
	lsp_handler.rpc_dispatcher.add_notification(lsp::notification__ShowMessageRequest(ls.clone()));
	lsp_handler.rpc_dispatcher.add_notification(lsp::notification__LogMessage(ls.clone()));
	lsp_handler.rpc_dispatcher.add_notification(lsp::notification__TelemetryEvent(ls.clone()));
	lsp_handler.rpc_dispatcher.add_notification(lsp::notification__WorkspaceChangeConfiguration(ls.clone()));
	lsp_handler.rpc_dispatcher.add_notification(lsp::notification__DidOpenTextDocument(ls.clone()));
	lsp_handler.rpc_dispatcher.add_notification(lsp::notification__DidChangeTextDocument(ls.clone()));
	lsp_handler.rpc_dispatcher.add_notification(lsp::notification__DidCloseTextDocument(ls.clone()));
	lsp_handler.rpc_dispatcher.add_notification(lsp::notification__DidSaveTextDocument(ls.clone()));
	lsp_handler.rpc_dispatcher.add_notification(lsp::notification__DidChangeWatchedFiles(ls.clone()));
	lsp_handler.rpc_dispatcher.add_notification(lsp::notification__PublishDiagnostics(ls.clone()));
	
	lsp_handler.rpc_dispatcher.add_request(lsp::request__Completion(ls.clone()));
	lsp_handler.rpc_dispatcher.add_request(lsp::request__ResolveCompletionItem(ls.clone()));
	lsp_handler.rpc_dispatcher.add_request(lsp::request__Hover(ls.clone()));
	lsp_handler.rpc_dispatcher.add_request(lsp::request__SignatureHelp(ls.clone()));
	lsp_handler.rpc_dispatcher.add_request(lsp::request__GotoDefinition(ls.clone()));
	lsp_handler.rpc_dispatcher.add_request(lsp::request__References(ls.clone()));
	lsp_handler.rpc_dispatcher.add_request(lsp::request__DocumentHighlight(ls.clone()));
	lsp_handler.rpc_dispatcher.add_request(lsp::request__DocumentSymbols(ls.clone()));
	lsp_handler.rpc_dispatcher.add_request(lsp::request__WorkspaceSymbols(ls.clone()));
	lsp_handler.rpc_dispatcher.add_request(lsp::request__CodeAction(ls.clone()));
	lsp_handler.rpc_dispatcher.add_request(lsp::request__CodeLens(ls.clone()));
	lsp_handler.rpc_dispatcher.add_request(lsp::request__CodeLensResolve(ls.clone()));
	lsp_handler.rpc_dispatcher.add_request(lsp::request__Formatting(ls.clone()));
	lsp_handler.rpc_dispatcher.add_request(lsp::request__RangeFormatting(ls.clone()));
	lsp_handler.rpc_dispatcher.add_request(lsp::request__OnTypeFormatting(ls.clone()));
	lsp_handler.rpc_dispatcher.add_request(lsp::request__Rename(ls.clone()));
	
}

/* ----------------- Parse content-length ----------------- */

const CONTENT_LENGTH: &'static str = "Content-Length:";
	
pub fn parse_transport_message<R : io::BufRead>(mut reader: R) -> GResult<String>
{
	
	let mut content_length : u32 = 0; 
	
	loop {
		let mut line : String = String::new();
		
		try!(reader.read_line(&mut line));
		
		if line.starts_with(CONTENT_LENGTH) {
			let len_str : &str = &line[CONTENT_LENGTH.len()..]; 
			let my_int = len_str.trim().parse::<u32>();
			
			content_length = try!(my_int);
			
		} else if line.eq("\r\n") {
			break;
		}
	}
	if content_length == 0 {
		return Err(ErrorMessage::create(String::from(CONTENT_LENGTH) + " not defined or invalid."));
	}
	
	let mut message_reader = reader.take(content_length as u64);
	let mut message = String::new();
	try!(message_reader.read_to_string(&mut message));
	return Ok(message);
}
