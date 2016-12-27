/* ----------------- Tests ----------------- */


use lsp::*;
use jsonrpc::method_types::MethodError;
use jsonrpc::*;
use ls_types::*;

use jsonrpc::json_util::JsonObject;
use serde_json::Value;

use std::io;
use std::thread;
use std::net::TcpListener;
use std::net::TcpStream;


#[test]
pub fn test_run_lsp_server() {
    
    let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
    let local_addr = listener.local_addr().unwrap();
    
    let server_listener = thread::spawn(|| {
        tcp_server(listener)
    });
    
    let stream = TcpStream::connect(local_addr).unwrap();
    let out_stream = stream.try_clone().expect("Failed to clone stream");
    let mut endpoint = LSPEndpoint::create_lsp_output_with_output_stream(|| { out_stream });
    
    let ls_client = TestsLanguageClient { counter: 0, endpoint : endpoint.clone() };
    
    let client_handler = thread::spawn(|| {
        let mut input = io::BufReader::new(stream);
        let endpoint = ls_client.endpoint.clone();
        LSPEndpoint::run_client_from_input(&mut input, endpoint, ls_client);
    });
    
    let init_params = InitializeParams { 
        process_id: None, 
        root_path: None,
        initialization_options: None,
        capabilities: Value::Object(JsonObject::new()),
    };
    
    // Create an rpc handle to the server methods
    let mut server_handle = server_rpc_handle(&mut endpoint);
    
    server_handle.initialize(init_params).unwrap();
    
    server_handle.shutdown().unwrap();
    
    server_handle.exit().unwrap();
    
    client_handler.join().unwrap();
    server_listener.join().unwrap();
}

fn tcp_server(listener: TcpListener) {
    
    for stream in listener.incoming() {
        let stream = stream.expect("Failed to open incoming stream");
        let conn_handler = thread::spawn(move|| {
            handle_connection(stream)
        });
        
        // Only listen to first connection, so that this example can be run as a test
        conn_handler.join().unwrap();
        break; 
    }
    
    drop(listener);
}

fn handle_connection(stream: TcpStream) {
    let out_stream = stream.try_clone().expect("Failed to clone stream");
    let endpoint = LSPEndpoint::create_lsp_output_with_output_stream(|| { out_stream });
    
    let ls = TestsLanguageServer { counter : 0, endpoint : endpoint.clone() };
    
    let mut input = io::BufReader::new(stream);
    LSPEndpoint::run_server_from_input(&mut input, endpoint, ls);
}

pub struct TestsLanguageServer {
    counter: u32,
    endpoint: Endpoint,
}

impl TestsLanguageServer {
    
    pub fn error_not_available<DATA>(data : DATA) -> MethodError<DATA> {
        let msg = "Functionality not implemented.".to_string();
        MethodError::<DATA> { code : 1, message : msg, data : data }
    }
    
}

impl LanguageServerHandling for TestsLanguageServer {
    
    fn initialize(&mut self, _: InitializeParams, completable: MethodCompletable<InitializeResult, InitializeError>) {
        let capabilities = ServerCapabilities::default();
        assert_eq!(self.counter, 0);
        self.counter = 1;
        completable.complete(Ok(InitializeResult { capabilities : capabilities }))
    }
    fn shutdown(&mut self, _: (), completable: LSCompletable<()>) {
        completable.complete(Ok(()));
    }
    fn exit(&mut self, _: ()) {
        self.endpoint.request_shutdown();
    }
    
    fn workspace_change_configuration(&mut self, _: DidChangeConfigurationParams) {}
    fn did_open_text_document(&mut self, _: DidOpenTextDocumentParams) {}
    fn did_change_text_document(&mut self, _: DidChangeTextDocumentParams) {}
    fn did_close_text_document(&mut self, _: DidCloseTextDocumentParams) {}
    fn did_save_text_document(&mut self, _: DidSaveTextDocumentParams) {}
    fn did_change_watched_files(&mut self, _: DidChangeWatchedFilesParams) {}
    
