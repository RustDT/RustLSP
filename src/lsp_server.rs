// Copyright 2016 Bruno Medeiros
//
// Licensed under the Apache License, Version 2.0 
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>. 
// This file may not be copied, modified, or distributed
// except according to those terms.


use std::io;

use util::core::*;

use jsonrpc::*;
use jsonrpc::service_util::MessageReader;
use jsonrpc::service_util::MessageWriter;

use jsonrpc::output_agent::OutputAgent;

use jsonrpc::method_types::MethodError;
use jsonrpc::jsonrpc_request::RequestParams;

use lsp_transport;
use ls_types::*;
use serde_json::Value;

/* -----------------  ----------------- */

pub struct LSPMessageReader<T : io::BufRead>(pub T);

impl<T : io::BufRead> MessageReader for LSPMessageReader<T> {
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
    
    /// Create an EndpointOutput output for use in the Language Server Protocol,
    /// with given output stream provider.
    pub fn create_lsp_output_with_output_stream<OUT, OUT_PROV>(output_stream_provider: OUT_PROV) 
        -> EndpointOutput
    where 
        OUT : io::Write + 'static, 
        OUT_PROV : FnOnce() -> OUT + Send + 'static
    {
        Self::create_lsp_output(|| {
            LSPMessageWriter(output_stream_provider())
        })
    }
    
    /// Create an EndpointOutput output for use in the Language Server Protocol
    /// with given message write provider.
    pub fn create_lsp_output<MW, MW_PROV>(msg_writer_provider: MW_PROV) 
        -> EndpointOutput
    where 
        MW : MessageWriter + 'static, 
        MW_PROV : FnOnce() -> MW + Send + 'static 
    {
        let output_agent = OutputAgent::start_with_provider(msg_writer_provider);
        EndpointOutput::start_with(output_agent)
    }
    
    /* -----------------  ----------------- */
    
    pub fn run_server_from_input<LS>(ls: LS, input: &mut io::BufRead, endpoint_out: EndpointOutput) 
    where 
        LS : LanguageServer + 'static,
    {
        Self::run_server(&mut LSPMessageReader(input), endpoint_out, ls)
    }
    
    /// Run the message read loop on the server, for given msg_reader.
    /// msg_reader must be a LSPMessageReader or compatible.
    pub fn run_server<LS, MR>(
        mut msg_reader: &mut MR, endpoint_out: EndpointOutput, ls: LS
    ) 
    where 
        LS : LanguageServer + 'static,
        MR : MessageReader,
    {
        Self::run_endpoint_loop(msg_reader, endpoint_out, new(LSRequestHandler(ls)))
    }
    
    pub fn run_endpoint_loop<MR>(
        mut msg_reader: &mut MR, endpoint_out: EndpointOutput, request_handler: Box<RequestHandler>
    ) 
    where 
        MR : MessageReader,
    {
        info!("Starting LSP server");
        
        let endpoint = EndpointHandler::create(endpoint_out, request_handler);
        
        let result = endpoint.run_message_read_loop(msg_reader);
        
        if let Err(error) = result {
            error!("Error handling the incoming stream: {}", error);
        }
    }
    
}

pub type LSResult<RET, ERR_DATA> = Result<RET, MethodError<ERR_DATA>>;
pub type LSCompletable<RET> = MethodCompletable<RET, ()>;

pub trait LanguageServer {
    
    fn initialize(&mut self, params: InitializeParams, completable: MethodCompletable<InitializeResult, InitializeError>);
    fn shutdown(&mut self, params: (), completable: LSCompletable<()>);
    fn exit(&mut self, params: ());
    fn workspace_change_configuration(&mut self, params: DidChangeConfigurationParams);
    fn did_open_text_document(&mut self, params: DidOpenTextDocumentParams);
    fn did_change_text_document(&mut self, params: DidChangeTextDocumentParams);
    fn did_close_text_document(&mut self, params: DidCloseTextDocumentParams);
    fn did_save_text_document(&mut self, params: DidSaveTextDocumentParams);
    fn did_change_watched_files(&mut self, params: DidChangeWatchedFilesParams);
    
