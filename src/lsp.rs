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

use lsp_transport::LSPMessageWriter;
use lsp_transport::LSPMessageReader;
use ls_types::*;
use serde_json::Value;

/* -----------------  ----------------- */

/// Helper empty type to help create a JSON-RPC endpoint for LSP communication
pub struct LSPEndpoint {
    
}

impl LSPEndpoint {
    
    /// Create an Endpoint for use in the Language Server Protocol,
    /// with given output stream provider.
    pub fn create_lsp_output_with_output_stream<OUT, OUT_PROV>(output_stream_provider: OUT_PROV) 
        -> Endpoint
    where 
        OUT : io::Write + 'static, 
        OUT_PROV : FnOnce() -> OUT + Send + 'static
    {
        Self::create_lsp_output(|| {
            LSPMessageWriter(output_stream_provider())
        })
    }
    
    /// Create an Endpoint for use in the Language Server Protocol
    /// with given message writer provider.
    pub fn create_lsp_output<MW, MW_PROV>(msg_writer_provider: MW_PROV) 
        -> Endpoint
    where 
        MW : MessageWriter + 'static, 
        MW_PROV : FnOnce() -> MW + Send + 'static 
    {
        let output_agent = OutputAgent::start_with_provider(msg_writer_provider);
        Endpoint::start_with(output_agent)
    }
    
    /* -----------------  ----------------- */
    
    pub fn run_server_from_input<SERVER>(
        input: &mut io::BufRead, endpoint: Endpoint, lsp_server_handler: SERVER, 
    ) 
    where 
        SERVER : LanguageServerHandling + 'static,
    {
        Self::run_server(&mut LSPMessageReader(input), endpoint, lsp_server_handler)
    }
    
    /// Run the message read loop on the server, for given msg_reader.
    /// msg_reader must be a LSPMessageReader or compatible.
    pub fn run_server<SERVER, MR>(
        mut msg_reader: &mut MR, endpoint: Endpoint, lsp_server_handler: SERVER
    ) 
    where 
        SERVER : LanguageServerHandling + 'static,
        MR : MessageReader,
    {
        Self::run_endpoint_loop(msg_reader, endpoint, new(ServerRequestHandler(lsp_server_handler)))
    }
    
    pub fn run_client_from_input<CLIENT>(
        input: &mut io::BufRead, endpoint: Endpoint, lsp_client_handler: CLIENT,
    ) 
    where 
        CLIENT : LanguageClientHandling + 'static,
    {
        let cl_handler = new(ClientRequestHandler(lsp_client_handler));
        Self::run_endpoint_loop(&mut LSPMessageReader(input), endpoint, cl_handler)
    }
    
    pub fn run_endpoint_loop<MR>(
        mut msg_reader: &mut MR, endpoint: Endpoint, request_handler: Box<RequestHandler>
    ) 
    where 
        MR : MessageReader,
    {
        info!("Starting LSP Endpoint");
        
        let endpoint = EndpointHandler::create(endpoint, request_handler);
        
        let result = endpoint.run_message_read_loop(msg_reader);
        
        if let Err(error) = result {
            error!("Error handling the incoming stream: {}", error);
        }
    }
    
}

pub type LSResult<RET, ERR_DATA> = Result<RET, MethodError<ERR_DATA>>;
pub type LSCompletable<RET> = MethodCompletable<RET, ()>;

/// Trait for the handling of LSP server requests
pub trait LanguageServerHandling {
    
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


pub struct ServerRequestHandler<LS : ?Sized>(pub LS);

impl<LS : LanguageServerHandling + ?Sized> RequestHandler for ServerRequestHandler<LS> {
    
    fn handle_request(
        &mut self, method_name: &str, params: RequestParams, completable: ResponseCompletable
    ) {
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


pub trait LspClientRpc {
    
    fn show_message(&mut self, params: ShowMessageParams) 
        -> GResult<()>;
    
    fn show_message_request(&mut self, params: ShowMessageRequestParams) 
        -> GResult<RequestFuture<MessageActionItem, ()>>;
    
    fn log_message(&mut self, params: LogMessageParams) 
        -> GResult<()>;
    
    fn telemetry_event(&mut self, params: Value) 
        -> GResult<()>;
    
    fn publish_diagnostics(&mut self, params: PublishDiagnosticsParams) 
        -> GResult<()>;

}

pub struct LSPServerEndpoint<'a> {
    pub endpoint: &'a mut Endpoint,    
}


impl<'a> LspClientRpc for LSPServerEndpoint<'a> {
    