    fn completion(&mut self, _: TextDocumentPositionParams, completable: LSCompletable<CompletionList>) {
        completable.complete(Err(Self::error_not_available(())));
    }
    fn resolve_completion_item(&mut self, _: CompletionItem, completable: LSCompletable<CompletionItem>) {
        completable.complete(Err(Self::error_not_available(())));
    }
    fn hover(&mut self, _: TextDocumentPositionParams, completable: LSCompletable<Hover>) {
        let mut endpoint = self.endpoint.clone();
        thread::spawn(move || {
            client_rpc_handle(&mut endpoint).telemetry_event(Value::Null)
                .unwrap();
            
            let hover_str = "hover_text".to_string();
            let hover = Hover { contents: vec![MarkedString::String(hover_str)], range: None };
            
            completable.complete(Ok(hover));
        });
    }
    fn signature_help(&mut self, _: TextDocumentPositionParams, completable: LSCompletable<SignatureHelp>) {
        completable.complete(Err(Self::error_not_available(())));
    }
    fn goto_definition(&mut self, _: TextDocumentPositionParams, completable: LSCompletable<Vec<Location>>) {
        completable.complete(Err(Self::error_not_available(())));
    }
    fn references(&mut self, _: ReferenceParams, completable: LSCompletable<Vec<Location>>) {
        completable.complete(Err(Self::error_not_available(())));
    }
    fn document_highlight(&mut self, _: TextDocumentPositionParams, completable: LSCompletable<Vec<DocumentHighlight>>) {
        completable.complete(Err(Self::error_not_available(())));
    }
    fn document_symbols(&mut self, _: DocumentSymbolParams, completable: LSCompletable<Vec<SymbolInformation>>) {
        completable.complete(Err(Self::error_not_available(())));
    }
    fn workspace_symbols(&mut self, _: WorkspaceSymbolParams, completable: LSCompletable<Vec<SymbolInformation>>) {
        completable.complete(Err(Self::error_not_available(())));
    }
    fn code_action(&mut self, _: CodeActionParams, completable: LSCompletable<Vec<Command>>) {
        completable.complete(Err(Self::error_not_available(())));
    }
    fn code_lens(&mut self, _: CodeLensParams, completable: LSCompletable<Vec<CodeLens>>) {
        completable.complete(Err(Self::error_not_available(())));
    }
    fn code_lens_resolve(&mut self, _: CodeLens, completable: LSCompletable<CodeLens>) {
        completable.complete(Err(Self::error_not_available(())));
    }
    fn document_link(&mut self, _params: DocumentLinkParams, completable: LSCompletable<Vec<DocumentLink>>) {
        completable.complete(Err(Self::error_not_available(())));
    }
    fn document_link_resolve(&mut self, _params: DocumentLink, completable: LSCompletable<DocumentLink>) {
        completable.complete(Err(Self::error_not_available(())));
    }
    fn formatting(&mut self, _: DocumentFormattingParams, completable: LSCompletable<Vec<TextEdit>>) {
        completable.complete(Err(Self::error_not_available(())));
    }
    fn range_formatting(&mut self, _: DocumentRangeFormattingParams, completable: LSCompletable<Vec<TextEdit>>) {
        completable.complete(Err(Self::error_not_available(())));
    }
    fn on_type_formatting(&mut self, _: DocumentOnTypeFormattingParams, completable: LSCompletable<Vec<TextEdit>>) {
        completable.complete(Err(Self::error_not_available(())));
    }
    fn rename(&mut self, _: RenameParams, completable: LSCompletable<WorkspaceEdit>) {
        completable.complete(Err(Self::error_not_available(())));
    }
}

/* -----------------  ----------------- */

pub struct TestsLanguageClient {
    counter: u32,
    endpoint: Endpoint,
}

#[allow(unused_variables)]
impl LanguageClientHandling for TestsLanguageClient {
    
    fn show_message(&mut self, params: ShowMessageParams) {
        
    }
    
    fn show_message_request(
        &mut self, params: ShowMessageRequestParams, completable: LSCompletable<MessageActionItem>
    ) {
        unimplemented!();
    }
    
    fn log_message(&mut self, params: LogMessageParams) {
        
    }
    
    fn telemetry_event(&mut self, params: Value) {
        self.counter += 1;
    }
    
    fn publish_diagnostics(&mut self, params: PublishDiagnosticsParams) {
        
    }
    
}