    fn completion(&mut self, params: TextDocumentPositionParams, completable: LSCompletable<CompletionList>);
    fn resolve_completion_item(&mut self, params: CompletionItem, completable: LSCompletable<CompletionItem>);
    fn hover(&mut self, params: TextDocumentPositionParams, completable: LSCompletable<Hover>);
    fn signature_help(&mut self, params: TextDocumentPositionParams, completable: LSCompletable<SignatureHelp>);
    fn goto_definition(&mut self, params: TextDocumentPositionParams, completable: LSCompletable<Vec<Location>>);
    fn references(&mut self, params: ReferenceParams, completable: LSCompletable<Vec<Location>>);
    fn document_highlight(&mut self, params: TextDocumentPositionParams, completable: LSCompletable<Vec<DocumentHighlight>>);
    fn document_symbols(&mut self, params: DocumentSymbolParams, completable: LSCompletable<Vec<SymbolInformation>>);
    fn workspace_symbols(&mut self, params: WorkspaceSymbolParams, completable: LSCompletable<Vec<SymbolInformation>>);
    fn code_action(&mut self, params: CodeActionParams, completable: LSCompletable<Vec<Command>>);
    fn code_lens(&mut self, params: CodeLensParams, completable: LSCompletable<Vec<CodeLens>>);
    fn code_lens_resolve(&mut self, params: CodeLens, completable: LSCompletable<CodeLens>);
    fn formatting(&mut self, params: DocumentFormattingParams, completable: LSCompletable<Vec<TextEdit>>);
    fn range_formatting(&mut self, params: DocumentRangeFormattingParams, completable: LSCompletable<Vec<TextEdit>>);
    fn on_type_formatting(&mut self, params: DocumentOnTypeFormattingParams, completable: LSCompletable<Vec<TextEdit>>);
    fn rename(&mut self, params: RenameParams, completable: LSCompletable<WorkspaceEdit>);
    
    #[allow(unused_variables)]
    fn handle_other_method(&mut self, method_name: &str, params: RequestParams, completable: ResponseCompletable) {
        completable.complete_with_error(jsonrpc_common::error_JSON_RPC_MethodNotFound()); 
    }
}


pub trait LanguageClientEndpoint {
    
    fn show_message(&mut self, params: ShowMessageParams) -> GResult<()>;
    fn show_message_request(&mut self, params: ShowMessageRequestParams) -> GResult<LSResult<MessageActionItem, ()>>;
    fn log_message(&mut self, params: LogMessageParams) -> GResult<()>;
    fn telemetry_event(&mut self, params: Value) -> GResult<()>;
    
    fn publish_diagnostics(&mut self, params: PublishDiagnosticsParams) -> GResult<()>;

}

pub struct LSRequestHandler<LS : ?Sized>(pub LS);

impl<LS : LanguageServer + ?Sized> RequestHandler for LSRequestHandler<LS> {
    
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
                self.0.handle_other_method(method_name, params, completable);
            }
        };
        
    }
    
}

impl LanguageClientEndpoint for EndpointOutput {
    
    fn show_message(&mut self, params: ShowMessageParams) -> GResult<()> {
        let endpoint = self;
        try!(endpoint.send_notification(NOTIFICATION__ShowMessage, params));
        Ok(())
    }
    
    fn show_message_request(&mut self, _params: ShowMessageRequestParams) -> GResult<LSResult<MessageActionItem, ()>> {
        let endpoint = self;
//        endpoint.send_request(NOTIFICATION__ShowMessageRequest, params);
        panic!("not implemented")
    }
    
    fn log_message(&mut self, params: LogMessageParams) -> GResult<()> {
        let endpoint = self;
        try!(endpoint.send_notification(NOTIFICATION__LogMessage, params));
        Ok(())
    }
    
    fn telemetry_event(&mut self, params: Value) -> GResult<()> {
        let endpoint = self;
        try!(endpoint.send_notification(NOTIFICATION__TelemetryEvent, params));
        Ok(())
    }
    
    fn publish_diagnostics(&mut self, params: PublishDiagnosticsParams) -> GResult<()> {
        let endpoint = self;
        try!(endpoint.send_notification(NOTIFICATION__PublishDiagnostics, params));
        Ok(())
    }
    
}

