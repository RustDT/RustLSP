// Copyright 2016 Bruno Medeiros
//
// Licensed under the Apache License, Version 2.0 
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>. 
// This file may not be copied, modified, or distributed
// except according to those terms.


#![allow(non_camel_case_types)]

use json_rpc::service_util::*;
use json_rpc::RpcRequest;
use json_rpc::RpcNotification;

use serde_json::Value;
use serde;
use std::collections::HashMap;


// Based on protocol: https://github.com/Microsoft/language-server-protocol/blob/master/protocol.md
// Last update 07/10/2016 at commit: 
// https://github.com/Microsoft/language-server-protocol/commit/8fd7e96205a7750eb8440040c247f4a6533b238f


pub type LSResult<RET, ERR_DATA> = Result<RET, ServiceError<ERR_DATA>>;

pub type FnLanguageServerNotification<PARAMS> = 
	(&'static str, RpcNotification<PARAMS>);
pub type FnLanguageServerRequest<PARAMS, RET, ERR> = 
	(&'static str, RpcRequest<PARAMS, RET, ERR>);


fn notification<
	PARAMS : serde::Deserialize + 'static, 
>(name: &'static str, method_fn: Box<Fn(PARAMS)>) 
-> (&'static str, RpcNotification<PARAMS>) {
	(name, RpcNotification { method_fn : method_fn } )
}

fn request<
	PARAMS : serde::Deserialize + 'static, 
	RET: serde::Serialize + 'static, 
	ERR : serde::Serialize + 'static
>(name: &'static str, method_fn: Box<Fn(PARAMS) -> LSResult<RET, ERR>>) 
-> (&'static str, RpcRequest<PARAMS, RET, ERR>) {
	(name, RpcRequest { method_fn : method_fn } )
}

use std::rc::Rc;


pub trait LanguageServer {
	
	fn initialize(&self, params: InitializeParams) -> LSResult<InitializeResult, InitializeError>;
	fn shutdown(&self, params: ()) -> LSResult<(), ()>;
	fn exit(&self, params: ());
	fn showMessage(&self, params: ShowMessageParams);
	fn showMessageRequest(&self, params: ShowMessageRequestParams) -> LSResult<MessageActionItem, ()>;
	fn logMessage(&self, params: LogMessageParams);
	fn telemetryEvent(&self, params: any);
	fn workspaceChangeConfiguration(&self, params: DidChangeConfigurationParams);
	fn didOpenTextDocument(&self, params: DidOpenTextDocumentParams);
	fn didChangeTextDocument(&self, params: DidChangeTextDocumentParams);
	fn didCloseTextDocument(&self, params: DidCloseTextDocumentParams);
	fn didSaveTextDocument(&self, params: DidSaveTextDocumentParams);
	fn didChangeWatchedFiles(&self, params: DidChangeWatchedFilesParams);
	
	fn publishDiagnostics(&self, params: PublishDiagnosticsParams);

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
	// FIXME move methods here
}


/* ----------------- Basic JSON Structures ----------------- */

pub type boolean = bool;
pub type string = String;
pub type number = u64;
pub type number_or_string = string; /* FIXME: */
pub type any = Value;

/// Position in a text document expressed as zero-based line and character offset. 
/// A position is between two characters like an 'insert' cursor in a editor.
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct Position {
    /**
     * Line position in a document (zero-based).
     */
    pub line: number,

    /**
     * Character offset on a line in a document (zero-based).
     */
    pub character: number,
}

/// A range in a text document expressed as (zero-based) start and end positions. 
/// A range is comparable to a selection in an editor. Therefore the end position is exclusive.
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct Range {
    /**
     * The range's start position.
     */
    pub start: Position,

    /**
     * The range's end position.
     */
    pub end: Position,
}

///Represents a location inside a resource, such as a line inside a text file.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Location {
	
    pub uri: string,
    
    pub range: Range,
    
}

/// Represents a diagnostic, such as a compiler error or warning. 
/// Diagnostic objects are only valid in the scope of a resource.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Diagnostic {
    /**
     * The range at which the message applies.
     */
    pub range: Range,

    /**
     * The diagnostic's severity. Can be omitted. If omitted it is up to the
     * client to interpret diagnostics as error, warning, info or hint.
     */
    pub severity: Option<DiagnosticSeverity>,

    /**
     * The diagnostic's code. Can be omitted.
     */
//    code?: number | string;
    pub code: Option<number_or_string>,

    /**
     * A human-readable string describing the source of this
     * diagnostic, e.g. 'typescript' or 'super lint'.
     */
    pub source: Option<string>,

    /**
     * The diagnostic's message.
     */
    pub message: string,
}

/// The protocol currently supports the following diagnostic severities:
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum DiagnosticSeverity {
    /**
     * Reports an error.
     */
    Error = 1,
    /**
     * Reports a warning.
     */
    Warning = 2,
    /**
     * Reports an information.
     */
    Information = 3,
    /**
     * Reports a hint.
     */
    Hint = 4
}

/**
 Represents a reference to a command. Provides a title which will be used to represent a command in the UI. 
 Commands are identitifed using a string identifier and the protocol currently doesn't specify a set of 
 well known commands. So executing a command requires some tool extension code.
*/
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Command {
    /**
     * Title of the command, like `save`.
     */
    pub title: string,
    /**
     * The identifier of the actual command handler.
     */
    pub command: string,
    /**
     * Arguments that the command handler should be
     * invoked with.
     */
//    arguments?: any[];
    pub arguments: Option<Vec<any>>,
}

/**
 * A textual edit applicable to a text document.
 */
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TextEdit {
    /**
     * The range of the text document to be manipulated. To insert
     * text into a document create a range where start === end.
     */
    pub range: Range,

    /**
     * The string to be inserted. For delete operations use an
     * empty string.
     */
    pub newText: string,
}

/**
 * A workspace edit represents changes to many resources managed in the workspace.
 */
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WorkspaceEdit {
    /**
     * Holds changes to existing resources.
     */
    //changes: { [uri: string]: TextEdit[]; };
    pub changes: HashMap<String, Vec<TextEdit>>, // FIXME review if this is correct
}

/**
 * Text documents are identified using a URI. On the protocol level, URIs are passed as strings. 
 * The corresponding JSON structure looks like this:
 */
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TextDocumentIdentifier {
    /**
     * The text document's URI.
     */
    pub uri: string,
}

/**
 * An item to transfer a text document from the client to the server. 
 */
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TextDocumentItem {
    /**
     * The text document's URI.
     */
    pub uri: string,

    /**
     * The text document's language identifier.
     */
    pub languageId: string,

    /**
     * The version number of this document (it will strictly increase after each
     * change, including undo/redo).
     */
    pub version: number,

    /**
     * The content of the opened text document.
     */
    pub text: string,
}

/**
 * An identifier to denote a specific version of a text document.
 */
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct VersionedTextDocumentIdentifier 
//extends TextDocumentIdentifier  FIXME review this
{
	pub extends: TextDocumentIdentifier,
	
    /**
     * The version number of this document.
     */
    pub version: number,
}

/**
 * A parameter literal used in requests to pass a text document and a position inside that document.
 */
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TextDocumentPositionParams {
    /**
     * The text document.
     */
    pub textDocument: TextDocumentIdentifier, 

    /**
     * The position inside the text document.
     */
    pub position: Position,
}

/* ========================= Protocol Structures ========================= */

/**
 * The initialize request is sent as the first request from the client to the server.
 */
pub fn request__Initialize(ls : Rc<LanguageServer>) 
	-> FnLanguageServerRequest<InitializeParams, InitializeResult, InitializeError> 
{
	request("initialize", Box::new(move |params| {
		ls.initialize(params) 
	}))
}

#[derive(Clone, Serialize, Deserialize, Debug)] 
pub struct InitializeParams {
    /**
     * The process Id of the parent process that started
     * the server.
     */
    pub processId: Option<number>, // XXX: LSP protocol is ambiguous if it can be null

    /**
     * The rootPath of the workspace. Is null
     * if no folder is open.
     */
    pub rootPath: Option<string>, // XXX: LSP protocol is ambiguous if it can be null
    
    /**
     * User provided initialization options.
     */
    pub initializationOptions: Option<any>,

    /**
     * The capabilities provided by the client (editor)
     */
    pub capabilities: ClientCapabilities,
}

/**
 * Where ClientCapabilities are currently empty:
 */
pub type ClientCapabilities = HashMap<String, Value>;


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InitializeResult {
    /**
     * The capabilities the language server provides.
     */
    pub capabilities: ServerCapabilities,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InitializeError {
    /**
     * Indicates whether the client should retry to send the
     * initilize request after showing the message provided
     * in the ResponseError.
     */
    pub retry: boolean,
}

// The server can signal the following capabilities:

/**
 * Defines how the host (editor) should sync document changes to the language server.
 */
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum TextDocumentSyncKind {
    /**
     * Documents should not be synced at all.
     */
    None = 0,
    /**
     * Documents are synced by always sending the full content of the document.
     */
    Full = 1,
    /**
     * Documents are synced by sending the full content on open. After that only incremental 
     * updates to the document are sent.
     */
    Incremental = 2
}

/**
 * Completion options.
 */
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CompletionOptions {
    /**
     * The server provides support to resolve additional information for a completion item.
     */
    pub resolveProvider: Option<boolean>,

    /**
     * The characters that trigger completion automatically.
     */
    //pub triggerCharacters?: string[];
    pub triggerCharacters: Option<Vec<string>>,
}

/**
 * Signature help options.
 */
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SignatureHelpOptions {
    /**
     * The characters that trigger signature help automatically.
     */
    //triggerCharacters?: string[];
    pub triggerCharacters: Option<Vec<string>>,
}

/**
 * Code Lens options.
 */
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CodeLensOptions {
    /**
     * Code lens has a resolve provider as well.
     */
    pub resolveProvider: Option<boolean>,
}

/**
 * Format document on type options
 */
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DocumentOnTypeFormattingOptions {
    /**
     * A character on which formatting should be triggered, like `}`.
     */
    pub firstTriggerCharacter: string,
    /**
     * More trigger characters.
     */
    //moreTriggerCharacter?: string[],
    pub triggerCharacters: Option<Vec<string>>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct ServerCapabilities {
    /**
     * Defines how text documents are synced.
     */
    //textDocumentSync?: number;
    pub textDocumentSync: Option<TextDocumentSyncKind>,
    /**
     * The server provides hover support.
     */
    pub hoverProvider: Option<boolean>,
    /**
     * The server provides completion support.
     */
    pub completionProvider: Option<CompletionOptions>,
    /**
     * The server provides signature help support.
     */
    pub signatureHelpProvider: Option<SignatureHelpOptions>,
    /**
     * The server provides goto definition support.
     */
    pub definitionProvider: Option<boolean>,
    /**
     * The server provides find references support.
     */
    pub referencesProvider: Option<boolean>,
    /**
     * The server provides document highlight support.
     */
    pub documentHighlightProvider: Option<boolean>,
    /**
     * The server provides document symbol support.
     */
    pub documentSymbolProvider: Option<boolean>,
    /**
     * The server provides workspace symbol support.
     */
    pub workspaceSymbolProvider: Option<boolean>,
    /**
     * The server provides code actions.
     */
    pub codeActionProvider: Option<boolean>,
    /**
     * The server provides code lens.
     */
    pub codeLensProvider: Option<CodeLensOptions>,
    /**
     * The server provides document formatting.
     */
    pub documentFormattingProvider: Option<boolean>,
    /**
     * The server provides document range formatting.
     */
    pub documentRangeFormattingProvider: Option<boolean>,
    /**
     * The server provides document formatting on typing.
     */
    pub documentOnTypeFormattingProvider: Option<DocumentOnTypeFormattingOptions>,
    /**
     * The server provides rename support.
     */
    pub renameProvider: Option<boolean>,
}


/**
 * The shutdown request is sent from the client to the server. It asks the server to shut down,
 * but to not exit (otherwise the response might not be delivered correctly to the client).
 * There is a separate exit notification that asks the server to exit.
 */
pub fn request__Shutdown(ls : Rc<LanguageServer>) 
	-> FnLanguageServerRequest<(), (), ()> 
{
	request("shutdown", Box::new(move |params| {
		ls.shutdown(params) 
	}))
}

/**
 * A notification to ask the server to exit its process.
 */
pub fn notification__Exit(ls : Rc<LanguageServer>) 
	-> FnLanguageServerNotification<()> 
{
	notification("exit", Box::new(move |params| {
		ls.exit(params) 
	}))
}

/**
 * The show message notification is sent from a server to a client to ask the client to display a particular message
 * in the user interface.
 */
pub fn notification__ShowMessage(ls : Rc<LanguageServer>) 
	-> FnLanguageServerNotification<ShowMessageParams> 
{
	notification("window/showMessage", Box::new(move |params| {
		ls.showMessage(params) 
	}))
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ShowMessageParams {
    /**
     * The message type. See {@link MessageType}.
     */
    #[serde(rename="type")]
    pub type_: number,

    /**
     * The actual message.
     */
    pub message: string,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum MessageType {
    /**
     * An error message.
     */
    Error = 1,
    /**
     * A warning message.
     */
    Warning = 2,
    /**
     * An information message.
     */
    Info = 3,
    /**
     * A log message.
     */
    Log = 4
}

/**
 * The show message request is sent from a server to a client to ask the client to display a particular message
 * in the user interface. In addition to the show message notification the request allows to pass actions and to
 * wait for an answer from the client.
 */
pub fn request__ShowMessageRequest(ls : Rc<LanguageServer>) 
	-> FnLanguageServerRequest<ShowMessageRequestParams, MessageActionItem, ()> 
{
	request("window/showMessageRequest", Box::new(move |params| {
		ls.showMessageRequest(params) 
	}))
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ShowMessageRequestParams {
    /**
     * The message type. See {@link MessageType}
     */
    #[serde(rename="type")]
    pub type_: number,

    /**
     * The actual message
     */
    pub message: string,

    /**
     * The message action items to present.
     */
    pub actions: Option<Vec<MessageActionItem>>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MessageActionItem {
    /**
     * A short title like 'Retry', 'Open Log' etc.
     */
    title: string,
}

/**
 * The log message notification is sent from the server to the client to ask the client to log a particular message.
 */
pub fn notification__LogMessage(ls : Rc<LanguageServer>) 
	-> FnLanguageServerNotification<LogMessageParams> 
{
	notification("window/logMessage", Box::new(move |params| {
		ls.logMessage(params) 
	}))
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LogMessageParams {
    /**
     * The message type. See {@link MessageType}
     */
    #[serde(rename="type")]
    pub type_: number,

    /**
     * The actual message
     */
    pub message: string,
}

/**
 * The telemetry notification is sent from the server to the client to ask the client to log a telemetry event.
 */
pub fn notification__TelemetryEvent(ls : Rc<LanguageServer>) 
	-> FnLanguageServerNotification<any> 
{
	notification("telemetry/event", Box::new(move |params| {
		ls.telemetryEvent(params) 
	}))
}

/**
 * A notification sent from the client to the server to signal the change of configuration settings.
 */
pub fn notification__WorkspaceChangeConfiguration(ls : Rc<LanguageServer>) 
	-> FnLanguageServerNotification<DidChangeConfigurationParams> 
{
	notification("workspace/didChangeConfiguration", Box::new(move |params| {
		ls.workspaceChangeConfiguration(params) 
	}))
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DidChangeConfigurationParams {
    /**
     * The actual changed settings
     */
    //settings: any;
    pub settings: any,
}


/**
 * The document open notification is sent from the client to the server to signal newly opened text documents.
 * The document's truth is now managed by the client and the server must not try to read the document's truth
 * using the document's uri.
 */
pub fn notification__DidOpenTextDocument(ls : Rc<LanguageServer>) 
	-> FnLanguageServerNotification<DidOpenTextDocumentParams> 
{
	notification("textDocument/didOpen", Box::new(move |params| {
		ls.didOpenTextDocument(params) 
	}))
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DidOpenTextDocumentParams {
    /**
     * The document that was opened.
     */
    pub textDocument: TextDocumentItem,
}

/**
 * The document change notification is sent from the client to the server to signal changes to a text document.
 * In 2.0 the shape of the params has changed to include proper version numbers and language ids.
 */
pub fn notification__DidChangeTextDocument(ls : Rc<LanguageServer>) 
	-> FnLanguageServerNotification<DidChangeTextDocumentParams> 
{
	notification("textDocument/didChange", Box::new(move |params| {
		ls.didChangeTextDocument(params) 
	}))
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DidChangeTextDocumentParams {
    /**
     * The document that did change. The version number points
     * to the version after all provided content changes have
     * been applied.
     */
    pub textDocument: VersionedTextDocumentIdentifier,

    /**
     * The actual content changes.
     */
    pub contentChanges: Vec<TextDocumentContentChangeEvent>,
}

/**
 * An event describing a change to a text document. If range and rangeLength are omitted
 * the new text is considered to be the full content of the document.
 */
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TextDocumentContentChangeEvent {
    /**
     * The range of the document that changed.
     */
    pub range: Option<Range>,

    /**
     * The length of the range that got replaced.
     */
    // NOTE: seems redundant, see: https://github.com/Microsoft/language-server-protocol/issues/9
    pub rangeLength: Option<number>,

    /**
     * The new text of the document.
     */
    pub text: string,
}

/**
 * The document close notification is sent from the client to the server when the document got closed in the client.
 * The document's truth now exists where the document's uri points to (e.g. if the document's uri is a file uri
 * the truth now exists on disk).
 */
pub fn notification__DidCloseTextDocument(ls : Rc<LanguageServer>) 
	-> FnLanguageServerNotification<DidCloseTextDocumentParams> 
{
	notification("textDocument/didClose", Box::new(move |params| {
		ls.didCloseTextDocument(params) 
	}))
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DidCloseTextDocumentParams {
    /**
     * The document that was closed.
     */
    pub textDocument: TextDocumentIdentifier,
}

/**
 * The document save notification is sent from the client to the server when the document was saved in the client.
 */
pub fn notification__DidSaveTextDocument(ls : Rc<LanguageServer>) 
	-> FnLanguageServerNotification<DidSaveTextDocumentParams> 
{
	notification("textDocument/didSave", Box::new(move |params| {
		ls.didSaveTextDocument(params) 
	}))
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DidSaveTextDocumentParams {
    /**
     * The document that was saved.
     */
    pub textDocument: TextDocumentIdentifier,
}

/**
 * The watched files notification is sent from the client to the server when the client detects changes to files
 * watched by the language client.
 */
pub fn notification__DidChangeWatchedFiles(ls : Rc<LanguageServer>) 
	-> FnLanguageServerNotification<DidChangeWatchedFilesParams> 
{
	notification("workspace/didChangeWatchedFiles", Box::new(move |params| {
		ls.didChangeWatchedFiles(params) 
	}))
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DidChangeWatchedFilesParams {
    /**
     * The actual file events.
     */
    pub changes: Vec<FileEvent>,
}

/**
 * The file event type.
 */
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum FileChangeType {
    /**
     * The file got created.
     */
    Created = 1,
    /**
     * The file got changed.
     */
    Changed = 2,
    /**
     * The file got deleted.
     */
    Deleted = 3
}

/**
 * An event describing a file change.
 */
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FileEvent {
    /**
     * The file's URI.
     */
    pub uri: string,
    /**
     * The change type.
     */
    #[serde(rename="type")]
    pub type_: FileChangeType,
}

/**
 * Diagnostics notification are sent from the server to the client to signal results of validation runs.
 */
pub fn notification__PublishDiagnostics(ls : Rc<LanguageServer>) 
	-> FnLanguageServerNotification<PublishDiagnosticsParams> 
{
	notification("textDocument/publishDiagnostics", Box::new(move |params| {
		ls.publishDiagnostics(params) 
	}))
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PublishDiagnosticsParams {
    /**
     * The URI for which diagnostic information is reported.
     */
    pub uri: string,

    /**
     * An array of diagnostic information items.
     */
    pub diagnostics: Vec<Diagnostic>,
}

/**
 The Completion request is sent from the client to the server to compute completion items at a given cursor position. 
 Completion items are presented in the IntelliSense user interface. If computing full completion items is expensive, 
 servers can additionally provide a handler for the completion item resolve request ('completionItem/resolve'). 
 This request is sent when a completion item is selected in the user interface. A typically use case is for example: 
 the 'textDocument/completion' request doesn't fill in the documentation property for returned completion items 
 since it is expensive to compute. When the item is selected in the user interface then a 'completionItem/resolve' 
 request is sent with the selected completion item as a param. The returned completion item should have the 
 documentation property filled in.
*/
pub fn request__Completion(ls : Rc<LanguageServer>) 
// result: CompletionItem[] | CompletionList FIXME
	-> FnLanguageServerRequest<TextDocumentPositionParams, CompletionList, ()> 
{
	request("textDocument/completion", Box::new(move |params| {
		ls.completion(params) 
	}))
}


/**
 * Represents a collection of [completion items](#CompletionItem) to be presented
 * in the editor.
 */
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CompletionList {
    /**
     * This list it not complete. Further typing should result in recomputing
     * this list.
     */
    pub isIncomplete: boolean,
    /**
     * The completion items.
     */
    pub items: Vec<CompletionItem>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CompletionItem {
    /**
     * The label of this completion item. By default
     * also the text that is inserted when selecting
     * this completion.
     */
    pub label: string,
    /**
     * The kind of this completion item. Based of the kind
     * an icon is chosen by the editor.
     */
    pub kind: Option<CompletionItemKind>,
    /**
     * A human-readable string with additional information
     * about this item, like type or symbol information.
     */
    pub detail: Option<string>,
    /**
     * A human-readable string that represents a doc-comment.
     */
    pub documentation: Option<string>,
    /**
     * A string that shoud be used when comparing this item
     * with other items. When `falsy` the label is used.
     */
    pub sortText: Option<string>,
    /**
     * A string that should be used when filtering a set of
     * completion items. When `falsy` the label is used.
     */
    pub filterText: Option<string>,
    /**
     * A string that should be inserted a document when selecting
     * this completion. When `falsy` the label is used.
     */
    pub insertText: Option<string>,
    /**
     * An edit which is applied to a document when selecting
     * this completion. When an edit is provided the value of
     * insertText is ignored.
     */
    pub textEdit: Option<TextEdit>,
    /**
     * An data entry field that is preserved on a completion item between
     * a completion and a completion resolve request.
     */
    pub data: Option<any>,
}

/**
 * The kind of a completion entry.
 */
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum CompletionItemKind {
    Text = 1,
    Method = 2,
    Function = 3,
    Constructor = 4,
    Field = 5,
    Variable = 6,
    Class = 7,
    Interface = 8,
    Module = 9,
    Property = 10,
    Unit = 11,
    Value = 12,
    Enum = 13,
    Keyword = 14,
    Snippet = 15,
    Color = 16,
    File = 17,
    Reference = 18
}


/**
 * The request is sent from the client to the server to resolve additional information for a given completion item. 
 */
pub fn request__ResolveCompletionItem(ls : Rc<LanguageServer>) 
	-> FnLanguageServerRequest<CompletionItem, CompletionItem, ()>
{
	request("completionItem/resolve", Box::new(move |params| {
		ls.resolveCompletionItem(params) 
	}))
}


/**
 * The hover request is sent from the client to the server to request hover information at a given text 
 * document position.
 */
pub fn request__Hover(ls : Rc<LanguageServer>) 
	-> FnLanguageServerRequest<TextDocumentPositionParams, Hover, ()>
{
	request("textDocument/hover", Box::new(move |params| {
		ls.hover(params) 
	}))
}

/**
 * The result of a hover request.
 */
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Hover {
    /**
     * The hover's content
     */
    //contents: MarkedString | MarkedString[];
    pub contents: Vec<MarkedString>, /* FIXME: */

    /**
     * An optional range
     */
    pub range: Option<Range>,
}

//type MarkedString = string | { language: string; value: string };
pub type MarkedString = string; /* FIXME: todo*/

/**
 * The signature help request is sent from the client to the server to request signature information at 
 * a given cursor position.
 */
pub fn request__SignatureHelp(ls : Rc<LanguageServer>) 
	-> FnLanguageServerRequest<TextDocumentPositionParams, SignatureHelp, ()>
{
	request("textDocument/signatureHelp", Box::new(move |params| {
		ls.signatureHelp(params) 
	}))
}


/**
 * Signature help represents the signature of something
 * callable. There can be multiple signature but only one
 * active and only one active parameter.
 */
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SignatureHelp {
    /**
     * One or more signatures.
     */
    pub signatures: Vec<SignatureInformation>,

    /**
     * The active signature.
     */
    pub activeSignature: Option<number>,

    /**
     * The active parameter of the active signature.
     */
    pub activeParameter: Option<number>,
}

/**
 * Represents the signature of something callable. A signature
 * can have a label, like a function-name, a doc-comment, and
 * a set of parameters.
 */
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SignatureInformation {
    /**
     * The label of this signature. Will be shown in
     * the UI.
     */
    pub label: string,

    /**
     * The human-readable doc-comment of this signature. Will be shown
     * in the UI but can be omitted.
     */
    pub documentation: Option<string>,

    /**
     * The parameters of this signature.
     */
    pub parameters: Option<Vec<ParameterInformation>>,
}

/**
 * Represents a parameter of a callable-signature. A parameter can
 * have a label and a doc-comment.
 */
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ParameterInformation {
    /**
     * The label of this signature. Will be shown in
     * the UI.
     */
    pub label: string,

    /**
     * The human-readable doc-comment of this signature. Will be shown
     * in the UI but can be omitted.
     */
    pub documentation: Option<string>,
}


/**
 * The goto definition request is sent from the client to the server to resolve the definition location of 
 * a symbol at a given text document position.
 */
pub fn request__GotoDefinition(ls : Rc<LanguageServer>) 
	-> FnLanguageServerRequest<TextDocumentPositionParams, Vec<Location>, ()>
{
	request("textDocument/definition", Box::new(move |params| {
		ls.gotoDefinition(params) 
	}))
}

/**
 * The references request is sent from the client to the server to resolve project-wide references for the 
 * symbol denoted by the given text document position.
 */
pub fn request__References(ls : Rc<LanguageServer>) 
	-> FnLanguageServerRequest<ReferenceParams, Vec<Location>, ()> 
{
	request("textDocument/references", Box::new(move |params| {
		ls.references(params) 
	}))
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ReferenceParams 
//extends TextDocumentPositionParams FIXME
{
    pub context: ReferenceContext,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ReferenceContext {
    /**
     * Include the declaration of the current symbol.
     */
    pub includeDeclaration: boolean,
}


/**
 The document highlight request is sent from the client to the server to resolve a document highlights 
 for a given text document position. 
 For programming languages this usually highlights all references to the symbol scoped to this file. 
 However we kept 'textDocument/documentHighlight' and 'textDocument/references' separate requests since 
 the first one is allowed to be more fuzzy. 
 Symbol matches usually have a DocumentHighlightKind of Read or Write whereas fuzzy or textual matches 
 use Textas the kind.
*/
pub fn request__DocumentHighlight(ls : Rc<LanguageServer>) 
	-> FnLanguageServerRequest<TextDocumentPositionParams, DocumentHighlight, ()>
{
	request("textDocument/documentHighlight", Box::new(move |params| {
		ls.documentHighlight(params) 
	}))
}

/**
 * A document highlight is a range inside a text document which deserves
 * special attention. Usually a document highlight is visualized by changing
 * the background color of its range.
 */
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DocumentHighlight {
    /**
     * The range this highlight applies to.
     */
    pub range: Range,

    /**
     * The highlight kind, default is DocumentHighlightKind.Text.
     */
    pub kind: Option<number>,
}

/**
 * A document highlight kind.
 */
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum DocumentHighlightKind {
    /**
     * A textual occurrance.
     */
    Text = 1,

    /**
     * Read-access of a symbol, like reading a variable.
     */
    Read = 2,

    /**
     * Write-access of a symbol, like writing to a variable.
     */
    Write = 3
}

/**
 * The document symbol request is sent from the client to the server to list all symbols found in a given 
 * text document.
 */
pub fn request__DocumentSymbols(ls : Rc<LanguageServer>) 
	-> FnLanguageServerRequest<DocumentSymbolParams, Vec<SymbolInformation>, ()>
{
	request("textDocument/documentSymbol", Box::new(move |params| {
		ls.documentSymbols(params) 
	}))
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DocumentSymbolParams {
    /**
     * The text document.
     */
    pub textDocument: TextDocumentIdentifier,
}

/**
 * Represents information about programming constructs like variables, classes,
 * interfaces etc.
 */
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SymbolInformation {
    /**
     * The name of this symbol.
     */
    pub name: string,

    /**
     * The kind of this symbol.
     */
    pub kind: number,

    /**
     * The location of this symbol.
     */
    pub location: Location,

    /**
     * The name of the symbol containing this symbol.
     */
    pub containerName: Option<string>
}

/**
 * A symbol kind.
 */
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum SymbolKind {
    File = 1,
    Module = 2,
    Namespace = 3,
    Package = 4,
    Class = 5,
    Method = 6,
    Property = 7,
    Field = 8,
    Constructor = 9,
    Enum = 10,
    Interface = 11,
    Function = 12,
    Variable = 13,
    Constant = 14,
    String = 15,
    Number = 16,
    Boolean = 17,
    Array = 18,
}

/**
 * The workspace symbol request is sent from the client to the server to list project-wide symbols 
 * matching the query string.
 */
pub fn request__WorkspaceSymbols(ls : Rc<LanguageServer>) 
	-> FnLanguageServerRequest<WorkspaceSymbolParams, Vec<SymbolInformation>, ()>
{
	request("workspace/symbol", Box::new(move |params| {
		ls.workspaceSymbols(params) 
	}))
}

/**
 * The parameters of a Workspace Symbol Request.
 */
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WorkspaceSymbolParams {
    /**
     * A non-empty query string
     */
    pub query: string,
}

/**
 * The code action request is sent from the client to the server to compute commands for a given text document
 * and range. The request is triggered when the user moves the cursor into a problem marker in the editor or 
 * presses the lightbulb associated with a marker.
 */
pub fn request__CodeAction(ls : Rc<LanguageServer>) 
	-> FnLanguageServerRequest<CodeActionParams, Vec<Command>, ()>
{
	request("textDocument/codeAction", Box::new(move |params| {
		ls.codeAction(params) 
	}))
}

/**
 * Params for the CodeActionRequest
 */
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CodeActionParams {
    /**
     * The document in which the command was invoked.
     */
    pub textDocument: TextDocumentIdentifier,

    /**
     * The range for which the command was invoked.
     */
    pub range: Range,

    /**
     * Context carrying additional information.
     */
    pub context: CodeActionContext,
}

/**
 * Contains additional diagnostic information about the context in which
 * a code action is run.
 */
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CodeActionContext {
    /**
     * An array of diagnostics.
     */
    pub diagnostics: Vec<Diagnostic>,
}

/**
 * The code lens request is sent from the client to the server to compute code lenses for a given text document.
 */
pub fn request__CodeLens(ls : Rc<LanguageServer>) 
	-> FnLanguageServerRequest<CodeLensParams, Vec<CodeLens>, ()>
{
	request("textDocument/codeLens", Box::new(move |params| {
		ls.codeLens(params) 
	}))
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CodeLensParams {
    /**
     * The document to request code lens for.
     */
    pub textDocument: TextDocumentIdentifier,
}


/**
 * A code lens represents a command that should be shown along with
 * source text, like the number of references, a way to run tests, etc.
 *
 * A code lens is _unresolved_ when no command is associated to it. For performance
 * reasons the creation of a code lens and resolving should be done in two stages.
 */
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CodeLens {
    /**
     * The range in which this code lens is valid. Should only span a single line.
     */
    pub range: Range,

    /**
     * The command this code lens represents.
     */
    pub command: Option<Command>,

    /**
     * A data entry field that is preserved on a code lens item between
     * a code lens and a code lens resolve request.
     */
    pub data: Option<any>
}


/**
 * The code lens resolve request is sent from the client to the server to resolve the command for a 
 * given code lens item.
 */
pub fn request__CodeLensResolve(ls : Rc<LanguageServer>) 
	-> FnLanguageServerRequest<CodeLens, CodeLens, ()> 
{
	request("codeLens/resolve", Box::new(move |params| {
		ls.codeLensResolve(params) 
	}))
}

/**
 * The document formatting request is sent from the server to the client to format a whole document.
 */
pub fn request__Formatting(ls : Rc<LanguageServer>) 
	-> FnLanguageServerRequest<DocumentFormattingParams, Vec<TextEdit>, ()>
{
	request("textDocument/formatting", Box::new(move |params| {
		ls.formatting(params) 
	}))
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DocumentFormattingParams {
    /**
     * The document to format.
     */
    pub textDocument: TextDocumentIdentifier,

    /**
     * The format options.
     */
    pub options: FormattingOptions,
}

/**
 * Value-object describing what options formatting should use.
 */
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FormattingOptions {
    /**
     * Size of a tab in spaces.
     */
    pub tabSize: number,

    /**
     * Prefer spaces over tabs.
     */
    pub insertSpaces: boolean,

//    /**
//     * Signature for further properties.
//     */
    //[key: string]: boolean | number | string;
    // FIXME what is this, I don't quite get it
    
}

/**
 * The document range formatting request is sent from the client to the server to format a given range in a document.
 */
pub fn request__RangeFormatting(ls : Rc<LanguageServer>) 
	-> FnLanguageServerRequest<DocumentRangeFormattingParams, Vec<TextEdit>, ()> 
{
	request("textDocument/rangeFormatting", Box::new(move |params| {
		ls.rangeFormatting(params) 
	}))
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DocumentRangeFormattingParams {
    /**
     * The document to format.
     */
    pub textDocument: TextDocumentIdentifier,

    /**
     * The range to format
     */
    pub range: Range,

    /**
     * The format options
     */
    pub options: FormattingOptions,
}

/**
 * The document on type formatting request is sent from the client to the server to format parts of 
 * the document during typing.
 */
pub fn request__OnTypeFormatting(ls : Rc<LanguageServer>) 
	-> FnLanguageServerRequest<DocumentOnTypeFormattingParams, Vec<TextEdit>, ()> 
{
	request("textDocument/onTypeFormatting", Box::new(move |params| {
		ls.onTypeFormatting(params) 
	}))
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DocumentOnTypeFormattingParams {
    /**
     * The document to format.
     */
    pub textDocument: TextDocumentIdentifier,

    /**
     * The position at which this request was sent.
     */
    pub position: Position,

    /**
     * The character that has been typed.
     */
    pub ch: string,

    /**
     * The format options.
     */
    pub options: FormattingOptions,
}

/**
 * The rename request is sent from the client to the server to perform a workspace-wide rename of a symbol.
 */
pub fn request__Rename(ls : Rc<LanguageServer>) 
	-> FnLanguageServerRequest<RenameParams, WorkspaceEdit, ()> 
{
	request("textDocument/rename", Box::new(move |params| {
		ls.rename(params) 
	}))
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RenameParams {
    /**
     * The document to format.
     */
    pub textDocument: TextDocumentIdentifier,

    /**
     * The position at which this request was sent.
     */
    pub position: Position,

    /**
     * The new name of the symbol. If the given name is not valid the
     * request must return a [ResponseError](#ResponseError) with an
     * appropriate message set.
     */
    pub newName: string,
}