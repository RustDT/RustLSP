// Copyright 2016 Bruno Medeiros
//
// Licensed under the Apache License, Version 2.0 
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>. 
// This file may not be copied, modified, or distributed
// except according to those terms.


// Note: Rust newbie code ahead (-_-)'

#![allow(non_camel_case_types)]

//use ::util::core::*;

extern crate serde_json;

use self::serde_json::Map;
use self::serde_json::Value;
use std::collections::HashMap;

/* ----------------- Basic JSON Structures ----------------- */

pub type string = String;
pub type number = u64;
pub type number_or_string = string; /* FIXME: */
pub type any_array = Vec<Value>;

/// Position in a text document expressed as zero-based line and character offset.
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

///A range in a text document expressed as (zero-based) start and end positions.
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
    pub severity: Option<number>,

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

/// Represents a reference to a command. Provides a title which will be used to represent a command in the UI and, 
/// optionally, an array of arguments which will be passed to the command handler function when invoked.
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
    pub arguments: Option<any_array>,
}

/// A textual edit applicable to a text document.
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

/// A workspace edit represents changes to many resources managed in the workspace.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WorkspaceEdit {
    /**
     * Holds changes to existing resources.
     */
    //changes: { [uri: string]: TextEdit[]; };
    pub changes: HashMap<String, Vec<TextEdit>>,
}

/// Text documents are identified using a URI. On the protocol level, URIs are passed as strings. The corresponding JSON structure looks like this:
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TextDocumentIdentifier {
    /**
     * The text document's URI.
     */
    pub uri: string,
}

/// An item to transfer a text document from the client to the server.
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

/// An identifier to denote a specific version of a text document.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct VersionedTextDocumentIdentifier 
//extends TextDocumentIdentifier 
{
	pub extends: TextDocumentIdentifier,
	
    /**
     * The version number of this document.
     */
    pub version: number,
}

/// A parameter literal used in requests to pass a text document and a position inside that document.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TextDocumentPositionParams {
    /**
     * The text document.
     */
    pub textDocument: TextDocumentIdentifier, // FIXME, might be VersionedTextDocumentIdentifier

    /**
     * The position inside the text document.
     */
    pub position: Position,
}

/* ----------------- Protocol Structures ----------------- */

/// The initialize request is sent as the first request from the client to the server.
#[derive(Clone, Serialize, Deserialize, Debug)] 
pub struct InitializeParams {
    /**
     * The process Id of the parent process that started
     * the server.
     */
    pub processId: number,

    /**
     * The rootPath of the workspace. Is null
     * if no folder is open.
     */
    pub rootPath: string,

    /**
     * The capabilities provided by the client (editor)
     */
    pub capabilities: ClientCapabilities,
}

/// Where ClientCapabilities are currently empty:
//struct ClientCapabilities {
//}

pub type ClientCapabilities = HashMap<String, Value>;