    fn show_message(&mut self, params: ShowMessageParams) 
        -> GResult<()> 
    {
        self.endpoint.send_notification(NOTIFICATION__ShowMessage, params)
    }
    
    fn show_message_request(&mut self, params: ShowMessageRequestParams) 
        -> GResult<RequestFuture<MessageActionItem, ()>> 
    {
        self.endpoint.send_request(REQUEST__ShowMessageRequest, params)
    }
    
    fn log_message(&mut self, params: LogMessageParams) 
        -> GResult<()> 
    {
        self.endpoint.send_notification(NOTIFICATION__LogMessage, params)
    }
    
    fn telemetry_event(&mut self, params: Value) 
        -> GResult<()> 
    {
        self.endpoint.send_notification(NOTIFICATION__TelemetryEvent, params)
    }
    
    fn publish_diagnostics(&mut self, params: PublishDiagnosticsParams) 
        -> GResult<()> 
    {
        self.endpoint.send_notification(NOTIFICATION__PublishDiagnostics, params)
    }
    
}

/* ----------------- LSP Client: ----------------- */

pub trait LSPServerRpc {
    
    fn initialize(&mut self, params: InitializeParams)
        -> GResult<RequestFuture<InitializeResult, InitializeError>>;
        
    fn shutdown(&mut self)
        -> GResult<RequestFuture<(), ()>>;
        
    fn exit(&mut self)
        -> GResult<()>;
        
    fn workspace_change_configuration(&mut self, params: DidChangeConfigurationParams)
        -> GResult<()>;
        
    fn did_open_text_document(&mut self, params: DidOpenTextDocumentParams)
        -> GResult<()>;
        
    fn did_change_text_document(&mut self, params: DidChangeTextDocumentParams)
        -> GResult<()>;
        
    fn did_close_text_document(&mut self, params: DidCloseTextDocumentParams)
        -> GResult<()>;
        
    fn did_save_text_document(&mut self, params: DidSaveTextDocumentParams)
        -> GResult<()>;
        
    fn did_change_watched_files(&mut self, params: DidChangeWatchedFilesParams)
        -> GResult<()>;
        
    fn completion(&mut self, params: TextDocumentPositionParams)
        -> GResult<RequestFuture<CompletionList, ()>>;
        
    fn resolve_completion_item(&mut self, params: CompletionItem)
        -> GResult<RequestFuture<CompletionItem, ()>>;
        
    fn hover(&mut self, params: TextDocumentPositionParams)
        -> GResult<RequestFuture<Hover, ()>>;
        
    fn signature_help(&mut self, params: TextDocumentPositionParams)
        -> GResult<RequestFuture<SignatureHelp, ()>>;
        
    fn goto_definition(&mut self, params: TextDocumentPositionParams)
        -> GResult<RequestFuture<Vec<Location>, ()>>;
        
    fn references(&mut self, params: ReferenceParams)
        -> GResult<RequestFuture<Vec<Location>, ()>>;
        
    fn document_highlight(&mut self, params: TextDocumentPositionParams)
        -> GResult<RequestFuture<Vec<DocumentHighlight>, ()>>;
        
    fn document_symbols(&mut self, params: DocumentSymbolParams)
        -> GResult<RequestFuture<Vec<SymbolInformation>, ()>>;
        
    fn workspace_symbols(&mut self, params: WorkspaceSymbolParams)
        -> GResult<RequestFuture<Vec<SymbolInformation>, ()>>;
        
    fn code_action(&mut self, params: CodeActionParams)
        -> GResult<RequestFuture<Vec<Command>, ()>>;
        
    fn code_lens(&mut self, params: CodeLensParams)
        -> GResult<RequestFuture<Vec<CodeLens>, ()>>;
        
    fn code_lens_resolve(&mut self, params: CodeLens)
        -> GResult<RequestFuture<CodeLens, ()>>;
        
    fn formatting(&mut self, params: DocumentFormattingParams)
        -> GResult<RequestFuture<Vec<TextEdit>, ()>>;
        
    fn range_formatting(&mut self, params: DocumentRangeFormattingParams)
        -> GResult<RequestFuture<Vec<TextEdit>, ()>>;
        
    fn on_type_formatting(&mut self, params: DocumentOnTypeFormattingParams)
        -> GResult<RequestFuture<Vec<TextEdit>, ()>>;
        
    fn rename(&mut self, params: RenameParams)
        -> GResult<RequestFuture<WorkspaceEdit, ()>>;
    
}


pub struct LSPClientEndpoint<'a> {
    pub endpoint: &'a mut Endpoint,    
}

impl<'a> LSPServerRpc for LSPClientEndpoint<'a> {
    
    fn initialize(&mut self, params: InitializeParams)
        -> GResult<RequestFuture<InitializeResult, InitializeError>> 
    {
        self.endpoint.send_request(REQUEST__Initialize, params)
    }
    
    fn shutdown(&mut self)
        -> GResult<RequestFuture<(), ()>>
    {
        self.endpoint.send_request(REQUEST__Shutdown, ())
    }
    
    fn exit(&mut self)
        -> GResult<()>
    {
        self.endpoint.send_notification(NOTIFICATION__Exit, ())
    }
    
    fn workspace_change_configuration(&mut self, params: DidChangeConfigurationParams)
        -> GResult<()>
    {
         self.endpoint.send_notification(NOTIFICATION__WorkspaceChangeConfiguration, params)
    }
    
    fn did_open_text_document(&mut self, params: DidOpenTextDocumentParams)
        -> GResult<()>
    {
        self.endpoint.send_notification(NOTIFICATION__DidOpenTextDocument, params)
    }
    
    fn did_change_text_document(&mut self, params: DidChangeTextDocumentParams)
        -> GResult<()>
    {
        self.endpoint.send_notification(NOTIFICATION__DidChangeTextDocument, params)
    }
    
    fn did_close_text_document(&mut self, params: DidCloseTextDocumentParams)
        -> GResult<()>
    {
        self.endpoint.send_notification(NOTIFICATION__DidCloseTextDocument, params)
    }
    
    fn did_save_text_document(&mut self, params: DidSaveTextDocumentParams)
        -> GResult<()>
    {
        self.endpoint.send_notification(NOTIFICATION__DidSaveTextDocument, params)
    }
    
    fn did_change_watched_files(&mut self, params: DidChangeWatchedFilesParams)
        -> GResult<()>
    {
        self.endpoint.send_notification(NOTIFICATION__DidChangeWatchedFiles, params)
    }
    
    fn completion(&mut self, params: TextDocumentPositionParams)
        -> GResult<RequestFuture<CompletionList, ()>>
    {
        self.endpoint.send_request(REQUEST__Completion, params)
    }
    
    fn resolve_completion_item(&mut self, params: CompletionItem)
        -> GResult<RequestFuture<CompletionItem, ()>>
    {
        self.endpoint.send_request(REQUEST__ResolveCompletionItem, params)
    }
    
    fn hover(&mut self, params: TextDocumentPositionParams)
        -> GResult<RequestFuture<Hover, ()>>
    {
        self.endpoint.send_request(REQUEST__Hover, params)
    }
    
    fn signature_help(&mut self, params: TextDocumentPositionParams)
        -> GResult<RequestFuture<SignatureHelp, ()>>
    {
        self.endpoint.send_request(REQUEST__SignatureHelp, params)
    }
    
    fn goto_definition(&mut self, params: TextDocumentPositionParams)
        -> GResult<RequestFuture<Vec<Location>, ()>>
    {
        self.endpoint.send_request(REQUEST__GotoDefinition, params)
    }
    
    fn references(&mut self, params: ReferenceParams)
        -> GResult<RequestFuture<Vec<Location>, ()>>
    {
        self.endpoint.send_request(REQUEST__References, params)
    }
    
    fn document_highlight(&mut self, params: TextDocumentPositionParams)
        -> GResult<RequestFuture<Vec<DocumentHighlight>, ()>>
    {
        self.endpoint.send_request(REQUEST__DocumentHighlight, params)
    }
    
    fn document_symbols(&mut self, params: DocumentSymbolParams)
        -> GResult<RequestFuture<Vec<SymbolInformation>, ()>>
    {
        self.endpoint.send_request(REQUEST__DocumentSymbols, params)
    }
    
    fn workspace_symbols(&mut self, params: WorkspaceSymbolParams)
        -> GResult<RequestFuture<Vec<SymbolInformation>, ()>>
    {
        self.endpoint.send_request(REQUEST__WorkspaceSymbols, params)
    }
    
    fn code_action(&mut self, params: CodeActionParams)
        -> GResult<RequestFuture<Vec<Command>, ()>>
    {
        self.endpoint.send_request(REQUEST__CodeAction, params)
    }
    
    fn code_lens(&mut self, params: CodeLensParams)
        -> GResult<RequestFuture<Vec<CodeLens>, ()>>
    {
        self.endpoint.send_request(REQUEST__CodeLens, params)
    }
    
    fn code_lens_resolve(&mut self, params: CodeLens)
        -> GResult<RequestFuture<CodeLens, ()>>
    {
        self.endpoint.send_request(REQUEST__CodeLensResolve, params)
    }
    
    fn formatting(&mut self, params: DocumentFormattingParams)
        -> GResult<RequestFuture<Vec<TextEdit>, ()>>
    {
        self.endpoint.send_request(REQUEST__Formatting, params)
    }
    
    fn range_formatting(&mut self, params: DocumentRangeFormattingParams)
        -> GResult<RequestFuture<Vec<TextEdit>, ()>>
    {
        self.endpoint.send_request(REQUEST__RangeFormatting, params)
    }
    
    fn on_type_formatting(&mut self, params: DocumentOnTypeFormattingParams)
        -> GResult<RequestFuture<Vec<TextEdit>, ()>>
    {
        self.endpoint.send_request(REQUEST__OnTypeFormatting, params)
    }
    
    fn rename(&mut self, params: RenameParams)
        -> GResult<RequestFuture<WorkspaceEdit, ()>>
    {
        self.endpoint.send_request(REQUEST__Rename, params)
    }
    
}


/// Trait for the handling of LSP client requests.
/// (An LSP server can act as a JSON-RPC Client and request to the LSP client)
pub trait LanguageClientHandling {
    
    fn show_message(&mut self, params: ShowMessageParams);
    
    fn show_message_request(&mut self, params: ShowMessageRequestParams, 
        completable: LSCompletable<MessageActionItem>);
    
    fn log_message(&mut self, params: LogMessageParams);
    
    fn telemetry_event(&mut self, params: Value);
    
    fn publish_diagnostics(&mut self, params: PublishDiagnosticsParams);
	
    #[allow(unused_variables)]
    fn handle_other_method(&mut self, method_name: &str, params: RequestParams, completable: ResponseCompletable) {
        completable.complete_with_error(jsonrpc_common::error_JSON_RPC_MethodNotFound()); 
    }
    
}

pub struct ClientRequestHandler<LS : ?Sized>(pub LS);

impl<LS : LanguageClientHandling + ?Sized> RequestHandler for ClientRequestHandler<LS> {

    fn handle_request(
        &mut self, method_name: &str, params: RequestParams, completable: ResponseCompletable
    ) {
        match method_name {
            NOTIFICATION__ShowMessage => {
                completable.handle_notification_with(params, 
                    |params| self.0.show_message(params)) 
            }
            REQUEST__ShowMessageRequest => {
                completable.handle_request_with(params, 
                    |params, completable| self.0.show_message_request(params, completable)
                )
            }
            NOTIFICATION__LogMessage => { 
                completable.handle_notification_with(params, 
                    |params| self.0.log_message(params)) 
            }
            NOTIFICATION__TelemetryEvent => {
                completable.handle_notification_with(params, 
                    |params| self.0.telemetry_event(params)
                ) 
            }
            NOTIFICATION__PublishDiagnostics => {
                completable.handle_notification_with(params, 
                    |params| self.0.publish_diagnostics(params)
                ) 
            }
            _ => {
                self.0.handle_other_method(method_name, params, completable);
            }
        }
    }
    
}