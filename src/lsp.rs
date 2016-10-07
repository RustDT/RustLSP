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


fn notification<PARAMS: serde::Deserialize +
                'static>(name: &'static str, method_fn: Box<Fn(PARAMS)>)
 -> (&'static str, RpcNotification<PARAMS>) {
    (name, RpcNotification{method_fn: method_fn,})
}

fn request<PARAMS: serde::Deserialize + 'static, RET: serde::Serialize +
           'static, ERR: serde::Serialize +
           'static>(name: &'static str,
                    method_fn: Box<Fn(PARAMS) -> LSResult<RET, ERR>>)
 -> (&'static str, RpcRequest<PARAMS, RET, ERR>) {
    (name, RpcRequest{method_fn: method_fn,})
}

use std::rc::Rc;


pub trait LanguageServer {
    fn initialize(&self, params: InitializeParams)
    -> LSResult<InitializeResult, InitializeError>;
    fn shutdown(&self, params: ())
    -> LSResult<(), ()>;
    fn exit(&self, params: ());
    fn showMessage(&self, params: ShowMessageParams);
    fn showMessageRequest(&self, params: ShowMessageRequestParams)
    -> LSResult<MessageActionItem, ()>;
    fn logMessage(&self, params: LogMessageParams);
    fn telemetryEvent(&self, params: any);
    fn workspaceChangeConfiguration(&self,
                                    params: DidChangeConfigurationParams);
    fn didOpenTextDocument(&self, params: DidOpenTextDocumentParams);
    fn didChangeTextDocument(&self, params: DidChangeTextDocumentParams);
    fn didCloseTextDocument(&self, params: DidCloseTextDocumentParams);
    fn didSaveTextDocument(&self, params: DidSaveTextDocumentParams);
    fn didChangeWatchedFiles(&self, params: DidChangeWatchedFilesParams);
    fn publishDiagnostics(&self, params: PublishDiagnosticsParams);

    fn completion(&self, params: TextDocumentPositionParams)
    -> LSResult<CompletionList, ()>;
    fn resolveCompletionItem(&self, params: CompletionItem)
    -> LSResult<CompletionItem, ()>;
    fn hover(&self, params: TextDocumentPositionParams)
    -> LSResult<Hover, ()>;
    fn signatureHelp(&self, params: TextDocumentPositionParams)
    -> LSResult<SignatureHelp, ()>;
    fn gotoDefinition(&self, params: TextDocumentPositionParams)
    -> LSResult<Vec<Location>, ()>;
    fn references(&self, params: ReferenceParams)
    -> LSResult<Vec<Location>, ()>;
    fn documentHighlight(&self, params: TextDocumentPositionParams)
    -> LSResult<DocumentHighlight, ()>;
    fn documentSymbols(&self, params: DocumentSymbolParams)
    -> LSResult<Vec<SymbolInformation>, ()>;
    fn workspaceSymbols(&self, params: WorkspaceSymbolParams)
    -> LSResult<Vec<SymbolInformation>, ()>;
    fn codeAction(&self, params: CodeActionParams)
    -> LSResult<Vec<Command>, ()>;
    fn codeLens(&self, params: CodeLensParams)
    -> LSResult<Vec<CodeLens>, ()>;
    fn codeLensResolve(&self, params: CodeLens)
    -> LSResult<CodeLens, ()>;
    fn formatting(&self, params: DocumentFormattingParams)
    -> LSResult<Vec<TextEdit>, ()>;
    fn rangeFormatting(&self, params: DocumentRangeFormattingParams)
    -> LSResult<Vec<TextEdit>, ()>;
    fn onTypeFormatting(&self, params: DocumentOnTypeFormattingParams)
    -> LSResult<Vec<TextEdit>, ()>;
    fn rename(&self, params: RenameParams)
    -> LSResult<WorkspaceEdit, ()>;
}


pub trait LanguageClient {
    // FIXME move methods here
}


/* ----------------- Basic JSON Structures ----------------- */

pub type boolean = bool;
pub type string = String;
pub type number = u64;
pub type number_or_string = string;
 /* FIXME: */
pub type any = Value;

/// Position in a text document expressed as zero-based line and character offset. 
/// A position is between two characters like an 'insert' cursor in a editor.
#[derive(Debug, Copy, Clone)]
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
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_Position: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for Position {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<Position, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __field1, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        1usize => { Ok(__Field::__field1) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "line" => { Ok(__Field::__field0) }
                                        "character" => {
                                            Ok(__Field::__field1)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"line" => { Ok(__Field::__field0) }
                                        b"character" => {
                                            Ok(__Field::__field1)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: 





                          //    code?: number | string;




                          //    arguments?: any[];



                          //changes: { [uri: string]: TextEdit[]; };
                           // FIXME review if this is correct






                          //extends TextDocumentIdentifier  FIXME review this



                          /* ========================= Protocol Structures ========================= */









                          // The server can signal the following capabilities:



                          //pub triggerCharacters?: string[];

                          //triggerCharacters?: string[];


                          //moreTriggerCharacter?: string[],

                          //textDocumentSync?: number;


















                          //settings: any;








                          // NOTE: seems redundant, see: https://github.com/Microsoft/language-server-protocol/issues/9













                          // result: CompletionItem[] | CompletionList FIXME









                          //contents: MarkedString | MarkedString[];
                           /* FIXME: */


                          //type MarkedString = string | { language: string; value: string };
                           /* FIXME: todo*/














                          //extends TextDocumentPositionParams FIXME




































                          //    /**
                          //     * Signature for further properties.
                          //     */
                          //[key: string]: boolean | number | string;
                          // FIXME what is this, I don't quite get it













                          _serde::de::Deserializer> _serde::de::Visitor for
                     __Visitor<__D> {
                        type
                        Value
                        =
                        Position;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         -> ::std::result::Result<Position, __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: < number > (
                                                )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                let __field1 =
                                    match try!(visitor . visit :: < number > (
                                                )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(1usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(Position{line: __field0,
                                            character: __field1,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         -> ::std::result::Result<Position, __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0: Option<number> = None;
                                let mut __field1: Option<number> = None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("line"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          number > (  )));
                                        }
                                        __Field::__field1 => {
                                            if __field1.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("character"));
                                            }
                                            __field1 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          number > (  )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field ( "line"
                                             )),
                                    };
                                let __field1 =
                                    match __field1 {
                                        Some(__field1) => __field1,
                                        None =>
                                        try!(visitor . missing_field (
                                             "character" )),
                                    };
                                Ok(Position{line: __field0,
                                            character: __field1,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["line", "character"];
                    deserializer.deserialize_struct("Position", FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_Position: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for Position {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "Position" , 0 + 1 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "line" , &self.line ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "character" , &self.character ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
/// A range in a text document expressed as (zero-based) start and end positions. 
/// A range is comparable to a selection in an editor. Therefore the end position is exclusive.
#[derive(Debug, Copy, Clone)]
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
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_Range: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for Range {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<Range, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __field1, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        1usize => { Ok(__Field::__field1) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "start" => { Ok(__Field::__field0) }
                                        "end" => { Ok(__Field::__field1) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"start" => { Ok(__Field::__field0) }
                                        b"end" => { Ok(__Field::__field1) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        Range;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         -> ::std::result::Result<Range, __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: < Position >
                                               (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                let __field1 =
                                    match try!(visitor . visit :: < Position >
                                               (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(1usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(Range{start: __field0, end: __field1,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         -> ::std::result::Result<Range, __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0: Option<Position> = None;
                                let mut __field1: Option<Position> = None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("start"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Position > (  )));
                                        }
                                        __Field::__field1 => {
                                            if __field1.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("end"));
                                            }
                                            __field1 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Position > (  )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field ( "start"
                                             )),
                                    };
                                let __field1 =
                                    match __field1 {
                                        Some(__field1) => __field1,
                                        None =>
                                        try!(visitor . missing_field ( "end"
                                             )),
                                    };
                                Ok(Range{start: __field0, end: __field1,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["start", "end"];
                    deserializer.deserialize_struct("Range", FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_Range: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for Range {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "Range" , 0 + 1 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "start" , &self.start ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "end" , &self.end ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
///Represents a location inside a resource, such as a line inside a text file.
#[derive(Debug, Clone)]
pub struct Location {
    pub uri: string,
    pub range: Range,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_Location: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for Location {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<Location, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __field1, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        1usize => { Ok(__Field::__field1) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "uri" => { Ok(__Field::__field0) }
                                        "range" => { Ok(__Field::__field1) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"uri" => { Ok(__Field::__field0) }
                                        b"range" => { Ok(__Field::__field1) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        Location;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         -> ::std::result::Result<Location, __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: < string > (
                                                )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                let __field1 =
                                    match try!(visitor . visit :: < Range > (
                                               )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(1usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(Location{uri: __field0, range: __field1,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         -> ::std::result::Result<Location, __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0: Option<string> = None;
                                let mut __field1: Option<Range> = None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("uri"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          string > (  )));
                                        }
                                        __Field::__field1 => {
                                            if __field1.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("range"));
                                            }
                                            __field1 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Range > (  )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field ( "uri"
                                             )),
                                    };
                                let __field1 =
                                    match __field1 {
                                        Some(__field1) => __field1,
                                        None =>
                                        try!(visitor . missing_field ( "range"
                                             )),
                                    };
                                Ok(Location{uri: __field0, range: __field1,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["uri", "range"];
                    deserializer.deserialize_struct("Location", FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_Location: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for Location {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "Location" , 0 + 1 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "uri" , &self.uri ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "range" , &self.range ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
/// Represents a diagnostic, such as a compiler error or warning. 
/// Diagnostic objects are only valid in the scope of a resource.
#[derive(Debug, Clone)]
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
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_Diagnostic: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for Diagnostic {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<Diagnostic, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __ignore,
                    }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        1usize => { Ok(__Field::__field1) }
                                        2usize => { Ok(__Field::__field2) }
                                        3usize => { Ok(__Field::__field3) }
                                        4usize => { Ok(__Field::__field4) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "range" => { Ok(__Field::__field0) }
                                        "severity" => {
                                            Ok(__Field::__field1)
                                        }
                                        "code" => { Ok(__Field::__field2) }
                                        "source" => { Ok(__Field::__field3) }
                                        "message" => { Ok(__Field::__field4) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"range" => { Ok(__Field::__field0) }
                                        b"severity" => {
                                            Ok(__Field::__field1)
                                        }
                                        b"code" => { Ok(__Field::__field2) }
                                        b"source" => { Ok(__Field::__field3) }
                                        b"message" => {
                                            Ok(__Field::__field4)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        Diagnostic;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         -> ::std::result::Result<Diagnostic, __V::Error>
                         where __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: < Range > (
                                               )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                let __field1 =
                                    match try!(visitor . visit :: <
                                               Option<DiagnosticSeverity> > (
                                               )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(1usize));
                                        }
                                    };
                                let __field2 =
                                    match try!(visitor . visit :: <
                                               Option<number_or_string> > (
                                               )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(2usize));
                                        }
                                    };
                                let __field3 =
                                    match try!(visitor . visit :: <
                                               Option<string> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(3usize));
                                        }
                                    };
                                let __field4 =
                                    match try!(visitor . visit :: < string > (
                                                )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(4usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(Diagnostic{range: __field0,
                                              severity: __field1,
                                              code: __field2,
                                              source: __field3,
                                              message: __field4,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         -> ::std::result::Result<Diagnostic, __V::Error>
                         where __V: _serde::de::MapVisitor {
                            {
                                let mut __field0: Option<Range> = None;
                                let mut __field1:
                                        Option<Option<DiagnosticSeverity>> =
                                    None;
                                let mut __field2:
                                        Option<Option<number_or_string>> =
                                    None;
                                let mut __field3: Option<Option<string>> =
                                    None;
                                let mut __field4: Option<string> = None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("range"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Range > (  )));
                                        }
                                        __Field::__field1 => {
                                            if __field1.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("severity"));
                                            }
                                            __field1 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<DiagnosticSeverity>
                                                          > (  )));
                                        }
                                        __Field::__field2 => {
                                            if __field2.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("code"));
                                            }
                                            __field2 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<number_or_string>
                                                          > (  )));
                                        }
                                        __Field::__field3 => {
                                            if __field3.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("source"));
                                            }
                                            __field3 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<string> > (
                                                          )));
                                        }
                                        __Field::__field4 => {
                                            if __field4.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("message"));
                                            }
                                            __field4 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          string > (  )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field ( "range"
                                             )),
                                    };
                                let __field1 =
                                    match __field1 {
                                        Some(__field1) => __field1,
                                        None =>
                                        try!(visitor . missing_field (
                                             "severity" )),
                                    };
                                let __field2 =
                                    match __field2 {
                                        Some(__field2) => __field2,
                                        None =>
                                        try!(visitor . missing_field ( "code"
                                             )),
                                    };
                                let __field3 =
                                    match __field3 {
                                        Some(__field3) => __field3,
                                        None =>
                                        try!(visitor . missing_field (
                                             "source" )),
                                    };
                                let __field4 =
                                    match __field4 {
                                        Some(__field4) => __field4,
                                        None =>
                                        try!(visitor . missing_field (
                                             "message" )),
                                    };
                                Ok(Diagnostic{range: __field0,
                                              severity: __field1,
                                              code: __field2,
                                              source: __field3,
                                              message: __field4,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["range", "severity", "code", "source", "message"];
                    deserializer.deserialize_struct("Diagnostic", FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_Diagnostic: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for Diagnostic {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "Diagnostic" , 0 + 1 + 1 + 1 + 1 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "range" , &self.range ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "severity" , &self.severity ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "code" , &self.code ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "source" , &self.source ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "message" , &self.message ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
/// The protocol currently supports the following diagnostic severities:
#[derive(Debug, Copy, Clone)]
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
    Hint = 4,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_DiagnosticSeverity: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for DiagnosticSeverity {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<DiagnosticSeverity, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __ignore,
                    }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        1usize => { Ok(__Field::__field1) }
                                        2usize => { Ok(__Field::__field2) }
                                        3usize => { Ok(__Field::__field3) }
                                        _ => {
                                            Err(_serde::de::Error::invalid_value("expected a variant"))
                                        }
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "Error" => { Ok(__Field::__field0) }
                                        "Warning" => { Ok(__Field::__field1) }
                                        "Information" => {
                                            Ok(__Field::__field2)
                                        }
                                        "Hint" => { Ok(__Field::__field3) }
                                        _ =>
                                        Err(_serde::de::Error::unknown_variant(value)),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"Error" => { Ok(__Field::__field0) }
                                        b"Warning" => {
                                            Ok(__Field::__field1)
                                        }
                                        b"Information" => {
                                            Ok(__Field::__field2)
                                        }
                                        b"Hint" => { Ok(__Field::__field3) }
                                        _ => {
                                            let value =
                                                ::std::string::String::from_utf8_lossy(value);
                                            Err(_serde::de::Error::unknown_variant(&value))
                                        }
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer>
                     _serde::de::EnumVisitor for __Visitor<__D> {
                        type
                        Value
                        =
                        DiagnosticSeverity;
                        fn visit<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<DiagnosticSeverity,
                                                   __V::Error> where
                         __V: _serde::de::VariantVisitor {
                            match try!(visitor . visit_variant (  )) {
                                __Field::__field0 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(DiagnosticSeverity::Error)
                                    }
                                }
                                __Field::__field1 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(DiagnosticSeverity::Warning)
                                    }
                                }
                                __Field::__field2 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(DiagnosticSeverity::Information)
                                    }
                                }
                                __Field::__field3 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(DiagnosticSeverity::Hint)
                                    }
                                }
                                __Field::__ignore => {
                                    Err(_serde::de::Error::end_of_stream())
                                }
                            }
                        }
                    }
                    const VARIANTS: &'static [&'static str] =
                        &["Error", "Warning", "Information", "Hint"];
                    deserializer.deserialize_enum("DiagnosticSeverity",
                                                  VARIANTS,
                                                  __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_DiagnosticSeverity: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for DiagnosticSeverity {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                match *self {
                    DiagnosticSeverity::Error => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "DiagnosticSeverity",
                                                                        0usize,
                                                                        "Error")
                    }
                    DiagnosticSeverity::Warning => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "DiagnosticSeverity",
                                                                        1usize,
                                                                        "Warning")
                    }
                    DiagnosticSeverity::Information => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "DiagnosticSeverity",
                                                                        2usize,
                                                                        "Information")
                    }
                    DiagnosticSeverity::Hint => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "DiagnosticSeverity",
                                                                        3usize,
                                                                        "Hint")
                    }
                }
            }
        }
    };
/**
 Represents a reference to a command. Provides a title which will be used to represent a command in the UI. 
 Commands are identitifed using a string identifier and the protocol currently doesn't specify a set of 
 well known commands. So executing a command requires some tool extension code.
*/
#[derive(Debug, Clone)]
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
    pub arguments: Option<Vec<any>>,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_Command: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for Command {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<Command, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __field1, __field2, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        1usize => { Ok(__Field::__field1) }
                                        2usize => { Ok(__Field::__field2) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "title" => { Ok(__Field::__field0) }
                                        "command" => { Ok(__Field::__field1) }
                                        "arguments" => {
                                            Ok(__Field::__field2)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"title" => { Ok(__Field::__field0) }
                                        b"command" => {
                                            Ok(__Field::__field1)
                                        }
                                        b"arguments" => {
                                            Ok(__Field::__field2)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        Command;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         -> ::std::result::Result<Command, __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: < string > (
                                                )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                let __field1 =
                                    match try!(visitor . visit :: < string > (
                                                )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(1usize));
                                        }
                                    };
                                let __field2 =
                                    match try!(visitor . visit :: <
                                               Option<Vec<any>> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(2usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(Command{title: __field0,
                                           command: __field1,
                                           arguments: __field2,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         -> ::std::result::Result<Command, __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0: Option<string> = None;
                                let mut __field1: Option<string> = None;
                                let mut __field2: Option<Option<Vec<any>>> =
                                    None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("title"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          string > (  )));
                                        }
                                        __Field::__field1 => {
                                            if __field1.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("command"));
                                            }
                                            __field1 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          string > (  )));
                                        }
                                        __Field::__field2 => {
                                            if __field2.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("arguments"));
                                            }
                                            __field2 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<Vec<any>> > (
                                                           )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field ( "title"
                                             )),
                                    };
                                let __field1 =
                                    match __field1 {
                                        Some(__field1) => __field1,
                                        None =>
                                        try!(visitor . missing_field (
                                             "command" )),
                                    };
                                let __field2 =
                                    match __field2 {
                                        Some(__field2) => __field2,
                                        None =>
                                        try!(visitor . missing_field (
                                             "arguments" )),
                                    };
                                Ok(Command{title: __field0,
                                           command: __field1,
                                           arguments: __field2,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["title", "command", "arguments"];
                    deserializer.deserialize_struct("Command", FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_Command: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for Command {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "Command" , 0 + 1 + 1 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "title" , &self.title ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "command" , &self.command ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "arguments" , &self.arguments ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
/**
 * A textual edit applicable to a text document.
 */
#[derive(Debug, Clone)]
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
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_TextEdit: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for TextEdit {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<TextEdit, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __field1, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        1usize => { Ok(__Field::__field1) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "range" => { Ok(__Field::__field0) }
                                        "newText" => { Ok(__Field::__field1) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"range" => { Ok(__Field::__field0) }
                                        b"newText" => {
                                            Ok(__Field::__field1)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        TextEdit;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         -> ::std::result::Result<TextEdit, __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: < Range > (
                                               )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                let __field1 =
                                    match try!(visitor . visit :: < string > (
                                                )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(1usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(TextEdit{range: __field0,
                                            newText: __field1,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         -> ::std::result::Result<TextEdit, __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0: Option<Range> = None;
                                let mut __field1: Option<string> = None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("range"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Range > (  )));
                                        }
                                        __Field::__field1 => {
                                            if __field1.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("newText"));
                                            }
                                            __field1 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          string > (  )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field ( "range"
                                             )),
                                    };
                                let __field1 =
                                    match __field1 {
                                        Some(__field1) => __field1,
                                        None =>
                                        try!(visitor . missing_field (
                                             "newText" )),
                                    };
                                Ok(TextEdit{range: __field0,
                                            newText: __field1,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["range", "newText"];
                    deserializer.deserialize_struct("TextEdit", FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_TextEdit: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for TextEdit {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "TextEdit" , 0 + 1 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "range" , &self.range ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "newText" , &self.newText ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
/**
 * A workspace edit represents changes to many resources managed in the workspace.
 */
#[derive(Debug, Clone)]
pub struct WorkspaceEdit {
    /**
     * Holds changes to existing resources.
     */
    pub changes: HashMap<String, Vec<TextEdit>>,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_WorkspaceEdit: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for WorkspaceEdit {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<WorkspaceEdit, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "changes" => { Ok(__Field::__field0) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"changes" => {
                                            Ok(__Field::__field0)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        WorkspaceEdit;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         -> ::std::result::Result<WorkspaceEdit, __V::Error>
                         where __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: <
                                               HashMap<String, Vec<TextEdit>>
                                               > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(WorkspaceEdit{changes: __field0,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         -> ::std::result::Result<WorkspaceEdit, __V::Error>
                         where __V: _serde::de::MapVisitor {
                            {
                                let mut __field0:
                                        Option<HashMap<String,
                                                       Vec<TextEdit>>> = None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("changes"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          HashMap<String, Vec<TextEdit>>
                                                          > (  )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field (
                                             "changes" )),
                                    };
                                Ok(WorkspaceEdit{changes: __field0,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["changes"];
                    deserializer.deserialize_struct("WorkspaceEdit", FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_WorkspaceEdit: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for WorkspaceEdit {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "WorkspaceEdit" , 0 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "changes" , &self.changes ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
/**
 * Text documents are identified using a URI. On the protocol level, URIs are passed as strings. 
 * The corresponding JSON structure looks like this:
 */
#[derive(Debug, Clone)]
pub struct TextDocumentIdentifier {
    /**
     * The text document's URI.
     */
    pub uri: string,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_TextDocumentIdentifier: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for TextDocumentIdentifier {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<TextDocumentIdentifier, __D::Error>
             where __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "uri" => { Ok(__Field::__field0) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"uri" => { Ok(__Field::__field0) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        TextDocumentIdentifier;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<TextDocumentIdentifier,
                                                   __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: < string > (
                                                )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(TextDocumentIdentifier{uri: __field0,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<TextDocumentIdentifier,
                                                   __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0: Option<string> = None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("uri"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          string > (  )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field ( "uri"
                                             )),
                                    };
                                Ok(TextDocumentIdentifier{uri: __field0,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["uri"];
                    deserializer.deserialize_struct("TextDocumentIdentifier",
                                                    FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_TextDocumentIdentifier: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for TextDocumentIdentifier {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "TextDocumentIdentifier" , 0 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "uri" , &self.uri ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
/**
 * An item to transfer a text document from the client to the server. 
 */
#[derive(Debug, Clone)]
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
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_TextDocumentItem: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for TextDocumentItem {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<TextDocumentItem, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __ignore,
                    }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        1usize => { Ok(__Field::__field1) }
                                        2usize => { Ok(__Field::__field2) }
                                        3usize => { Ok(__Field::__field3) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "uri" => { Ok(__Field::__field0) }
                                        "languageId" => {
                                            Ok(__Field::__field1)
                                        }
                                        "version" => { Ok(__Field::__field2) }
                                        "text" => { Ok(__Field::__field3) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"uri" => { Ok(__Field::__field0) }
                                        b"languageId" => {
                                            Ok(__Field::__field1)
                                        }
                                        b"version" => {
                                            Ok(__Field::__field2)
                                        }
                                        b"text" => { Ok(__Field::__field3) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        TextDocumentItem;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<TextDocumentItem,
                                                   __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: < string > (
                                                )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                let __field1 =
                                    match try!(visitor . visit :: < string > (
                                                )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(1usize));
                                        }
                                    };
                                let __field2 =
                                    match try!(visitor . visit :: < number > (
                                                )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(2usize));
                                        }
                                    };
                                let __field3 =
                                    match try!(visitor . visit :: < string > (
                                                )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(3usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(TextDocumentItem{uri: __field0,
                                                    languageId: __field1,
                                                    version: __field2,
                                                    text: __field3,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<TextDocumentItem,
                                                   __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0: Option<string> = None;
                                let mut __field1: Option<string> = None;
                                let mut __field2: Option<number> = None;
                                let mut __field3: Option<string> = None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("uri"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          string > (  )));
                                        }
                                        __Field::__field1 => {
                                            if __field1.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("languageId"));
                                            }
                                            __field1 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          string > (  )));
                                        }
                                        __Field::__field2 => {
                                            if __field2.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("version"));
                                            }
                                            __field2 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          number > (  )));
                                        }
                                        __Field::__field3 => {
                                            if __field3.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("text"));
                                            }
                                            __field3 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          string > (  )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field ( "uri"
                                             )),
                                    };
                                let __field1 =
                                    match __field1 {
                                        Some(__field1) => __field1,
                                        None =>
                                        try!(visitor . missing_field (
                                             "languageId" )),
                                    };
                                let __field2 =
                                    match __field2 {
                                        Some(__field2) => __field2,
                                        None =>
                                        try!(visitor . missing_field (
                                             "version" )),
                                    };
                                let __field3 =
                                    match __field3 {
                                        Some(__field3) => __field3,
                                        None =>
                                        try!(visitor . missing_field ( "text"
                                             )),
                                    };
                                Ok(TextDocumentItem{uri: __field0,
                                                    languageId: __field1,
                                                    version: __field2,
                                                    text: __field3,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["uri", "languageId", "version", "text"];
                    deserializer.deserialize_struct("TextDocumentItem",
                                                    FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_TextDocumentItem: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for TextDocumentItem {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "TextDocumentItem" , 0 + 1 + 1 + 1 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "uri" , &self.uri ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "languageId" , &self.languageId ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "version" , &self.version ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "text" , &self.text ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
/**
 * An identifier to denote a specific version of a text document.
 */
#[derive(Debug, Clone)]
pub struct VersionedTextDocumentIdentifier {
    pub extends: TextDocumentIdentifier,
    /**
     * The version number of this document.
     */
    pub version: number,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_VersionedTextDocumentIdentifier: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for VersionedTextDocumentIdentifier {
            fn deserialize<__D>(deserializer: &mut __D)
             ->
                 ::std::result::Result<VersionedTextDocumentIdentifier,
                                       __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __field1, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        1usize => { Ok(__Field::__field1) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "extends" => { Ok(__Field::__field0) }
                                        "version" => { Ok(__Field::__field1) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"extends" => {
                                            Ok(__Field::__field0)
                                        }
                                        b"version" => {
                                            Ok(__Field::__field1)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        VersionedTextDocumentIdentifier;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<VersionedTextDocumentIdentifier,
                                                   __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: <
                                               TextDocumentIdentifier > (  ))
                                        {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                let __field1 =
                                    match try!(visitor . visit :: < number > (
                                                )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(1usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(VersionedTextDocumentIdentifier{extends:
                                                                       __field0,
                                                                   version:
                                                                       __field1,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<VersionedTextDocumentIdentifier,
                                                   __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0:
                                        Option<TextDocumentIdentifier> = None;
                                let mut __field1: Option<number> = None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("extends"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          TextDocumentIdentifier
                                                          > (  )));
                                        }
                                        __Field::__field1 => {
                                            if __field1.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("version"));
                                            }
                                            __field1 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          number > (  )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field (
                                             "extends" )),
                                    };
                                let __field1 =
                                    match __field1 {
                                        Some(__field1) => __field1,
                                        None =>
                                        try!(visitor . missing_field (
                                             "version" )),
                                    };
                                Ok(VersionedTextDocumentIdentifier{extends:
                                                                       __field0,
                                                                   version:
                                                                       __field1,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["extends", "version"];
                    deserializer.deserialize_struct("VersionedTextDocumentIdentifier",
                                                    FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_VersionedTextDocumentIdentifier: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for VersionedTextDocumentIdentifier {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "VersionedTextDocumentIdentifier" , 0 + 1 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "extends" , &self.extends ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "version" , &self.version ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
/**
 * A parameter literal used in requests to pass a text document and a position inside that document.
 */
#[derive(Debug, Clone)]
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
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_TextDocumentPositionParams: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for TextDocumentPositionParams {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<TextDocumentPositionParams, __D::Error>
             where __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __field1, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        1usize => { Ok(__Field::__field1) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "textDocument" => {
                                            Ok(__Field::__field0)
                                        }
                                        "position" => {
                                            Ok(__Field::__field1)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"textDocument" => {
                                            Ok(__Field::__field0)
                                        }
                                        b"position" => {
                                            Ok(__Field::__field1)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        TextDocumentPositionParams;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<TextDocumentPositionParams,
                                                   __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: <
                                               TextDocumentIdentifier > (  ))
                                        {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                let __field1 =
                                    match try!(visitor . visit :: < Position >
                                               (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(1usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(TextDocumentPositionParams{textDocument:
                                                                  __field0,
                                                              position:
                                                                  __field1,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<TextDocumentPositionParams,
                                                   __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0:
                                        Option<TextDocumentIdentifier> = None;
                                let mut __field1: Option<Position> = None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("textDocument"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          TextDocumentIdentifier
                                                          > (  )));
                                        }
                                        __Field::__field1 => {
                                            if __field1.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("position"));
                                            }
                                            __field1 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Position > (  )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field (
                                             "textDocument" )),
                                    };
                                let __field1 =
                                    match __field1 {
                                        Some(__field1) => __field1,
                                        None =>
                                        try!(visitor . missing_field (
                                             "position" )),
                                    };
                                Ok(TextDocumentPositionParams{textDocument:
                                                                  __field0,
                                                              position:
                                                                  __field1,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["textDocument", "position"];
                    deserializer.deserialize_struct("TextDocumentPositionParams",
                                                    FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_TextDocumentPositionParams: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for TextDocumentPositionParams {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "TextDocumentPositionParams" , 0 + 1 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "textDocument" , &self.textDocument
                             ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "position" , &self.position ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
/**
 * The initialize request is sent as the first request from the client to the server.
 */
pub fn request__Initialize(ls: Rc<LanguageServer>)
 ->
     FnLanguageServerRequest<InitializeParams, InitializeResult,
                             InitializeError> {
    request("initialize", Box::new(move |params| { ls.initialize(params) }))
}
#[derive(Debug, Clone)]
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
     * User provided initialization options.
     */
    pub initializationOptions: Option<any>,
    /**
     * The capabilities provided by the client (editor)
     */
    pub capabilities: ClientCapabilities,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_InitializeParams: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for InitializeParams {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<InitializeParams, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __ignore,
                    }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        1usize => { Ok(__Field::__field1) }
                                        2usize => { Ok(__Field::__field2) }
                                        3usize => { Ok(__Field::__field3) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "processId" => {
                                            Ok(__Field::__field0)
                                        }
                                        "rootPath" => {
                                            Ok(__Field::__field1)
                                        }
                                        "initializationOptions" => {
                                            Ok(__Field::__field2)
                                        }
                                        "capabilities" => {
                                            Ok(__Field::__field3)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"processId" => {
                                            Ok(__Field::__field0)
                                        }
                                        b"rootPath" => {
                                            Ok(__Field::__field1)
                                        }
                                        b"initializationOptions" => {
                                            Ok(__Field::__field2)
                                        }
                                        b"capabilities" => {
                                            Ok(__Field::__field3)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        InitializeParams;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<InitializeParams,
                                                   __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: < number > (
                                                )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                let __field1 =
                                    match try!(visitor . visit :: < string > (
                                                )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(1usize));
                                        }
                                    };
                                let __field2 =
                                    match try!(visitor . visit :: <
                                               Option<any> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(2usize));
                                        }
                                    };
                                let __field3 =
                                    match try!(visitor . visit :: <
                                               ClientCapabilities > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(3usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(InitializeParams{processId: __field0,
                                                    rootPath: __field1,
                                                    initializationOptions:
                                                        __field2,
                                                    capabilities: __field3,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<InitializeParams,
                                                   __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0: Option<number> = None;
                                let mut __field1: Option<string> = None;
                                let mut __field2: Option<Option<any>> = None;
                                let mut __field3: Option<ClientCapabilities> =
                                    None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("processId"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          number > (  )));
                                        }
                                        __Field::__field1 => {
                                            if __field1.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("rootPath"));
                                            }
                                            __field1 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          string > (  )));
                                        }
                                        __Field::__field2 => {
                                            if __field2.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("initializationOptions"));
                                            }
                                            __field2 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<any> > (
                                                          )));
                                        }
                                        __Field::__field3 => {
                                            if __field3.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("capabilities"));
                                            }
                                            __field3 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          ClientCapabilities >
                                                          (  )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field (
                                             "processId" )),
                                    };
                                let __field1 =
                                    match __field1 {
                                        Some(__field1) => __field1,
                                        None =>
                                        try!(visitor . missing_field (
                                             "rootPath" )),
                                    };
                                let __field2 =
                                    match __field2 {
                                        Some(__field2) => __field2,
                                        None =>
                                        try!(visitor . missing_field (
                                             "initializationOptions" )),
                                    };
                                let __field3 =
                                    match __field3 {
                                        Some(__field3) => __field3,
                                        None =>
                                        try!(visitor . missing_field (
                                             "capabilities" )),
                                    };
                                Ok(InitializeParams{processId: __field0,
                                                    rootPath: __field1,
                                                    initializationOptions:
                                                        __field2,
                                                    capabilities: __field3,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["processId", "rootPath", "initializationOptions",
                          "capabilities"];
                    deserializer.deserialize_struct("InitializeParams",
                                                    FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_InitializeParams: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for InitializeParams {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "InitializeParams" , 0 + 1 + 1 + 1 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "processId" , &self.processId ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "rootPath" , &self.rootPath ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "initializationOptions" ,
                             &self.initializationOptions ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "capabilities" , &self.capabilities
                             ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
/**
 * Where ClientCapabilities are currently empty:
 */
pub type ClientCapabilities = HashMap<String, Value>;
#[derive(Debug, Clone)]
pub struct InitializeResult {
    /**
     * The capabilities the language server provides.
     */
    pub capabilities: ServerCapabilities,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_InitializeResult: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for InitializeResult {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<InitializeResult, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "capabilities" => {
                                            Ok(__Field::__field0)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"capabilities" => {
                                            Ok(__Field::__field0)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        InitializeResult;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<InitializeResult,
                                                   __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: <
                                               ServerCapabilities > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(InitializeResult{capabilities: __field0,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<InitializeResult,
                                                   __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0: Option<ServerCapabilities> =
                                    None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("capabilities"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          ServerCapabilities >
                                                          (  )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field (
                                             "capabilities" )),
                                    };
                                Ok(InitializeResult{capabilities: __field0,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["capabilities"];
                    deserializer.deserialize_struct("InitializeResult",
                                                    FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_InitializeResult: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for InitializeResult {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "InitializeResult" , 0 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "capabilities" , &self.capabilities
                             ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
#[derive(Debug, Clone)]
pub struct InitializeError {
    /**
     * Indicates whether the client should retry to send the
     * initilize request after showing the message provided
     * in the ResponseError.
     */
    pub retry: boolean,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_InitializeError: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for InitializeError {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<InitializeError, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "retry" => { Ok(__Field::__field0) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"retry" => { Ok(__Field::__field0) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        InitializeError;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         -> ::std::result::Result<InitializeError, __V::Error>
                         where __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: < boolean >
                                               (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(InitializeError{retry: __field0,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         -> ::std::result::Result<InitializeError, __V::Error>
                         where __V: _serde::de::MapVisitor {
                            {
                                let mut __field0: Option<boolean> = None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("retry"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          boolean > (  )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field ( "retry"
                                             )),
                                    };
                                Ok(InitializeError{retry: __field0,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["retry"];
                    deserializer.deserialize_struct("InitializeError", FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_InitializeError: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for InitializeError {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "InitializeError" , 0 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "retry" , &self.retry ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
/**
 * Defines how the host (editor) should sync document changes to the language server.
 */
#[derive(Debug, Clone)]
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
    Incremental = 2,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_TextDocumentSyncKind: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for TextDocumentSyncKind {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<TextDocumentSyncKind, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __field1, __field2, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        1usize => { Ok(__Field::__field1) }
                                        2usize => { Ok(__Field::__field2) }
                                        _ => {
                                            Err(_serde::de::Error::invalid_value("expected a variant"))
                                        }
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "None" => { Ok(__Field::__field0) }
                                        "Full" => { Ok(__Field::__field1) }
                                        "Incremental" => {
                                            Ok(__Field::__field2)
                                        }
                                        _ =>
                                        Err(_serde::de::Error::unknown_variant(value)),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"None" => { Ok(__Field::__field0) }
                                        b"Full" => { Ok(__Field::__field1) }
                                        b"Incremental" => {
                                            Ok(__Field::__field2)
                                        }
                                        _ => {
                                            let value =
                                                ::std::string::String::from_utf8_lossy(value);
                                            Err(_serde::de::Error::unknown_variant(&value))
                                        }
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer>
                     _serde::de::EnumVisitor for __Visitor<__D> {
                        type
                        Value
                        =
                        TextDocumentSyncKind;
                        fn visit<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<TextDocumentSyncKind,
                                                   __V::Error> where
                         __V: _serde::de::VariantVisitor {
                            match try!(visitor . visit_variant (  )) {
                                __Field::__field0 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(TextDocumentSyncKind::None)
                                    }
                                }
                                __Field::__field1 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(TextDocumentSyncKind::Full)
                                    }
                                }
                                __Field::__field2 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(TextDocumentSyncKind::Incremental)
                                    }
                                }
                                __Field::__ignore => {
                                    Err(_serde::de::Error::end_of_stream())
                                }
                            }
                        }
                    }
                    const VARIANTS: &'static [&'static str] =
                        &["None", "Full", "Incremental"];
                    deserializer.deserialize_enum("TextDocumentSyncKind",
                                                  VARIANTS,
                                                  __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_TextDocumentSyncKind: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for TextDocumentSyncKind {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                match *self {
                    TextDocumentSyncKind::None => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "TextDocumentSyncKind",
                                                                        0usize,
                                                                        "None")
                    }
                    TextDocumentSyncKind::Full => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "TextDocumentSyncKind",
                                                                        1usize,
                                                                        "Full")
                    }
                    TextDocumentSyncKind::Incremental => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "TextDocumentSyncKind",
                                                                        2usize,
                                                                        "Incremental")
                    }
                }
            }
        }
    };
/**
 * Completion options.
 */
#[derive(Debug, Clone)]
pub struct CompletionOptions {
    /**
     * The server provides support to resolve additional information for a completion item.
     */
    pub resolveProvider: Option<boolean>,
    /**
     * The characters that trigger completion automatically.
     */
    pub triggerCharacters: Option<Vec<string>>,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_CompletionOptions: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for CompletionOptions {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<CompletionOptions, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __field1, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        1usize => { Ok(__Field::__field1) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "resolveProvider" => {
                                            Ok(__Field::__field0)
                                        }
                                        "triggerCharacters" => {
                                            Ok(__Field::__field1)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"resolveProvider" => {
                                            Ok(__Field::__field0)
                                        }
                                        b"triggerCharacters" => {
                                            Ok(__Field::__field1)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        CompletionOptions;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<CompletionOptions,
                                                   __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: <
                                               Option<boolean> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                let __field1 =
                                    match try!(visitor . visit :: <
                                               Option<Vec<string>> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(1usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(CompletionOptions{resolveProvider:
                                                         __field0,
                                                     triggerCharacters:
                                                         __field1,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<CompletionOptions,
                                                   __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0: Option<Option<boolean>> =
                                    None;
                                let mut __field1:
                                        Option<Option<Vec<string>>> = None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("resolveProvider"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<boolean> > (
                                                          )));
                                        }
                                        __Field::__field1 => {
                                            if __field1.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("triggerCharacters"));
                                            }
                                            __field1 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<Vec<string>>
                                                          > (  )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field (
                                             "resolveProvider" )),
                                    };
                                let __field1 =
                                    match __field1 {
                                        Some(__field1) => __field1,
                                        None =>
                                        try!(visitor . missing_field (
                                             "triggerCharacters" )),
                                    };
                                Ok(CompletionOptions{resolveProvider:
                                                         __field0,
                                                     triggerCharacters:
                                                         __field1,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["resolveProvider", "triggerCharacters"];
                    deserializer.deserialize_struct("CompletionOptions",
                                                    FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_CompletionOptions: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for CompletionOptions {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "CompletionOptions" , 0 + 1 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "resolveProvider" ,
                             &self.resolveProvider ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "triggerCharacters" ,
                             &self.triggerCharacters ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
/**
 * Signature help options.
 */
#[derive(Debug, Clone)]
pub struct SignatureHelpOptions {
    /**
     * The characters that trigger signature help automatically.
     */
    pub triggerCharacters: Option<Vec<string>>,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_SignatureHelpOptions: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for SignatureHelpOptions {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<SignatureHelpOptions, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "triggerCharacters" => {
                                            Ok(__Field::__field0)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"triggerCharacters" => {
                                            Ok(__Field::__field0)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        SignatureHelpOptions;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<SignatureHelpOptions,
                                                   __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: <
                                               Option<Vec<string>> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(SignatureHelpOptions{triggerCharacters:
                                                            __field0,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<SignatureHelpOptions,
                                                   __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0:
                                        Option<Option<Vec<string>>> = None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("triggerCharacters"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<Vec<string>>
                                                          > (  )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field (
                                             "triggerCharacters" )),
                                    };
                                Ok(SignatureHelpOptions{triggerCharacters:
                                                            __field0,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["triggerCharacters"];
                    deserializer.deserialize_struct("SignatureHelpOptions",
                                                    FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_SignatureHelpOptions: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for SignatureHelpOptions {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "SignatureHelpOptions" , 0 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "triggerCharacters" ,
                             &self.triggerCharacters ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
/**
 * Code Lens options.
 */
#[derive(Debug, Clone)]
pub struct CodeLensOptions {
    /**
     * Code lens has a resolve provider as well.
     */
    pub resolveProvider: Option<boolean>,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_CodeLensOptions: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for CodeLensOptions {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<CodeLensOptions, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "resolveProvider" => {
                                            Ok(__Field::__field0)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"resolveProvider" => {
                                            Ok(__Field::__field0)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        CodeLensOptions;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         -> ::std::result::Result<CodeLensOptions, __V::Error>
                         where __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: <
                                               Option<boolean> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(CodeLensOptions{resolveProvider:
                                                       __field0,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         -> ::std::result::Result<CodeLensOptions, __V::Error>
                         where __V: _serde::de::MapVisitor {
                            {
                                let mut __field0: Option<Option<boolean>> =
                                    None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("resolveProvider"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<boolean> > (
                                                          )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field (
                                             "resolveProvider" )),
                                    };
                                Ok(CodeLensOptions{resolveProvider:
                                                       __field0,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["resolveProvider"];
                    deserializer.deserialize_struct("CodeLensOptions", FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_CodeLensOptions: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for CodeLensOptions {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "CodeLensOptions" , 0 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "resolveProvider" ,
                             &self.resolveProvider ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
/**
 * Format document on type options
 */
#[derive(Debug, Clone)]
pub struct DocumentOnTypeFormattingOptions {
    /**
     * A character on which formatting should be triggered, like `}`.
     */
    pub firstTriggerCharacter: string,
    /**
     * More trigger characters.
     */
    pub triggerCharacters: Option<Vec<string>>,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_DocumentOnTypeFormattingOptions: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for DocumentOnTypeFormattingOptions {
            fn deserialize<__D>(deserializer: &mut __D)
             ->
                 ::std::result::Result<DocumentOnTypeFormattingOptions,
                                       __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __field1, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        1usize => { Ok(__Field::__field1) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "firstTriggerCharacter" => {
                                            Ok(__Field::__field0)
                                        }
                                        "triggerCharacters" => {
                                            Ok(__Field::__field1)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"firstTriggerCharacter" => {
                                            Ok(__Field::__field0)
                                        }
                                        b"triggerCharacters" => {
                                            Ok(__Field::__field1)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        DocumentOnTypeFormattingOptions;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<DocumentOnTypeFormattingOptions,
                                                   __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: < string > (
                                                )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                let __field1 =
                                    match try!(visitor . visit :: <
                                               Option<Vec<string>> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(1usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(DocumentOnTypeFormattingOptions{firstTriggerCharacter:
                                                                       __field0,
                                                                   triggerCharacters:
                                                                       __field1,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<DocumentOnTypeFormattingOptions,
                                                   __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0: Option<string> = None;
                                let mut __field1:
                                        Option<Option<Vec<string>>> = None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("firstTriggerCharacter"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          string > (  )));
                                        }
                                        __Field::__field1 => {
                                            if __field1.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("triggerCharacters"));
                                            }
                                            __field1 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<Vec<string>>
                                                          > (  )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field (
                                             "firstTriggerCharacter" )),
                                    };
                                let __field1 =
                                    match __field1 {
                                        Some(__field1) => __field1,
                                        None =>
                                        try!(visitor . missing_field (
                                             "triggerCharacters" )),
                                    };
                                Ok(DocumentOnTypeFormattingOptions{firstTriggerCharacter:
                                                                       __field0,
                                                                   triggerCharacters:
                                                                       __field1,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["firstTriggerCharacter", "triggerCharacters"];
                    deserializer.deserialize_struct("DocumentOnTypeFormattingOptions",
                                                    FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_DocumentOnTypeFormattingOptions: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for DocumentOnTypeFormattingOptions {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "DocumentOnTypeFormattingOptions" , 0 + 1 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "firstTriggerCharacter" ,
                             &self.firstTriggerCharacter ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "triggerCharacters" ,
                             &self.triggerCharacters ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
#[derive(Default, Debug, Clone)]
pub struct ServerCapabilities {
    /**
     * Defines how text documents are synced.
     */
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
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_ServerCapabilities: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for ServerCapabilities {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<ServerCapabilities, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __field6,
                        __field7,
                        __field8,
                        __field9,
                        __field10,
                        __field11,
                        __field12,
                        __field13,
                        __field14,
                        __ignore,
                    }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        1usize => { Ok(__Field::__field1) }
                                        2usize => { Ok(__Field::__field2) }
                                        3usize => { Ok(__Field::__field3) }
                                        4usize => { Ok(__Field::__field4) }
                                        5usize => { Ok(__Field::__field5) }
                                        6usize => { Ok(__Field::__field6) }
                                        7usize => { Ok(__Field::__field7) }
                                        8usize => { Ok(__Field::__field8) }
                                        9usize => { Ok(__Field::__field9) }
                                        10usize => { Ok(__Field::__field10) }
                                        11usize => { Ok(__Field::__field11) }
                                        12usize => { Ok(__Field::__field12) }
                                        13usize => { Ok(__Field::__field13) }
                                        14usize => { Ok(__Field::__field14) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "textDocumentSync" => {
                                            Ok(__Field::__field0)
                                        }
                                        "hoverProvider" => {
                                            Ok(__Field::__field1)
                                        }
                                        "completionProvider" => {
                                            Ok(__Field::__field2)
                                        }
                                        "signatureHelpProvider" => {
                                            Ok(__Field::__field3)
                                        }
                                        "definitionProvider" => {
                                            Ok(__Field::__field4)
                                        }
                                        "referencesProvider" => {
                                            Ok(__Field::__field5)
                                        }
                                        "documentHighlightProvider" => {
                                            Ok(__Field::__field6)
                                        }
                                        "documentSymbolProvider" => {
                                            Ok(__Field::__field7)
                                        }
                                        "workspaceSymbolProvider" => {
                                            Ok(__Field::__field8)
                                        }
                                        "codeActionProvider" => {
                                            Ok(__Field::__field9)
                                        }
                                        "codeLensProvider" => {
                                            Ok(__Field::__field10)
                                        }
                                        "documentFormattingProvider" => {
                                            Ok(__Field::__field11)
                                        }
                                        "documentRangeFormattingProvider" => {
                                            Ok(__Field::__field12)
                                        }
                                        "documentOnTypeFormattingProvider" =>
                                        {
                                            Ok(__Field::__field13)
                                        }
                                        "renameProvider" => {
                                            Ok(__Field::__field14)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"textDocumentSync" => {
                                            Ok(__Field::__field0)
                                        }
                                        b"hoverProvider" => {
                                            Ok(__Field::__field1)
                                        }
                                        b"completionProvider" => {
                                            Ok(__Field::__field2)
                                        }
                                        b"signatureHelpProvider" => {
                                            Ok(__Field::__field3)
                                        }
                                        b"definitionProvider" => {
                                            Ok(__Field::__field4)
                                        }
                                        b"referencesProvider" => {
                                            Ok(__Field::__field5)
                                        }
                                        b"documentHighlightProvider" => {
                                            Ok(__Field::__field6)
                                        }
                                        b"documentSymbolProvider" => {
                                            Ok(__Field::__field7)
                                        }
                                        b"workspaceSymbolProvider" => {
                                            Ok(__Field::__field8)
                                        }
                                        b"codeActionProvider" => {
                                            Ok(__Field::__field9)
                                        }
                                        b"codeLensProvider" => {
                                            Ok(__Field::__field10)
                                        }
                                        b"documentFormattingProvider" => {
                                            Ok(__Field::__field11)
                                        }
                                        b"documentRangeFormattingProvider" =>
                                        {
                                            Ok(__Field::__field12)
                                        }
                                        b"documentOnTypeFormattingProvider" =>
                                        {
                                            Ok(__Field::__field13)
                                        }
                                        b"renameProvider" => {
                                            Ok(__Field::__field14)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        ServerCapabilities;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<ServerCapabilities,
                                                   __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: <
                                               Option<TextDocumentSyncKind> >
                                               (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                let __field1 =
                                    match try!(visitor . visit :: <
                                               Option<boolean> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(1usize));
                                        }
                                    };
                                let __field2 =
                                    match try!(visitor . visit :: <
                                               Option<CompletionOptions> > (
                                               )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(2usize));
                                        }
                                    };
                                let __field3 =
                                    match try!(visitor . visit :: <
                                               Option<SignatureHelpOptions> >
                                               (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(3usize));
                                        }
                                    };
                                let __field4 =
                                    match try!(visitor . visit :: <
                                               Option<boolean> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(4usize));
                                        }
                                    };
                                let __field5 =
                                    match try!(visitor . visit :: <
                                               Option<boolean> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(5usize));
                                        }
                                    };
                                let __field6 =
                                    match try!(visitor . visit :: <
                                               Option<boolean> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(6usize));
                                        }
                                    };
                                let __field7 =
                                    match try!(visitor . visit :: <
                                               Option<boolean> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(7usize));
                                        }
                                    };
                                let __field8 =
                                    match try!(visitor . visit :: <
                                               Option<boolean> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(8usize));
                                        }
                                    };
                                let __field9 =
                                    match try!(visitor . visit :: <
                                               Option<boolean> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(9usize));
                                        }
                                    };
                                let __field10 =
                                    match try!(visitor . visit :: <
                                               Option<CodeLensOptions> > (  ))
                                        {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(10usize));
                                        }
                                    };
                                let __field11 =
                                    match try!(visitor . visit :: <
                                               Option<boolean> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(11usize));
                                        }
                                    };
                                let __field12 =
                                    match try!(visitor . visit :: <
                                               Option<boolean> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(12usize));
                                        }
                                    };
                                let __field13 =
                                    match try!(visitor . visit :: <
                                               Option<DocumentOnTypeFormattingOptions>
                                               > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(13usize));
                                        }
                                    };
                                let __field14 =
                                    match try!(visitor . visit :: <
                                               Option<boolean> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(14usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(ServerCapabilities{textDocumentSync:
                                                          __field0,
                                                      hoverProvider: __field1,
                                                      completionProvider:
                                                          __field2,
                                                      signatureHelpProvider:
                                                          __field3,
                                                      definitionProvider:
                                                          __field4,
                                                      referencesProvider:
                                                          __field5,
                                                      documentHighlightProvider:
                                                          __field6,
                                                      documentSymbolProvider:
                                                          __field7,
                                                      workspaceSymbolProvider:
                                                          __field8,
                                                      codeActionProvider:
                                                          __field9,
                                                      codeLensProvider:
                                                          __field10,
                                                      documentFormattingProvider:
                                                          __field11,
                                                      documentRangeFormattingProvider:
                                                          __field12,
                                                      documentOnTypeFormattingProvider:
                                                          __field13,
                                                      renameProvider:
                                                          __field14,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<ServerCapabilities,
                                                   __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0:
                                        Option<Option<TextDocumentSyncKind>> =
                                    None;
                                let mut __field1: Option<Option<boolean>> =
                                    None;
                                let mut __field2:
                                        Option<Option<CompletionOptions>> =
                                    None;
                                let mut __field3:
                                        Option<Option<SignatureHelpOptions>> =
                                    None;
                                let mut __field4: Option<Option<boolean>> =
                                    None;
                                let mut __field5: Option<Option<boolean>> =
                                    None;
                                let mut __field6: Option<Option<boolean>> =
                                    None;
                                let mut __field7: Option<Option<boolean>> =
                                    None;
                                let mut __field8: Option<Option<boolean>> =
                                    None;
                                let mut __field9: Option<Option<boolean>> =
                                    None;
                                let mut __field10:
                                        Option<Option<CodeLensOptions>> =
                                    None;
                                let mut __field11: Option<Option<boolean>> =
                                    None;
                                let mut __field12: Option<Option<boolean>> =
                                    None;
                                let mut __field13:
                                        Option<Option<DocumentOnTypeFormattingOptions>> =
                                    None;
                                let mut __field14: Option<Option<boolean>> =
                                    None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("textDocumentSync"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<TextDocumentSyncKind>
                                                          > (  )));
                                        }
                                        __Field::__field1 => {
                                            if __field1.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("hoverProvider"));
                                            }
                                            __field1 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<boolean> > (
                                                          )));
                                        }
                                        __Field::__field2 => {
                                            if __field2.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("completionProvider"));
                                            }
                                            __field2 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<CompletionOptions>
                                                          > (  )));
                                        }
                                        __Field::__field3 => {
                                            if __field3.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("signatureHelpProvider"));
                                            }
                                            __field3 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<SignatureHelpOptions>
                                                          > (  )));
                                        }
                                        __Field::__field4 => {
                                            if __field4.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("definitionProvider"));
                                            }
                                            __field4 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<boolean> > (
                                                          )));
                                        }
                                        __Field::__field5 => {
                                            if __field5.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("referencesProvider"));
                                            }
                                            __field5 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<boolean> > (
                                                          )));
                                        }
                                        __Field::__field6 => {
                                            if __field6.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("documentHighlightProvider"));
                                            }
                                            __field6 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<boolean> > (
                                                          )));
                                        }
                                        __Field::__field7 => {
                                            if __field7.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("documentSymbolProvider"));
                                            }
                                            __field7 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<boolean> > (
                                                          )));
                                        }
                                        __Field::__field8 => {
                                            if __field8.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("workspaceSymbolProvider"));
                                            }
                                            __field8 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<boolean> > (
                                                          )));
                                        }
                                        __Field::__field9 => {
                                            if __field9.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("codeActionProvider"));
                                            }
                                            __field9 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<boolean> > (
                                                          )));
                                        }
                                        __Field::__field10 => {
                                            if __field10.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("codeLensProvider"));
                                            }
                                            __field10 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<CodeLensOptions>
                                                          > (  )));
                                        }
                                        __Field::__field11 => {
                                            if __field11.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("documentFormattingProvider"));
                                            }
                                            __field11 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<boolean> > (
                                                          )));
                                        }
                                        __Field::__field12 => {
                                            if __field12.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("documentRangeFormattingProvider"));
                                            }
                                            __field12 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<boolean> > (
                                                          )));
                                        }
                                        __Field::__field13 => {
                                            if __field13.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("documentOnTypeFormattingProvider"));
                                            }
                                            __field13 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<DocumentOnTypeFormattingOptions>
                                                          > (  )));
                                        }
                                        __Field::__field14 => {
                                            if __field14.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("renameProvider"));
                                            }
                                            __field14 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<boolean> > (
                                                          )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field (
                                             "textDocumentSync" )),
                                    };
                                let __field1 =
                                    match __field1 {
                                        Some(__field1) => __field1,
                                        None =>
                                        try!(visitor . missing_field (
                                             "hoverProvider" )),
                                    };
                                let __field2 =
                                    match __field2 {
                                        Some(__field2) => __field2,
                                        None =>
                                        try!(visitor . missing_field (
                                             "completionProvider" )),
                                    };
                                let __field3 =
                                    match __field3 {
                                        Some(__field3) => __field3,
                                        None =>
                                        try!(visitor . missing_field (
                                             "signatureHelpProvider" )),
                                    };
                                let __field4 =
                                    match __field4 {
                                        Some(__field4) => __field4,
                                        None =>
                                        try!(visitor . missing_field (
                                             "definitionProvider" )),
                                    };
                                let __field5 =
                                    match __field5 {
                                        Some(__field5) => __field5,
                                        None =>
                                        try!(visitor . missing_field (
                                             "referencesProvider" )),
                                    };
                                let __field6 =
                                    match __field6 {
                                        Some(__field6) => __field6,
                                        None =>
                                        try!(visitor . missing_field (
                                             "documentHighlightProvider" )),
                                    };
                                let __field7 =
                                    match __field7 {
                                        Some(__field7) => __field7,
                                        None =>
                                        try!(visitor . missing_field (
                                             "documentSymbolProvider" )),
                                    };
                                let __field8 =
                                    match __field8 {
                                        Some(__field8) => __field8,
                                        None =>
                                        try!(visitor . missing_field (
                                             "workspaceSymbolProvider" )),
                                    };
                                let __field9 =
                                    match __field9 {
                                        Some(__field9) => __field9,
                                        None =>
                                        try!(visitor . missing_field (
                                             "codeActionProvider" )),
                                    };
                                let __field10 =
                                    match __field10 {
                                        Some(__field10) => __field10,
                                        None =>
                                        try!(visitor . missing_field (
                                             "codeLensProvider" )),
                                    };
                                let __field11 =
                                    match __field11 {
                                        Some(__field11) => __field11,
                                        None =>
                                        try!(visitor . missing_field (
                                             "documentFormattingProvider" )),
                                    };
                                let __field12 =
                                    match __field12 {
                                        Some(__field12) => __field12,
                                        None =>
                                        try!(visitor . missing_field (
                                             "documentRangeFormattingProvider"
                                             )),
                                    };
                                let __field13 =
                                    match __field13 {
                                        Some(__field13) => __field13,
                                        None =>
                                        try!(visitor . missing_field (
                                             "documentOnTypeFormattingProvider"
                                             )),
                                    };
                                let __field14 =
                                    match __field14 {
                                        Some(__field14) => __field14,
                                        None =>
                                        try!(visitor . missing_field (
                                             "renameProvider" )),
                                    };
                                Ok(ServerCapabilities{textDocumentSync:
                                                          __field0,
                                                      hoverProvider: __field1,
                                                      completionProvider:
                                                          __field2,
                                                      signatureHelpProvider:
                                                          __field3,
                                                      definitionProvider:
                                                          __field4,
                                                      referencesProvider:
                                                          __field5,
                                                      documentHighlightProvider:
                                                          __field6,
                                                      documentSymbolProvider:
                                                          __field7,
                                                      workspaceSymbolProvider:
                                                          __field8,
                                                      codeActionProvider:
                                                          __field9,
                                                      codeLensProvider:
                                                          __field10,
                                                      documentFormattingProvider:
                                                          __field11,
                                                      documentRangeFormattingProvider:
                                                          __field12,
                                                      documentOnTypeFormattingProvider:
                                                          __field13,
                                                      renameProvider:
                                                          __field14,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["textDocumentSync", "hoverProvider",
                          "completionProvider", "signatureHelpProvider",
                          "definitionProvider", "referencesProvider",
                          "documentHighlightProvider",
                          "documentSymbolProvider", "workspaceSymbolProvider",
                          "codeActionProvider", "codeLensProvider",
                          "documentFormattingProvider",
                          "documentRangeFormattingProvider",
                          "documentOnTypeFormattingProvider",
                          "renameProvider"];
                    deserializer.deserialize_struct("ServerCapabilities",
                                                    FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_ServerCapabilities: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for ServerCapabilities {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "ServerCapabilities" ,
                             0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1
                             ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "textDocumentSync" ,
                             &self.textDocumentSync ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "hoverProvider" ,
                             &self.hoverProvider ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "completionProvider" ,
                             &self.completionProvider ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "signatureHelpProvider" ,
                             &self.signatureHelpProvider ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "definitionProvider" ,
                             &self.definitionProvider ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "referencesProvider" ,
                             &self.referencesProvider ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "documentHighlightProvider" ,
                             &self.documentHighlightProvider ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "documentSymbolProvider" ,
                             &self.documentSymbolProvider ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "workspaceSymbolProvider" ,
                             &self.workspaceSymbolProvider ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "codeActionProvider" ,
                             &self.codeActionProvider ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "codeLensProvider" ,
                             &self.codeLensProvider ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "documentFormattingProvider" ,
                             &self.documentFormattingProvider ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "documentRangeFormattingProvider" ,
                             &self.documentRangeFormattingProvider ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "documentOnTypeFormattingProvider"
                             , &self.documentOnTypeFormattingProvider ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "renameProvider" ,
                             &self.renameProvider ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
/**
 * The shutdown request is sent from the client to the server. It asks the server to shut down,
 * but to not exit (otherwise the response might not be delivered correctly to the client).
 * There is a separate exit notification that asks the server to exit.
 */
pub fn request__Shutdown(ls: Rc<LanguageServer>)
 -> FnLanguageServerRequest<(), (), ()> {
    request("shutdown", Box::new(move |params| { ls.shutdown(params) }))
}
/**
 * A notification to ask the server to exit its process.
 */
pub fn notification__Exit(ls: Rc<LanguageServer>)
 -> FnLanguageServerNotification<()> {
    notification("exit", Box::new(move |params| { ls.exit(params) }))
}
/**
 * The show message notification is sent from a server to a client to ask the client to display a particular message
 * in the user interface.
 */
pub fn notification__ShowMessage(ls: Rc<LanguageServer>)
 -> FnLanguageServerNotification<ShowMessageParams> {
    notification("window/showMessage",
                 Box::new(move |params| { ls.showMessage(params) }))
}
#[derive(Debug, Clone)]
pub struct ShowMessageParams {
    /**
     * The message type. See {@link MessageType}.
     */
    pub type_: number,
    /**
     * The actual message.
     */
    pub message: string,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_ShowMessageParams: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for ShowMessageParams {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<ShowMessageParams, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __field1, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        1usize => { Ok(__Field::__field1) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "type" => { Ok(__Field::__field0) }
                                        "message" => { Ok(__Field::__field1) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"type" => { Ok(__Field::__field0) }
                                        b"message" => {
                                            Ok(__Field::__field1)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        ShowMessageParams;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<ShowMessageParams,
                                                   __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: < number > (
                                                )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                let __field1 =
                                    match try!(visitor . visit :: < string > (
                                                )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(1usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(ShowMessageParams{type_: __field0,
                                                     message: __field1,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<ShowMessageParams,
                                                   __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0: Option<number> = None;
                                let mut __field1: Option<string> = None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("type"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          number > (  )));
                                        }
                                        __Field::__field1 => {
                                            if __field1.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("message"));
                                            }
                                            __field1 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          string > (  )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field ( "type"
                                             )),
                                    };
                                let __field1 =
                                    match __field1 {
                                        Some(__field1) => __field1,
                                        None =>
                                        try!(visitor . missing_field (
                                             "message" )),
                                    };
                                Ok(ShowMessageParams{type_: __field0,
                                                     message: __field1,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["type_", "message"];
                    deserializer.deserialize_struct("ShowMessageParams",
                                                    FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_ShowMessageParams: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for ShowMessageParams {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "ShowMessageParams" , 0 + 1 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "type" , &self.type_ ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "message" , &self.message ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
#[derive(Debug, Clone)]
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
    Log = 4,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_MessageType: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for MessageType {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<MessageType, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __ignore,
                    }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        1usize => { Ok(__Field::__field1) }
                                        2usize => { Ok(__Field::__field2) }
                                        3usize => { Ok(__Field::__field3) }
                                        _ => {
                                            Err(_serde::de::Error::invalid_value("expected a variant"))
                                        }
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "Error" => { Ok(__Field::__field0) }
                                        "Warning" => { Ok(__Field::__field1) }
                                        "Info" => { Ok(__Field::__field2) }
                                        "Log" => { Ok(__Field::__field3) }
                                        _ =>
                                        Err(_serde::de::Error::unknown_variant(value)),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"Error" => { Ok(__Field::__field0) }
                                        b"Warning" => {
                                            Ok(__Field::__field1)
                                        }
                                        b"Info" => { Ok(__Field::__field2) }
                                        b"Log" => { Ok(__Field::__field3) }
                                        _ => {
                                            let value =
                                                ::std::string::String::from_utf8_lossy(value);
                                            Err(_serde::de::Error::unknown_variant(&value))
                                        }
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer>
                     _serde::de::EnumVisitor for __Visitor<__D> {
                        type
                        Value
                        =
                        MessageType;
                        fn visit<__V>(&mut self, mut visitor: __V)
                         -> ::std::result::Result<MessageType, __V::Error>
                         where __V: _serde::de::VariantVisitor {
                            match try!(visitor . visit_variant (  )) {
                                __Field::__field0 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(MessageType::Error)
                                    }
                                }
                                __Field::__field1 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(MessageType::Warning)
                                    }
                                }
                                __Field::__field2 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(MessageType::Info)
                                    }
                                }
                                __Field::__field3 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(MessageType::Log)
                                    }
                                }
                                __Field::__ignore => {
                                    Err(_serde::de::Error::end_of_stream())
                                }
                            }
                        }
                    }
                    const VARIANTS: &'static [&'static str] =
                        &["Error", "Warning", "Info", "Log"];
                    deserializer.deserialize_enum("MessageType", VARIANTS,
                                                  __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_MessageType: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for MessageType {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                match *self {
                    MessageType::Error => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "MessageType",
                                                                        0usize,
                                                                        "Error")
                    }
                    MessageType::Warning => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "MessageType",
                                                                        1usize,
                                                                        "Warning")
                    }
                    MessageType::Info => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "MessageType",
                                                                        2usize,
                                                                        "Info")
                    }
                    MessageType::Log => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "MessageType",
                                                                        3usize,
                                                                        "Log")
                    }
                }
            }
        }
    };
/**
 * The show message request is sent from a server to a client to ask the client to display a particular message
 * in the user interface. In addition to the show message notification the request allows to pass actions and to
 * wait for an answer from the client.
 */
pub fn request__ShowMessageRequest(ls: Rc<LanguageServer>)
 -> FnLanguageServerRequest<ShowMessageRequestParams, MessageActionItem, ()> {
    request("window/showMessageRequest",
            Box::new(move |params| { ls.showMessageRequest(params) }))
}
#[derive(Debug, Clone)]
pub struct ShowMessageRequestParams {
    /**
     * The message type. See {@link MessageType}
     */
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
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_ShowMessageRequestParams: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for ShowMessageRequestParams {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<ShowMessageRequestParams, __D::Error>
             where __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __field1, __field2, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        1usize => { Ok(__Field::__field1) }
                                        2usize => { Ok(__Field::__field2) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "type" => { Ok(__Field::__field0) }
                                        "message" => { Ok(__Field::__field1) }
                                        "actions" => { Ok(__Field::__field2) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"type" => { Ok(__Field::__field0) }
                                        b"message" => {
                                            Ok(__Field::__field1)
                                        }
                                        b"actions" => {
                                            Ok(__Field::__field2)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        ShowMessageRequestParams;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<ShowMessageRequestParams,
                                                   __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: < number > (
                                                )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                let __field1 =
                                    match try!(visitor . visit :: < string > (
                                                )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(1usize));
                                        }
                                    };
                                let __field2 =
                                    match try!(visitor . visit :: <
                                               Option<Vec<MessageActionItem>>
                                               > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(2usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(ShowMessageRequestParams{type_: __field0,
                                                            message: __field1,
                                                            actions:
                                                                __field2,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<ShowMessageRequestParams,
                                                   __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0: Option<number> = None;
                                let mut __field1: Option<string> = None;
                                let mut __field2:
                                        Option<Option<Vec<MessageActionItem>>> =
                                    None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("type"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          number > (  )));
                                        }
                                        __Field::__field1 => {
                                            if __field1.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("message"));
                                            }
                                            __field1 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          string > (  )));
                                        }
                                        __Field::__field2 => {
                                            if __field2.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("actions"));
                                            }
                                            __field2 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<Vec<MessageActionItem>>
                                                          > (  )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field ( "type"
                                             )),
                                    };
                                let __field1 =
                                    match __field1 {
                                        Some(__field1) => __field1,
                                        None =>
                                        try!(visitor . missing_field (
                                             "message" )),
                                    };
                                let __field2 =
                                    match __field2 {
                                        Some(__field2) => __field2,
                                        None =>
                                        try!(visitor . missing_field (
                                             "actions" )),
                                    };
                                Ok(ShowMessageRequestParams{type_: __field0,
                                                            message: __field1,
                                                            actions:
                                                                __field2,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["type_", "message", "actions"];
                    deserializer.deserialize_struct("ShowMessageRequestParams",
                                                    FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_ShowMessageRequestParams: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for ShowMessageRequestParams {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "ShowMessageRequestParams" , 0 + 1 + 1 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "type" , &self.type_ ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "message" , &self.message ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "actions" , &self.actions ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
#[derive(Debug, Clone)]
pub struct MessageActionItem {
    /**
     * A short title like 'Retry', 'Open Log' etc.
     */
    title: string,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_MessageActionItem: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for MessageActionItem {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<MessageActionItem, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "title" => { Ok(__Field::__field0) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"title" => { Ok(__Field::__field0) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        MessageActionItem;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<MessageActionItem,
                                                   __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: < string > (
                                                )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(MessageActionItem{title: __field0,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<MessageActionItem,
                                                   __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0: Option<string> = None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("title"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          string > (  )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field ( "title"
                                             )),
                                    };
                                Ok(MessageActionItem{title: __field0,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["title"];
                    deserializer.deserialize_struct("MessageActionItem",
                                                    FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_MessageActionItem: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for MessageActionItem {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "MessageActionItem" , 0 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "title" , &self.title ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
/**
 * The log message notification is sent from the server to the client to ask the client to log a particular message.
 */
pub fn notification__LogMessage(ls: Rc<LanguageServer>)
 -> FnLanguageServerNotification<LogMessageParams> {
    notification("window/logMessage",
                 Box::new(move |params| { ls.logMessage(params) }))
}
#[derive(Debug, Clone)]
pub struct LogMessageParams {
    /**
     * The message type. See {@link MessageType}
     */
    pub type_: number,
    /**
     * The actual message
     */
    pub message: string,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_LogMessageParams: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for LogMessageParams {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<LogMessageParams, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __field1, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        1usize => { Ok(__Field::__field1) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "type" => { Ok(__Field::__field0) }
                                        "message" => { Ok(__Field::__field1) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"type" => { Ok(__Field::__field0) }
                                        b"message" => {
                                            Ok(__Field::__field1)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        LogMessageParams;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<LogMessageParams,
                                                   __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: < number > (
                                                )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                let __field1 =
                                    match try!(visitor . visit :: < string > (
                                                )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(1usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(LogMessageParams{type_: __field0,
                                                    message: __field1,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<LogMessageParams,
                                                   __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0: Option<number> = None;
                                let mut __field1: Option<string> = None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("type"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          number > (  )));
                                        }
                                        __Field::__field1 => {
                                            if __field1.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("message"));
                                            }
                                            __field1 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          string > (  )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field ( "type"
                                             )),
                                    };
                                let __field1 =
                                    match __field1 {
                                        Some(__field1) => __field1,
                                        None =>
                                        try!(visitor . missing_field (
                                             "message" )),
                                    };
                                Ok(LogMessageParams{type_: __field0,
                                                    message: __field1,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["type_", "message"];
                    deserializer.deserialize_struct("LogMessageParams",
                                                    FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_LogMessageParams: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for LogMessageParams {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "LogMessageParams" , 0 + 1 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "type" , &self.type_ ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "message" , &self.message ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
/**
 * The telemetry notification is sent from the server to the client to ask the client to log a telemetry event.
 */
pub fn notification__TelemetryEvent(ls: Rc<LanguageServer>)
 -> FnLanguageServerNotification<any> {
    notification("telemetry/event",
                 Box::new(move |params| { ls.telemetryEvent(params) }))
}
/**
 * A notification sent from the client to the server to signal the change of configuration settings.
 */
pub fn notification__WorkspaceChangeConfiguration(ls: Rc<LanguageServer>)
 -> FnLanguageServerNotification<DidChangeConfigurationParams> {
    notification("workspace/didChangeConfiguration",
                 Box::new(move |params| {
                          ls.workspaceChangeConfiguration(params) }))
}
#[derive(Debug, Clone)]
pub struct DidChangeConfigurationParams {
    /**
     * The actual changed settings
     */
    pub settings: any,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_DidChangeConfigurationParams: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for DidChangeConfigurationParams {
            fn deserialize<__D>(deserializer: &mut __D)
             ->
                 ::std::result::Result<DidChangeConfigurationParams,
                                       __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "settings" => {
                                            Ok(__Field::__field0)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"settings" => {
                                            Ok(__Field::__field0)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        DidChangeConfigurationParams;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<DidChangeConfigurationParams,
                                                   __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: < any > (
                                               )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(DidChangeConfigurationParams{settings:
                                                                    __field0,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<DidChangeConfigurationParams,
                                                   __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0: Option<any> = None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("settings"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: < any
                                                          > (  )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field (
                                             "settings" )),
                                    };
                                Ok(DidChangeConfigurationParams{settings:
                                                                    __field0,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["settings"];
                    deserializer.deserialize_struct("DidChangeConfigurationParams",
                                                    FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_DidChangeConfigurationParams: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for DidChangeConfigurationParams {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "DidChangeConfigurationParams" , 0 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "settings" , &self.settings ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
/**
 * The document open notification is sent from the client to the server to signal newly opened text documents.
 * The document's truth is now managed by the client and the server must not try to read the document's truth
 * using the document's uri.
 */
pub fn notification__DidOpenTextDocument(ls: Rc<LanguageServer>)
 -> FnLanguageServerNotification<DidOpenTextDocumentParams> {
    notification("textDocument/didOpen",
                 Box::new(move |params| { ls.didOpenTextDocument(params) }))
}
#[derive(Debug, Clone)]
pub struct DidOpenTextDocumentParams {
    /**
     * The document that was opened.
     */
    pub textDocument: TextDocumentItem,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_DidOpenTextDocumentParams: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for DidOpenTextDocumentParams {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<DidOpenTextDocumentParams, __D::Error>
             where __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "textDocument" => {
                                            Ok(__Field::__field0)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"textDocument" => {
                                            Ok(__Field::__field0)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        DidOpenTextDocumentParams;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<DidOpenTextDocumentParams,
                                                   __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: <
                                               TextDocumentItem > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(DidOpenTextDocumentParams{textDocument:
                                                                 __field0,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<DidOpenTextDocumentParams,
                                                   __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0: Option<TextDocumentItem> =
                                    None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("textDocument"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          TextDocumentItem > (
                                                           )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field (
                                             "textDocument" )),
                                    };
                                Ok(DidOpenTextDocumentParams{textDocument:
                                                                 __field0,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["textDocument"];
                    deserializer.deserialize_struct("DidOpenTextDocumentParams",
                                                    FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_DidOpenTextDocumentParams: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for DidOpenTextDocumentParams {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "DidOpenTextDocumentParams" , 0 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "textDocument" , &self.textDocument
                             ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
/**
 * The document change notification is sent from the client to the server to signal changes to a text document.
 * In 2.0 the shape of the params has changed to include proper version numbers and language ids.
 */
pub fn notification__DidChangeTextDocument(ls: Rc<LanguageServer>)
 -> FnLanguageServerNotification<DidChangeTextDocumentParams> {
    notification("textDocument/didChange",
                 Box::new(move |params| { ls.didChangeTextDocument(params) }))
}
#[derive(Debug, Clone)]
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
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_DidChangeTextDocumentParams: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for DidChangeTextDocumentParams {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<DidChangeTextDocumentParams, __D::Error>
             where __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __field1, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        1usize => { Ok(__Field::__field1) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "textDocument" => {
                                            Ok(__Field::__field0)
                                        }
                                        "contentChanges" => {
                                            Ok(__Field::__field1)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"textDocument" => {
                                            Ok(__Field::__field0)
                                        }
                                        b"contentChanges" => {
                                            Ok(__Field::__field1)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        DidChangeTextDocumentParams;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<DidChangeTextDocumentParams,
                                                   __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: <
                                               VersionedTextDocumentIdentifier
                                               > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                let __field1 =
                                    match try!(visitor . visit :: <
                                               Vec<TextDocumentContentChangeEvent>
                                               > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(1usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(DidChangeTextDocumentParams{textDocument:
                                                                   __field0,
                                                               contentChanges:
                                                                   __field1,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<DidChangeTextDocumentParams,
                                                   __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0:
                                        Option<VersionedTextDocumentIdentifier> =
                                    None;
                                let mut __field1:
                                        Option<Vec<TextDocumentContentChangeEvent>> =
                                    None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("textDocument"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          VersionedTextDocumentIdentifier
                                                          > (  )));
                                        }
                                        __Field::__field1 => {
                                            if __field1.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("contentChanges"));
                                            }
                                            __field1 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Vec<TextDocumentContentChangeEvent>
                                                          > (  )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field (
                                             "textDocument" )),
                                    };
                                let __field1 =
                                    match __field1 {
                                        Some(__field1) => __field1,
                                        None =>
                                        try!(visitor . missing_field (
                                             "contentChanges" )),
                                    };
                                Ok(DidChangeTextDocumentParams{textDocument:
                                                                   __field0,
                                                               contentChanges:
                                                                   __field1,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["textDocument", "contentChanges"];
                    deserializer.deserialize_struct("DidChangeTextDocumentParams",
                                                    FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_DidChangeTextDocumentParams: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for DidChangeTextDocumentParams {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "DidChangeTextDocumentParams" , 0 + 1 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "textDocument" , &self.textDocument
                             ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "contentChanges" ,
                             &self.contentChanges ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
/**
 * An event describing a change to a text document. If range and rangeLength are omitted
 * the new text is considered to be the full content of the document.
 */
#[derive(Debug, Clone)]
pub struct TextDocumentContentChangeEvent {
    /**
     * The range of the document that changed.
     */
    pub range: Option<Range>,
    /**
     * The length of the range that got replaced.
     */
    pub rangeLength: Option<number>,
    /**
     * The new text of the document.
     */
    pub text: string,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_TextDocumentContentChangeEvent: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for TextDocumentContentChangeEvent {
            fn deserialize<__D>(deserializer: &mut __D)
             ->
                 ::std::result::Result<TextDocumentContentChangeEvent,
                                       __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __field1, __field2, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        1usize => { Ok(__Field::__field1) }
                                        2usize => { Ok(__Field::__field2) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "range" => { Ok(__Field::__field0) }
                                        "rangeLength" => {
                                            Ok(__Field::__field1)
                                        }
                                        "text" => { Ok(__Field::__field2) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"range" => { Ok(__Field::__field0) }
                                        b"rangeLength" => {
                                            Ok(__Field::__field1)
                                        }
                                        b"text" => { Ok(__Field::__field2) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        TextDocumentContentChangeEvent;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<TextDocumentContentChangeEvent,
                                                   __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: <
                                               Option<Range> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                let __field1 =
                                    match try!(visitor . visit :: <
                                               Option<number> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(1usize));
                                        }
                                    };
                                let __field2 =
                                    match try!(visitor . visit :: < string > (
                                                )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(2usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(TextDocumentContentChangeEvent{range:
                                                                      __field0,
                                                                  rangeLength:
                                                                      __field1,
                                                                  text:
                                                                      __field2,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<TextDocumentContentChangeEvent,
                                                   __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0: Option<Option<Range>> =
                                    None;
                                let mut __field1: Option<Option<number>> =
                                    None;
                                let mut __field2: Option<string> = None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("range"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<Range> > (
                                                          )));
                                        }
                                        __Field::__field1 => {
                                            if __field1.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("rangeLength"));
                                            }
                                            __field1 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<number> > (
                                                          )));
                                        }
                                        __Field::__field2 => {
                                            if __field2.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("text"));
                                            }
                                            __field2 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          string > (  )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field ( "range"
                                             )),
                                    };
                                let __field1 =
                                    match __field1 {
                                        Some(__field1) => __field1,
                                        None =>
                                        try!(visitor . missing_field (
                                             "rangeLength" )),
                                    };
                                let __field2 =
                                    match __field2 {
                                        Some(__field2) => __field2,
                                        None =>
                                        try!(visitor . missing_field ( "text"
                                             )),
                                    };
                                Ok(TextDocumentContentChangeEvent{range:
                                                                      __field0,
                                                                  rangeLength:
                                                                      __field1,
                                                                  text:
                                                                      __field2,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["range", "rangeLength", "text"];
                    deserializer.deserialize_struct("TextDocumentContentChangeEvent",
                                                    FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_TextDocumentContentChangeEvent: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for TextDocumentContentChangeEvent {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "TextDocumentContentChangeEvent" , 0 + 1 + 1 + 1
                             ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "range" , &self.range ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "rangeLength" , &self.rangeLength
                             ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "text" , &self.text ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
/**
 * The document close notification is sent from the client to the server when the document got closed in the client.
 * The document's truth now exists where the document's uri points to (e.g. if the document's uri is a file uri
 * the truth now exists on disk).
 */
pub fn notification__DidCloseTextDocument(ls: Rc<LanguageServer>)
 -> FnLanguageServerNotification<DidCloseTextDocumentParams> {
    notification("textDocument/didClose",
                 Box::new(move |params| { ls.didCloseTextDocument(params) }))
}
#[derive(Debug, Clone)]
pub struct DidCloseTextDocumentParams {
    /**
     * The document that was closed.
     */
    pub textDocument: TextDocumentIdentifier,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_DidCloseTextDocumentParams: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for DidCloseTextDocumentParams {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<DidCloseTextDocumentParams, __D::Error>
             where __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "textDocument" => {
                                            Ok(__Field::__field0)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"textDocument" => {
                                            Ok(__Field::__field0)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        DidCloseTextDocumentParams;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<DidCloseTextDocumentParams,
                                                   __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: <
                                               TextDocumentIdentifier > (  ))
                                        {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(DidCloseTextDocumentParams{textDocument:
                                                                  __field0,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<DidCloseTextDocumentParams,
                                                   __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0:
                                        Option<TextDocumentIdentifier> = None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("textDocument"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          TextDocumentIdentifier
                                                          > (  )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field (
                                             "textDocument" )),
                                    };
                                Ok(DidCloseTextDocumentParams{textDocument:
                                                                  __field0,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["textDocument"];
                    deserializer.deserialize_struct("DidCloseTextDocumentParams",
                                                    FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_DidCloseTextDocumentParams: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for DidCloseTextDocumentParams {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "DidCloseTextDocumentParams" , 0 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "textDocument" , &self.textDocument
                             ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
/**
 * The document save notification is sent from the client to the server when the document was saved in the client.
 */
pub fn notification__DidSaveTextDocument(ls: Rc<LanguageServer>)
 -> FnLanguageServerNotification<DidSaveTextDocumentParams> {
    notification("textDocument/didSave",
                 Box::new(move |params| { ls.didSaveTextDocument(params) }))
}
#[derive(Debug, Clone)]
pub struct DidSaveTextDocumentParams {
    /**
     * The document that was saved.
     */
    pub textDocument: TextDocumentIdentifier,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_DidSaveTextDocumentParams: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for DidSaveTextDocumentParams {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<DidSaveTextDocumentParams, __D::Error>
             where __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "textDocument" => {
                                            Ok(__Field::__field0)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"textDocument" => {
                                            Ok(__Field::__field0)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        DidSaveTextDocumentParams;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<DidSaveTextDocumentParams,
                                                   __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: <
                                               TextDocumentIdentifier > (  ))
                                        {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(DidSaveTextDocumentParams{textDocument:
                                                                 __field0,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<DidSaveTextDocumentParams,
                                                   __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0:
                                        Option<TextDocumentIdentifier> = None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("textDocument"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          TextDocumentIdentifier
                                                          > (  )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field (
                                             "textDocument" )),
                                    };
                                Ok(DidSaveTextDocumentParams{textDocument:
                                                                 __field0,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["textDocument"];
                    deserializer.deserialize_struct("DidSaveTextDocumentParams",
                                                    FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_DidSaveTextDocumentParams: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for DidSaveTextDocumentParams {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "DidSaveTextDocumentParams" , 0 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "textDocument" , &self.textDocument
                             ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
/**
 * The watched files notification is sent from the client to the server when the client detects changes to files
 * watched by the language client.
 */
pub fn notification__DidChangeWatchedFiles(ls: Rc<LanguageServer>)
 -> FnLanguageServerNotification<DidChangeWatchedFilesParams> {
    notification("workspace/didChangeWatchedFiles",
                 Box::new(move |params| { ls.didChangeWatchedFiles(params) }))
}
#[derive(Debug, Clone)]
pub struct DidChangeWatchedFilesParams {
    /**
     * The actual file events.
     */
    pub changes: Vec<FileEvent>,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_DidChangeWatchedFilesParams: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for DidChangeWatchedFilesParams {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<DidChangeWatchedFilesParams, __D::Error>
             where __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "changes" => { Ok(__Field::__field0) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"changes" => {
                                            Ok(__Field::__field0)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        DidChangeWatchedFilesParams;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<DidChangeWatchedFilesParams,
                                                   __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: <
                                               Vec<FileEvent> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(DidChangeWatchedFilesParams{changes:
                                                                   __field0,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<DidChangeWatchedFilesParams,
                                                   __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0: Option<Vec<FileEvent>> =
                                    None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("changes"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Vec<FileEvent> > (
                                                          )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field (
                                             "changes" )),
                                    };
                                Ok(DidChangeWatchedFilesParams{changes:
                                                                   __field0,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["changes"];
                    deserializer.deserialize_struct("DidChangeWatchedFilesParams",
                                                    FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_DidChangeWatchedFilesParams: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for DidChangeWatchedFilesParams {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "DidChangeWatchedFilesParams" , 0 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "changes" , &self.changes ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
/**
 * The file event type.
 */
#[derive(Debug, Clone)]
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
    Deleted = 3,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_FileChangeType: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for FileChangeType {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<FileChangeType, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __field1, __field2, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        1usize => { Ok(__Field::__field1) }
                                        2usize => { Ok(__Field::__field2) }
                                        _ => {
                                            Err(_serde::de::Error::invalid_value("expected a variant"))
                                        }
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "Created" => { Ok(__Field::__field0) }
                                        "Changed" => { Ok(__Field::__field1) }
                                        "Deleted" => { Ok(__Field::__field2) }
                                        _ =>
                                        Err(_serde::de::Error::unknown_variant(value)),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"Created" => {
                                            Ok(__Field::__field0)
                                        }
                                        b"Changed" => {
                                            Ok(__Field::__field1)
                                        }
                                        b"Deleted" => {
                                            Ok(__Field::__field2)
                                        }
                                        _ => {
                                            let value =
                                                ::std::string::String::from_utf8_lossy(value);
                                            Err(_serde::de::Error::unknown_variant(&value))
                                        }
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer>
                     _serde::de::EnumVisitor for __Visitor<__D> {
                        type
                        Value
                        =
                        FileChangeType;
                        fn visit<__V>(&mut self, mut visitor: __V)
                         -> ::std::result::Result<FileChangeType, __V::Error>
                         where __V: _serde::de::VariantVisitor {
                            match try!(visitor . visit_variant (  )) {
                                __Field::__field0 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(FileChangeType::Created)
                                    }
                                }
                                __Field::__field1 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(FileChangeType::Changed)
                                    }
                                }
                                __Field::__field2 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(FileChangeType::Deleted)
                                    }
                                }
                                __Field::__ignore => {
                                    Err(_serde::de::Error::end_of_stream())
                                }
                            }
                        }
                    }
                    const VARIANTS: &'static [&'static str] =
                        &["Created", "Changed", "Deleted"];
                    deserializer.deserialize_enum("FileChangeType", VARIANTS,
                                                  __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_FileChangeType: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for FileChangeType {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                match *self {
                    FileChangeType::Created => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "FileChangeType",
                                                                        0usize,
                                                                        "Created")
                    }
                    FileChangeType::Changed => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "FileChangeType",
                                                                        1usize,
                                                                        "Changed")
                    }
                    FileChangeType::Deleted => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "FileChangeType",
                                                                        2usize,
                                                                        "Deleted")
                    }
                }
            }
        }
    };
/**
 * An event describing a file change.
 */
#[derive(Debug, Clone)]
pub struct FileEvent {
    /**
     * The file's URI.
     */
    pub uri: string,
    /**
     * The change type.
     */
    pub type_: FileChangeType,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_FileEvent: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for FileEvent {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<FileEvent, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __field1, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        1usize => { Ok(__Field::__field1) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "uri" => { Ok(__Field::__field0) }
                                        "type" => { Ok(__Field::__field1) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"uri" => { Ok(__Field::__field0) }
                                        b"type" => { Ok(__Field::__field1) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        FileEvent;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         -> ::std::result::Result<FileEvent, __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: < string > (
                                                )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                let __field1 =
                                    match try!(visitor . visit :: <
                                               FileChangeType > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(1usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(FileEvent{uri: __field0, type_: __field1,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         -> ::std::result::Result<FileEvent, __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0: Option<string> = None;
                                let mut __field1: Option<FileChangeType> =
                                    None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("uri"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          string > (  )));
                                        }
                                        __Field::__field1 => {
                                            if __field1.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("type"));
                                            }
                                            __field1 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          FileChangeType > (
                                                          )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field ( "uri"
                                             )),
                                    };
                                let __field1 =
                                    match __field1 {
                                        Some(__field1) => __field1,
                                        None =>
                                        try!(visitor . missing_field ( "type"
                                             )),
                                    };
                                Ok(FileEvent{uri: __field0, type_: __field1,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["uri", "type_"];
                    deserializer.deserialize_struct("FileEvent", FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_FileEvent: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for FileEvent {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "FileEvent" , 0 + 1 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "uri" , &self.uri ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "type" , &self.type_ ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
/**
 * Diagnostics notification are sent from the server to the client to signal results of validation runs.
 */
pub fn notification__PublishDiagnostics(ls: Rc<LanguageServer>)
 -> FnLanguageServerNotification<PublishDiagnosticsParams> {
    notification("textDocument/publishDiagnostics",
                 Box::new(move |params| { ls.publishDiagnostics(params) }))
}
#[derive(Debug, Clone)]
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
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_PublishDiagnosticsParams: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for PublishDiagnosticsParams {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<PublishDiagnosticsParams, __D::Error>
             where __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __field1, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        1usize => { Ok(__Field::__field1) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "uri" => { Ok(__Field::__field0) }
                                        "diagnostics" => {
                                            Ok(__Field::__field1)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"uri" => { Ok(__Field::__field0) }
                                        b"diagnostics" => {
                                            Ok(__Field::__field1)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        PublishDiagnosticsParams;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<PublishDiagnosticsParams,
                                                   __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: < string > (
                                                )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                let __field1 =
                                    match try!(visitor . visit :: <
                                               Vec<Diagnostic> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(1usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(PublishDiagnosticsParams{uri: __field0,
                                                            diagnostics:
                                                                __field1,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<PublishDiagnosticsParams,
                                                   __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0: Option<string> = None;
                                let mut __field1: Option<Vec<Diagnostic>> =
                                    None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("uri"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          string > (  )));
                                        }
                                        __Field::__field1 => {
                                            if __field1.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("diagnostics"));
                                            }
                                            __field1 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Vec<Diagnostic> > (
                                                          )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field ( "uri"
                                             )),
                                    };
                                let __field1 =
                                    match __field1 {
                                        Some(__field1) => __field1,
                                        None =>
                                        try!(visitor . missing_field (
                                             "diagnostics" )),
                                    };
                                Ok(PublishDiagnosticsParams{uri: __field0,
                                                            diagnostics:
                                                                __field1,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["uri", "diagnostics"];
                    deserializer.deserialize_struct("PublishDiagnosticsParams",
                                                    FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_PublishDiagnosticsParams: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for PublishDiagnosticsParams {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "PublishDiagnosticsParams" , 0 + 1 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "uri" , &self.uri ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "diagnostics" , &self.diagnostics
                             ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
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
pub fn request__Completion(ls: Rc<LanguageServer>)
 -> FnLanguageServerRequest<TextDocumentPositionParams, CompletionList, ()> {
    request("textDocument/completion",
            Box::new(move |params| { ls.completion(params) }))
}
/**
 * Represents a collection of [completion items](#CompletionItem) to be presented
 * in the editor.
 */
#[derive(Debug, Clone)]
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
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_CompletionList: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for CompletionList {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<CompletionList, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __field1, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        1usize => { Ok(__Field::__field1) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "isIncomplete" => {
                                            Ok(__Field::__field0)
                                        }
                                        "items" => { Ok(__Field::__field1) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"isIncomplete" => {
                                            Ok(__Field::__field0)
                                        }
                                        b"items" => { Ok(__Field::__field1) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        CompletionList;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         -> ::std::result::Result<CompletionList, __V::Error>
                         where __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: < boolean >
                                               (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                let __field1 =
                                    match try!(visitor . visit :: <
                                               Vec<CompletionItem> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(1usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(CompletionList{isIncomplete: __field0,
                                                  items: __field1,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         -> ::std::result::Result<CompletionList, __V::Error>
                         where __V: _serde::de::MapVisitor {
                            {
                                let mut __field0: Option<boolean> = None;
                                let mut __field1:
                                        Option<Vec<CompletionItem>> = None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("isIncomplete"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          boolean > (  )));
                                        }
                                        __Field::__field1 => {
                                            if __field1.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("items"));
                                            }
                                            __field1 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Vec<CompletionItem>
                                                          > (  )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field (
                                             "isIncomplete" )),
                                    };
                                let __field1 =
                                    match __field1 {
                                        Some(__field1) => __field1,
                                        None =>
                                        try!(visitor . missing_field ( "items"
                                             )),
                                    };
                                Ok(CompletionList{isIncomplete: __field0,
                                                  items: __field1,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["isIncomplete", "items"];
                    deserializer.deserialize_struct("CompletionList", FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_CompletionList: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for CompletionList {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "CompletionList" , 0 + 1 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "isIncomplete" , &self.isIncomplete
                             ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "items" , &self.items ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
#[derive(Debug, Clone)]
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
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_CompletionItem: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for CompletionItem {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<CompletionItem, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __field6,
                        __field7,
                        __field8,
                        __ignore,
                    }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        1usize => { Ok(__Field::__field1) }
                                        2usize => { Ok(__Field::__field2) }
                                        3usize => { Ok(__Field::__field3) }
                                        4usize => { Ok(__Field::__field4) }
                                        5usize => { Ok(__Field::__field5) }
                                        6usize => { Ok(__Field::__field6) }
                                        7usize => { Ok(__Field::__field7) }
                                        8usize => { Ok(__Field::__field8) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "label" => { Ok(__Field::__field0) }
                                        "kind" => { Ok(__Field::__field1) }
                                        "detail" => { Ok(__Field::__field2) }
                                        "documentation" => {
                                            Ok(__Field::__field3)
                                        }
                                        "sortText" => {
                                            Ok(__Field::__field4)
                                        }
                                        "filterText" => {
                                            Ok(__Field::__field5)
                                        }
                                        "insertText" => {
                                            Ok(__Field::__field6)
                                        }
                                        "textEdit" => {
                                            Ok(__Field::__field7)
                                        }
                                        "data" => { Ok(__Field::__field8) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"label" => { Ok(__Field::__field0) }
                                        b"kind" => { Ok(__Field::__field1) }
                                        b"detail" => { Ok(__Field::__field2) }
                                        b"documentation" => {
                                            Ok(__Field::__field3)
                                        }
                                        b"sortText" => {
                                            Ok(__Field::__field4)
                                        }
                                        b"filterText" => {
                                            Ok(__Field::__field5)
                                        }
                                        b"insertText" => {
                                            Ok(__Field::__field6)
                                        }
                                        b"textEdit" => {
                                            Ok(__Field::__field7)
                                        }
                                        b"data" => { Ok(__Field::__field8) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        CompletionItem;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         -> ::std::result::Result<CompletionItem, __V::Error>
                         where __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: < string > (
                                                )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                let __field1 =
                                    match try!(visitor . visit :: <
                                               Option<CompletionItemKind> > (
                                               )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(1usize));
                                        }
                                    };
                                let __field2 =
                                    match try!(visitor . visit :: <
                                               Option<string> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(2usize));
                                        }
                                    };
                                let __field3 =
                                    match try!(visitor . visit :: <
                                               Option<string> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(3usize));
                                        }
                                    };
                                let __field4 =
                                    match try!(visitor . visit :: <
                                               Option<string> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(4usize));
                                        }
                                    };
                                let __field5 =
                                    match try!(visitor . visit :: <
                                               Option<string> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(5usize));
                                        }
                                    };
                                let __field6 =
                                    match try!(visitor . visit :: <
                                               Option<string> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(6usize));
                                        }
                                    };
                                let __field7 =
                                    match try!(visitor . visit :: <
                                               Option<TextEdit> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(7usize));
                                        }
                                    };
                                let __field8 =
                                    match try!(visitor . visit :: <
                                               Option<any> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(8usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(CompletionItem{label: __field0,
                                                  kind: __field1,
                                                  detail: __field2,
                                                  documentation: __field3,
                                                  sortText: __field4,
                                                  filterText: __field5,
                                                  insertText: __field6,
                                                  textEdit: __field7,
                                                  data: __field8,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         -> ::std::result::Result<CompletionItem, __V::Error>
                         where __V: _serde::de::MapVisitor {
                            {
                                let mut __field0: Option<string> = None;
                                let mut __field1:
                                        Option<Option<CompletionItemKind>> =
                                    None;
                                let mut __field2: Option<Option<string>> =
                                    None;
                                let mut __field3: Option<Option<string>> =
                                    None;
                                let mut __field4: Option<Option<string>> =
                                    None;
                                let mut __field5: Option<Option<string>> =
                                    None;
                                let mut __field6: Option<Option<string>> =
                                    None;
                                let mut __field7: Option<Option<TextEdit>> =
                                    None;
                                let mut __field8: Option<Option<any>> = None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("label"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          string > (  )));
                                        }
                                        __Field::__field1 => {
                                            if __field1.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("kind"));
                                            }
                                            __field1 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<CompletionItemKind>
                                                          > (  )));
                                        }
                                        __Field::__field2 => {
                                            if __field2.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("detail"));
                                            }
                                            __field2 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<string> > (
                                                          )));
                                        }
                                        __Field::__field3 => {
                                            if __field3.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("documentation"));
                                            }
                                            __field3 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<string> > (
                                                          )));
                                        }
                                        __Field::__field4 => {
                                            if __field4.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("sortText"));
                                            }
                                            __field4 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<string> > (
                                                          )));
                                        }
                                        __Field::__field5 => {
                                            if __field5.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("filterText"));
                                            }
                                            __field5 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<string> > (
                                                          )));
                                        }
                                        __Field::__field6 => {
                                            if __field6.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("insertText"));
                                            }
                                            __field6 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<string> > (
                                                          )));
                                        }
                                        __Field::__field7 => {
                                            if __field7.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("textEdit"));
                                            }
                                            __field7 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<TextEdit> > (
                                                           )));
                                        }
                                        __Field::__field8 => {
                                            if __field8.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("data"));
                                            }
                                            __field8 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<any> > (
                                                          )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field ( "label"
                                             )),
                                    };
                                let __field1 =
                                    match __field1 {
                                        Some(__field1) => __field1,
                                        None =>
                                        try!(visitor . missing_field ( "kind"
                                             )),
                                    };
                                let __field2 =
                                    match __field2 {
                                        Some(__field2) => __field2,
                                        None =>
                                        try!(visitor . missing_field (
                                             "detail" )),
                                    };
                                let __field3 =
                                    match __field3 {
                                        Some(__field3) => __field3,
                                        None =>
                                        try!(visitor . missing_field (
                                             "documentation" )),
                                    };
                                let __field4 =
                                    match __field4 {
                                        Some(__field4) => __field4,
                                        None =>
                                        try!(visitor . missing_field (
                                             "sortText" )),
                                    };
                                let __field5 =
                                    match __field5 {
                                        Some(__field5) => __field5,
                                        None =>
                                        try!(visitor . missing_field (
                                             "filterText" )),
                                    };
                                let __field6 =
                                    match __field6 {
                                        Some(__field6) => __field6,
                                        None =>
                                        try!(visitor . missing_field (
                                             "insertText" )),
                                    };
                                let __field7 =
                                    match __field7 {
                                        Some(__field7) => __field7,
                                        None =>
                                        try!(visitor . missing_field (
                                             "textEdit" )),
                                    };
                                let __field8 =
                                    match __field8 {
                                        Some(__field8) => __field8,
                                        None =>
                                        try!(visitor . missing_field ( "data"
                                             )),
                                    };
                                Ok(CompletionItem{label: __field0,
                                                  kind: __field1,
                                                  detail: __field2,
                                                  documentation: __field3,
                                                  sortText: __field4,
                                                  filterText: __field5,
                                                  insertText: __field6,
                                                  textEdit: __field7,
                                                  data: __field8,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["label", "kind", "detail", "documentation",
                          "sortText", "filterText", "insertText", "textEdit",
                          "data"];
                    deserializer.deserialize_struct("CompletionItem", FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_CompletionItem: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for CompletionItem {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "CompletionItem" ,
                             0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "label" , &self.label ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "kind" , &self.kind ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "detail" , &self.detail ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "documentation" ,
                             &self.documentation ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "sortText" , &self.sortText ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "filterText" , &self.filterText ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "insertText" , &self.insertText ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "textEdit" , &self.textEdit ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "data" , &self.data ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
/**
 * The kind of a completion entry.
 */
#[derive(Debug, Clone)]
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
    Reference = 18,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_CompletionItemKind: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for CompletionItemKind {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<CompletionItemKind, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __field6,
                        __field7,
                        __field8,
                        __field9,
                        __field10,
                        __field11,
                        __field12,
                        __field13,
                        __field14,
                        __field15,
                        __field16,
                        __field17,
                        __ignore,
                    }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        1usize => { Ok(__Field::__field1) }
                                        2usize => { Ok(__Field::__field2) }
                                        3usize => { Ok(__Field::__field3) }
                                        4usize => { Ok(__Field::__field4) }
                                        5usize => { Ok(__Field::__field5) }
                                        6usize => { Ok(__Field::__field6) }
                                        7usize => { Ok(__Field::__field7) }
                                        8usize => { Ok(__Field::__field8) }
                                        9usize => { Ok(__Field::__field9) }
                                        10usize => { Ok(__Field::__field10) }
                                        11usize => { Ok(__Field::__field11) }
                                        12usize => { Ok(__Field::__field12) }
                                        13usize => { Ok(__Field::__field13) }
                                        14usize => { Ok(__Field::__field14) }
                                        15usize => { Ok(__Field::__field15) }
                                        16usize => { Ok(__Field::__field16) }
                                        17usize => { Ok(__Field::__field17) }
                                        _ => {
                                            Err(_serde::de::Error::invalid_value("expected a variant"))
                                        }
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "Text" => { Ok(__Field::__field0) }
                                        "Method" => { Ok(__Field::__field1) }
                                        "Function" => {
                                            Ok(__Field::__field2)
                                        }
                                        "Constructor" => {
                                            Ok(__Field::__field3)
                                        }
                                        "Field" => { Ok(__Field::__field4) }
                                        "Variable" => {
                                            Ok(__Field::__field5)
                                        }
                                        "Class" => { Ok(__Field::__field6) }
                                        "Interface" => {
                                            Ok(__Field::__field7)
                                        }
                                        "Module" => { Ok(__Field::__field8) }
                                        "Property" => {
                                            Ok(__Field::__field9)
                                        }
                                        "Unit" => { Ok(__Field::__field10) }
                                        "Value" => { Ok(__Field::__field11) }
                                        "Enum" => { Ok(__Field::__field12) }
                                        "Keyword" => {
                                            Ok(__Field::__field13)
                                        }
                                        "Snippet" => {
                                            Ok(__Field::__field14)
                                        }
                                        "Color" => { Ok(__Field::__field15) }
                                        "File" => { Ok(__Field::__field16) }
                                        "Reference" => {
                                            Ok(__Field::__field17)
                                        }
                                        _ =>
                                        Err(_serde::de::Error::unknown_variant(value)),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"Text" => { Ok(__Field::__field0) }
                                        b"Method" => { Ok(__Field::__field1) }
                                        b"Function" => {
                                            Ok(__Field::__field2)
                                        }
                                        b"Constructor" => {
                                            Ok(__Field::__field3)
                                        }
                                        b"Field" => { Ok(__Field::__field4) }
                                        b"Variable" => {
                                            Ok(__Field::__field5)
                                        }
                                        b"Class" => { Ok(__Field::__field6) }
                                        b"Interface" => {
                                            Ok(__Field::__field7)
                                        }
                                        b"Module" => { Ok(__Field::__field8) }
                                        b"Property" => {
                                            Ok(__Field::__field9)
                                        }
                                        b"Unit" => { Ok(__Field::__field10) }
                                        b"Value" => { Ok(__Field::__field11) }
                                        b"Enum" => { Ok(__Field::__field12) }
                                        b"Keyword" => {
                                            Ok(__Field::__field13)
                                        }
                                        b"Snippet" => {
                                            Ok(__Field::__field14)
                                        }
                                        b"Color" => { Ok(__Field::__field15) }
                                        b"File" => { Ok(__Field::__field16) }
                                        b"Reference" => {
                                            Ok(__Field::__field17)
                                        }
                                        _ => {
                                            let value =
                                                ::std::string::String::from_utf8_lossy(value);
                                            Err(_serde::de::Error::unknown_variant(&value))
                                        }
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer>
                     _serde::de::EnumVisitor for __Visitor<__D> {
                        type
                        Value
                        =
                        CompletionItemKind;
                        fn visit<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<CompletionItemKind,
                                                   __V::Error> where
                         __V: _serde::de::VariantVisitor {
                            match try!(visitor . visit_variant (  )) {
                                __Field::__field0 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(CompletionItemKind::Text)
                                    }
                                }
                                __Field::__field1 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(CompletionItemKind::Method)
                                    }
                                }
                                __Field::__field2 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(CompletionItemKind::Function)
                                    }
                                }
                                __Field::__field3 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(CompletionItemKind::Constructor)
                                    }
                                }
                                __Field::__field4 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(CompletionItemKind::Field)
                                    }
                                }
                                __Field::__field5 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(CompletionItemKind::Variable)
                                    }
                                }
                                __Field::__field6 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(CompletionItemKind::Class)
                                    }
                                }
                                __Field::__field7 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(CompletionItemKind::Interface)
                                    }
                                }
                                __Field::__field8 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(CompletionItemKind::Module)
                                    }
                                }
                                __Field::__field9 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(CompletionItemKind::Property)
                                    }
                                }
                                __Field::__field10 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(CompletionItemKind::Unit)
                                    }
                                }
                                __Field::__field11 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(CompletionItemKind::Value)
                                    }
                                }
                                __Field::__field12 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(CompletionItemKind::Enum)
                                    }
                                }
                                __Field::__field13 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(CompletionItemKind::Keyword)
                                    }
                                }
                                __Field::__field14 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(CompletionItemKind::Snippet)
                                    }
                                }
                                __Field::__field15 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(CompletionItemKind::Color)
                                    }
                                }
                                __Field::__field16 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(CompletionItemKind::File)
                                    }
                                }
                                __Field::__field17 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(CompletionItemKind::Reference)
                                    }
                                }
                                __Field::__ignore => {
                                    Err(_serde::de::Error::end_of_stream())
                                }
                            }
                        }
                    }
                    const VARIANTS: &'static [&'static str] =
                        &["Text", "Method", "Function", "Constructor",
                          "Field", "Variable", "Class", "Interface", "Module",
                          "Property", "Unit", "Value", "Enum", "Keyword",
                          "Snippet", "Color", "File", "Reference"];
                    deserializer.deserialize_enum("CompletionItemKind",
                                                  VARIANTS,
                                                  __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_CompletionItemKind: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for CompletionItemKind {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                match *self {
                    CompletionItemKind::Text => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "CompletionItemKind",
                                                                        0usize,
                                                                        "Text")
                    }
                    CompletionItemKind::Method => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "CompletionItemKind",
                                                                        1usize,
                                                                        "Method")
                    }
                    CompletionItemKind::Function => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "CompletionItemKind",
                                                                        2usize,
                                                                        "Function")
                    }
                    CompletionItemKind::Constructor => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "CompletionItemKind",
                                                                        3usize,
                                                                        "Constructor")
                    }
                    CompletionItemKind::Field => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "CompletionItemKind",
                                                                        4usize,
                                                                        "Field")
                    }
                    CompletionItemKind::Variable => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "CompletionItemKind",
                                                                        5usize,
                                                                        "Variable")
                    }
                    CompletionItemKind::Class => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "CompletionItemKind",
                                                                        6usize,
                                                                        "Class")
                    }
                    CompletionItemKind::Interface => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "CompletionItemKind",
                                                                        7usize,
                                                                        "Interface")
                    }
                    CompletionItemKind::Module => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "CompletionItemKind",
                                                                        8usize,
                                                                        "Module")
                    }
                    CompletionItemKind::Property => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "CompletionItemKind",
                                                                        9usize,
                                                                        "Property")
                    }
                    CompletionItemKind::Unit => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "CompletionItemKind",
                                                                        10usize,
                                                                        "Unit")
                    }
                    CompletionItemKind::Value => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "CompletionItemKind",
                                                                        11usize,
                                                                        "Value")
                    }
                    CompletionItemKind::Enum => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "CompletionItemKind",
                                                                        12usize,
                                                                        "Enum")
                    }
                    CompletionItemKind::Keyword => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "CompletionItemKind",
                                                                        13usize,
                                                                        "Keyword")
                    }
                    CompletionItemKind::Snippet => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "CompletionItemKind",
                                                                        14usize,
                                                                        "Snippet")
                    }
                    CompletionItemKind::Color => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "CompletionItemKind",
                                                                        15usize,
                                                                        "Color")
                    }
                    CompletionItemKind::File => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "CompletionItemKind",
                                                                        16usize,
                                                                        "File")
                    }
                    CompletionItemKind::Reference => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "CompletionItemKind",
                                                                        17usize,
                                                                        "Reference")
                    }
                }
            }
        }
    };
/**
 * The request is sent from the client to the server to resolve additional information for a given completion item. 
 */
pub fn request__ResolveCompletionItem(ls: Rc<LanguageServer>)
 -> FnLanguageServerRequest<CompletionItem, CompletionItem, ()> {
    request("completionItem/resolve",
            Box::new(move |params| { ls.resolveCompletionItem(params) }))
}
/**
 * The hover request is sent from the client to the server to request hover information at a given text 
 * document position.
 */
pub fn request__Hover(ls: Rc<LanguageServer>)
 -> FnLanguageServerRequest<TextDocumentPositionParams, Hover, ()> {
    request("textDocument/hover",
            Box::new(move |params| { ls.hover(params) }))
}
/**
 * The result of a hover request.
 */
#[derive(Debug, Clone)]
pub struct Hover {
    /**
     * The hover's content
     */
    pub contents: Vec<MarkedString>,
    /**
     * An optional range
     */
    pub range: Option<Range>,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_Hover: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for Hover {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<Hover, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __field1, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        1usize => { Ok(__Field::__field1) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "contents" => {
                                            Ok(__Field::__field0)
                                        }
                                        "range" => { Ok(__Field::__field1) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"contents" => {
                                            Ok(__Field::__field0)
                                        }
                                        b"range" => { Ok(__Field::__field1) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        Hover;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         -> ::std::result::Result<Hover, __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: <
                                               Vec<MarkedString> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                let __field1 =
                                    match try!(visitor . visit :: <
                                               Option<Range> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(1usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(Hover{contents: __field0,
                                         range: __field1,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         -> ::std::result::Result<Hover, __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0: Option<Vec<MarkedString>> =
                                    None;
                                let mut __field1: Option<Option<Range>> =
                                    None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("contents"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Vec<MarkedString> >
                                                          (  )));
                                        }
                                        __Field::__field1 => {
                                            if __field1.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("range"));
                                            }
                                            __field1 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<Range> > (
                                                          )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field (
                                             "contents" )),
                                    };
                                let __field1 =
                                    match __field1 {
                                        Some(__field1) => __field1,
                                        None =>
                                        try!(visitor . missing_field ( "range"
                                             )),
                                    };
                                Ok(Hover{contents: __field0,
                                         range: __field1,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["contents", "range"];
                    deserializer.deserialize_struct("Hover", FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_Hover: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for Hover {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "Hover" , 0 + 1 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "contents" , &self.contents ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "range" , &self.range ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
pub type MarkedString = string;
/**
 * The signature help request is sent from the client to the server to request signature information at 
 * a given cursor position.
 */
pub fn request__SignatureHelp(ls: Rc<LanguageServer>)
 -> FnLanguageServerRequest<TextDocumentPositionParams, SignatureHelp, ()> {
    request("textDocument/signatureHelp",
            Box::new(move |params| { ls.signatureHelp(params) }))
}
/**
 * Signature help represents the signature of something
 * callable. There can be multiple signature but only one
 * active and only one active parameter.
 */
#[derive(Debug, Clone)]
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
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_SignatureHelp: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for SignatureHelp {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<SignatureHelp, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __field1, __field2, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        1usize => { Ok(__Field::__field1) }
                                        2usize => { Ok(__Field::__field2) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "signatures" => {
                                            Ok(__Field::__field0)
                                        }
                                        "activeSignature" => {
                                            Ok(__Field::__field1)
                                        }
                                        "activeParameter" => {
                                            Ok(__Field::__field2)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"signatures" => {
                                            Ok(__Field::__field0)
                                        }
                                        b"activeSignature" => {
                                            Ok(__Field::__field1)
                                        }
                                        b"activeParameter" => {
                                            Ok(__Field::__field2)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        SignatureHelp;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         -> ::std::result::Result<SignatureHelp, __V::Error>
                         where __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: <
                                               Vec<SignatureInformation> > (
                                               )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                let __field1 =
                                    match try!(visitor . visit :: <
                                               Option<number> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(1usize));
                                        }
                                    };
                                let __field2 =
                                    match try!(visitor . visit :: <
                                               Option<number> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(2usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(SignatureHelp{signatures: __field0,
                                                 activeSignature: __field1,
                                                 activeParameter: __field2,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         -> ::std::result::Result<SignatureHelp, __V::Error>
                         where __V: _serde::de::MapVisitor {
                            {
                                let mut __field0:
                                        Option<Vec<SignatureInformation>> =
                                    None;
                                let mut __field1: Option<Option<number>> =
                                    None;
                                let mut __field2: Option<Option<number>> =
                                    None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("signatures"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Vec<SignatureInformation>
                                                          > (  )));
                                        }
                                        __Field::__field1 => {
                                            if __field1.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("activeSignature"));
                                            }
                                            __field1 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<number> > (
                                                          )));
                                        }
                                        __Field::__field2 => {
                                            if __field2.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("activeParameter"));
                                            }
                                            __field2 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<number> > (
                                                          )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field (
                                             "signatures" )),
                                    };
                                let __field1 =
                                    match __field1 {
                                        Some(__field1) => __field1,
                                        None =>
                                        try!(visitor . missing_field (
                                             "activeSignature" )),
                                    };
                                let __field2 =
                                    match __field2 {
                                        Some(__field2) => __field2,
                                        None =>
                                        try!(visitor . missing_field (
                                             "activeParameter" )),
                                    };
                                Ok(SignatureHelp{signatures: __field0,
                                                 activeSignature: __field1,
                                                 activeParameter: __field2,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["signatures", "activeSignature", "activeParameter"];
                    deserializer.deserialize_struct("SignatureHelp", FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_SignatureHelp: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for SignatureHelp {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "SignatureHelp" , 0 + 1 + 1 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "signatures" , &self.signatures ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "activeSignature" ,
                             &self.activeSignature ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "activeParameter" ,
                             &self.activeParameter ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
/**
 * Represents the signature of something callable. A signature
 * can have a label, like a function-name, a doc-comment, and
 * a set of parameters.
 */
#[derive(Debug, Clone)]
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
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_SignatureInformation: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for SignatureInformation {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<SignatureInformation, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __field1, __field2, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        1usize => { Ok(__Field::__field1) }
                                        2usize => { Ok(__Field::__field2) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "label" => { Ok(__Field::__field0) }
                                        "documentation" => {
                                            Ok(__Field::__field1)
                                        }
                                        "parameters" => {
                                            Ok(__Field::__field2)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"label" => { Ok(__Field::__field0) }
                                        b"documentation" => {
                                            Ok(__Field::__field1)
                                        }
                                        b"parameters" => {
                                            Ok(__Field::__field2)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        SignatureInformation;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<SignatureInformation,
                                                   __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: < string > (
                                                )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                let __field1 =
                                    match try!(visitor . visit :: <
                                               Option<string> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(1usize));
                                        }
                                    };
                                let __field2 =
                                    match try!(visitor . visit :: <
                                               Option<Vec<ParameterInformation>>
                                               > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(2usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(SignatureInformation{label: __field0,
                                                        documentation:
                                                            __field1,
                                                        parameters:
                                                            __field2,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<SignatureInformation,
                                                   __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0: Option<string> = None;
                                let mut __field1: Option<Option<string>> =
                                    None;
                                let mut __field2:
                                        Option<Option<Vec<ParameterInformation>>> =
                                    None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("label"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          string > (  )));
                                        }
                                        __Field::__field1 => {
                                            if __field1.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("documentation"));
                                            }
                                            __field1 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<string> > (
                                                          )));
                                        }
                                        __Field::__field2 => {
                                            if __field2.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("parameters"));
                                            }
                                            __field2 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<Vec<ParameterInformation>>
                                                          > (  )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field ( "label"
                                             )),
                                    };
                                let __field1 =
                                    match __field1 {
                                        Some(__field1) => __field1,
                                        None =>
                                        try!(visitor . missing_field (
                                             "documentation" )),
                                    };
                                let __field2 =
                                    match __field2 {
                                        Some(__field2) => __field2,
                                        None =>
                                        try!(visitor . missing_field (
                                             "parameters" )),
                                    };
                                Ok(SignatureInformation{label: __field0,
                                                        documentation:
                                                            __field1,
                                                        parameters:
                                                            __field2,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["label", "documentation", "parameters"];
                    deserializer.deserialize_struct("SignatureInformation",
                                                    FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_SignatureInformation: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for SignatureInformation {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "SignatureInformation" , 0 + 1 + 1 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "label" , &self.label ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "documentation" ,
                             &self.documentation ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "parameters" , &self.parameters ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
/**
 * Represents a parameter of a callable-signature. A parameter can
 * have a label and a doc-comment.
 */
#[derive(Debug, Clone)]
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
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_ParameterInformation: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for ParameterInformation {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<ParameterInformation, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __field1, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        1usize => { Ok(__Field::__field1) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "label" => { Ok(__Field::__field0) }
                                        "documentation" => {
                                            Ok(__Field::__field1)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"label" => { Ok(__Field::__field0) }
                                        b"documentation" => {
                                            Ok(__Field::__field1)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        ParameterInformation;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<ParameterInformation,
                                                   __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: < string > (
                                                )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                let __field1 =
                                    match try!(visitor . visit :: <
                                               Option<string> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(1usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(ParameterInformation{label: __field0,
                                                        documentation:
                                                            __field1,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<ParameterInformation,
                                                   __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0: Option<string> = None;
                                let mut __field1: Option<Option<string>> =
                                    None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("label"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          string > (  )));
                                        }
                                        __Field::__field1 => {
                                            if __field1.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("documentation"));
                                            }
                                            __field1 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<string> > (
                                                          )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field ( "label"
                                             )),
                                    };
                                let __field1 =
                                    match __field1 {
                                        Some(__field1) => __field1,
                                        None =>
                                        try!(visitor . missing_field (
                                             "documentation" )),
                                    };
                                Ok(ParameterInformation{label: __field0,
                                                        documentation:
                                                            __field1,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["label", "documentation"];
                    deserializer.deserialize_struct("ParameterInformation",
                                                    FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_ParameterInformation: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for ParameterInformation {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "ParameterInformation" , 0 + 1 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "label" , &self.label ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "documentation" ,
                             &self.documentation ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
/**
 * The goto definition request is sent from the client to the server to resolve the definition location of 
 * a symbol at a given text document position.
 */
pub fn request__GotoDefinition(ls: Rc<LanguageServer>)
 -> FnLanguageServerRequest<TextDocumentPositionParams, Vec<Location>, ()> {
    request("textDocument/definition",
            Box::new(move |params| { ls.gotoDefinition(params) }))
}
/**
 * The references request is sent from the client to the server to resolve project-wide references for the 
 * symbol denoted by the given text document position.
 */
pub fn request__References(ls: Rc<LanguageServer>)
 -> FnLanguageServerRequest<ReferenceParams, Vec<Location>, ()> {
    request("textDocument/references",
            Box::new(move |params| { ls.references(params) }))
}
#[derive(Debug, Clone)]
pub struct ReferenceParams {
    pub context: ReferenceContext,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_ReferenceParams: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for ReferenceParams {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<ReferenceParams, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "context" => { Ok(__Field::__field0) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"context" => {
                                            Ok(__Field::__field0)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        ReferenceParams;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         -> ::std::result::Result<ReferenceParams, __V::Error>
                         where __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: <
                                               ReferenceContext > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(ReferenceParams{context: __field0,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         -> ::std::result::Result<ReferenceParams, __V::Error>
                         where __V: _serde::de::MapVisitor {
                            {
                                let mut __field0: Option<ReferenceContext> =
                                    None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("context"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          ReferenceContext > (
                                                           )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field (
                                             "context" )),
                                    };
                                Ok(ReferenceParams{context: __field0,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["context"];
                    deserializer.deserialize_struct("ReferenceParams", FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_ReferenceParams: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for ReferenceParams {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "ReferenceParams" , 0 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "context" , &self.context ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
#[derive(Debug, Clone)]
pub struct ReferenceContext {
    /**
     * Include the declaration of the current symbol.
     */
    pub includeDeclaration: boolean,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_ReferenceContext: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for ReferenceContext {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<ReferenceContext, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "includeDeclaration" => {
                                            Ok(__Field::__field0)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"includeDeclaration" => {
                                            Ok(__Field::__field0)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        ReferenceContext;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<ReferenceContext,
                                                   __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: < boolean >
                                               (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(ReferenceContext{includeDeclaration:
                                                        __field0,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<ReferenceContext,
                                                   __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0: Option<boolean> = None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("includeDeclaration"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          boolean > (  )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field (
                                             "includeDeclaration" )),
                                    };
                                Ok(ReferenceContext{includeDeclaration:
                                                        __field0,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["includeDeclaration"];
                    deserializer.deserialize_struct("ReferenceContext",
                                                    FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_ReferenceContext: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for ReferenceContext {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "ReferenceContext" , 0 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "includeDeclaration" ,
                             &self.includeDeclaration ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
/**
 The document highlight request is sent from the client to the server to resolve a document highlights 
 for a given text document position. 
 For programming languages this usually highlights all references to the symbol scoped to this file. 
 However we kept 'textDocument/documentHighlight' and 'textDocument/references' separate requests since 
 the first one is allowed to be more fuzzy. 
 Symbol matches usually have a DocumentHighlightKind of Read or Write whereas fuzzy or textual matches 
 use Textas the kind.
*/
pub fn request__DocumentHighlight(ls: Rc<LanguageServer>)
 ->
     FnLanguageServerRequest<TextDocumentPositionParams, DocumentHighlight,
                             ()> {
    request("textDocument/documentHighlight",
            Box::new(move |params| { ls.documentHighlight(params) }))
}
/**
 * A document highlight is a range inside a text document which deserves
 * special attention. Usually a document highlight is visualized by changing
 * the background color of its range.
 */
#[derive(Debug, Clone)]
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
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_DocumentHighlight: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for DocumentHighlight {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<DocumentHighlight, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __field1, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        1usize => { Ok(__Field::__field1) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "range" => { Ok(__Field::__field0) }
                                        "kind" => { Ok(__Field::__field1) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"range" => { Ok(__Field::__field0) }
                                        b"kind" => { Ok(__Field::__field1) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        DocumentHighlight;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<DocumentHighlight,
                                                   __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: < Range > (
                                               )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                let __field1 =
                                    match try!(visitor . visit :: <
                                               Option<number> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(1usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(DocumentHighlight{range: __field0,
                                                     kind: __field1,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<DocumentHighlight,
                                                   __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0: Option<Range> = None;
                                let mut __field1: Option<Option<number>> =
                                    None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("range"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Range > (  )));
                                        }
                                        __Field::__field1 => {
                                            if __field1.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("kind"));
                                            }
                                            __field1 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<number> > (
                                                          )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field ( "range"
                                             )),
                                    };
                                let __field1 =
                                    match __field1 {
                                        Some(__field1) => __field1,
                                        None =>
                                        try!(visitor . missing_field ( "kind"
                                             )),
                                    };
                                Ok(DocumentHighlight{range: __field0,
                                                     kind: __field1,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["range", "kind"];
                    deserializer.deserialize_struct("DocumentHighlight",
                                                    FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_DocumentHighlight: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for DocumentHighlight {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "DocumentHighlight" , 0 + 1 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "range" , &self.range ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "kind" , &self.kind ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
/**
 * A document highlight kind.
 */
#[derive(Debug, Clone)]
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
    Write = 3,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_DocumentHighlightKind: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for DocumentHighlightKind {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<DocumentHighlightKind, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __field1, __field2, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        1usize => { Ok(__Field::__field1) }
                                        2usize => { Ok(__Field::__field2) }
                                        _ => {
                                            Err(_serde::de::Error::invalid_value("expected a variant"))
                                        }
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "Text" => { Ok(__Field::__field0) }
                                        "Read" => { Ok(__Field::__field1) }
                                        "Write" => { Ok(__Field::__field2) }
                                        _ =>
                                        Err(_serde::de::Error::unknown_variant(value)),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"Text" => { Ok(__Field::__field0) }
                                        b"Read" => { Ok(__Field::__field1) }
                                        b"Write" => { Ok(__Field::__field2) }
                                        _ => {
                                            let value =
                                                ::std::string::String::from_utf8_lossy(value);
                                            Err(_serde::de::Error::unknown_variant(&value))
                                        }
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer>
                     _serde::de::EnumVisitor for __Visitor<__D> {
                        type
                        Value
                        =
                        DocumentHighlightKind;
                        fn visit<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<DocumentHighlightKind,
                                                   __V::Error> where
                         __V: _serde::de::VariantVisitor {
                            match try!(visitor . visit_variant (  )) {
                                __Field::__field0 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(DocumentHighlightKind::Text)
                                    }
                                }
                                __Field::__field1 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(DocumentHighlightKind::Read)
                                    }
                                }
                                __Field::__field2 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(DocumentHighlightKind::Write)
                                    }
                                }
                                __Field::__ignore => {
                                    Err(_serde::de::Error::end_of_stream())
                                }
                            }
                        }
                    }
                    const VARIANTS: &'static [&'static str] =
                        &["Text", "Read", "Write"];
                    deserializer.deserialize_enum("DocumentHighlightKind",
                                                  VARIANTS,
                                                  __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_DocumentHighlightKind: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for DocumentHighlightKind {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                match *self {
                    DocumentHighlightKind::Text => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "DocumentHighlightKind",
                                                                        0usize,
                                                                        "Text")
                    }
                    DocumentHighlightKind::Read => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "DocumentHighlightKind",
                                                                        1usize,
                                                                        "Read")
                    }
                    DocumentHighlightKind::Write => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "DocumentHighlightKind",
                                                                        2usize,
                                                                        "Write")
                    }
                }
            }
        }
    };
/**
 * The document symbol request is sent from the client to the server to list all symbols found in a given 
 * text document.
 */
pub fn request__DocumentSymbols(ls: Rc<LanguageServer>)
 ->
     FnLanguageServerRequest<DocumentSymbolParams, Vec<SymbolInformation>,
                             ()> {
    request("textDocument/documentSymbol",
            Box::new(move |params| { ls.documentSymbols(params) }))
}
#[derive(Debug, Clone)]
pub struct DocumentSymbolParams {
    /**
     * The text document.
     */
    pub textDocument: TextDocumentIdentifier,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_DocumentSymbolParams: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for DocumentSymbolParams {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<DocumentSymbolParams, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "textDocument" => {
                                            Ok(__Field::__field0)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"textDocument" => {
                                            Ok(__Field::__field0)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        DocumentSymbolParams;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<DocumentSymbolParams,
                                                   __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: <
                                               TextDocumentIdentifier > (  ))
                                        {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(DocumentSymbolParams{textDocument:
                                                            __field0,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<DocumentSymbolParams,
                                                   __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0:
                                        Option<TextDocumentIdentifier> = None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("textDocument"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          TextDocumentIdentifier
                                                          > (  )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field (
                                             "textDocument" )),
                                    };
                                Ok(DocumentSymbolParams{textDocument:
                                                            __field0,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["textDocument"];
                    deserializer.deserialize_struct("DocumentSymbolParams",
                                                    FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_DocumentSymbolParams: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for DocumentSymbolParams {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "DocumentSymbolParams" , 0 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "textDocument" , &self.textDocument
                             ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
/**
 * Represents information about programming constructs like variables, classes,
 * interfaces etc.
 */
#[derive(Debug, Clone)]
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
    pub containerName: Option<string>,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_SymbolInformation: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for SymbolInformation {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<SymbolInformation, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __ignore,
                    }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        1usize => { Ok(__Field::__field1) }
                                        2usize => { Ok(__Field::__field2) }
                                        3usize => { Ok(__Field::__field3) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "name" => { Ok(__Field::__field0) }
                                        "kind" => { Ok(__Field::__field1) }
                                        "location" => {
                                            Ok(__Field::__field2)
                                        }
                                        "containerName" => {
                                            Ok(__Field::__field3)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"name" => { Ok(__Field::__field0) }
                                        b"kind" => { Ok(__Field::__field1) }
                                        b"location" => {
                                            Ok(__Field::__field2)
                                        }
                                        b"containerName" => {
                                            Ok(__Field::__field3)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        SymbolInformation;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<SymbolInformation,
                                                   __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: < string > (
                                                )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                let __field1 =
                                    match try!(visitor . visit :: < number > (
                                                )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(1usize));
                                        }
                                    };
                                let __field2 =
                                    match try!(visitor . visit :: < Location >
                                               (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(2usize));
                                        }
                                    };
                                let __field3 =
                                    match try!(visitor . visit :: <
                                               Option<string> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(3usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(SymbolInformation{name: __field0,
                                                     kind: __field1,
                                                     location: __field2,
                                                     containerName:
                                                         __field3,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<SymbolInformation,
                                                   __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0: Option<string> = None;
                                let mut __field1: Option<number> = None;
                                let mut __field2: Option<Location> = None;
                                let mut __field3: Option<Option<string>> =
                                    None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("name"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          string > (  )));
                                        }
                                        __Field::__field1 => {
                                            if __field1.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("kind"));
                                            }
                                            __field1 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          number > (  )));
                                        }
                                        __Field::__field2 => {
                                            if __field2.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("location"));
                                            }
                                            __field2 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Location > (  )));
                                        }
                                        __Field::__field3 => {
                                            if __field3.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("containerName"));
                                            }
                                            __field3 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<string> > (
                                                          )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field ( "name"
                                             )),
                                    };
                                let __field1 =
                                    match __field1 {
                                        Some(__field1) => __field1,
                                        None =>
                                        try!(visitor . missing_field ( "kind"
                                             )),
                                    };
                                let __field2 =
                                    match __field2 {
                                        Some(__field2) => __field2,
                                        None =>
                                        try!(visitor . missing_field (
                                             "location" )),
                                    };
                                let __field3 =
                                    match __field3 {
                                        Some(__field3) => __field3,
                                        None =>
                                        try!(visitor . missing_field (
                                             "containerName" )),
                                    };
                                Ok(SymbolInformation{name: __field0,
                                                     kind: __field1,
                                                     location: __field2,
                                                     containerName:
                                                         __field3,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["name", "kind", "location", "containerName"];
                    deserializer.deserialize_struct("SymbolInformation",
                                                    FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_SymbolInformation: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for SymbolInformation {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "SymbolInformation" , 0 + 1 + 1 + 1 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "name" , &self.name ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "kind" , &self.kind ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "location" , &self.location ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "containerName" ,
                             &self.containerName ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
/**
 * A symbol kind.
 */
#[derive(Debug, Copy, Clone)]
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
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_SymbolKind: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for SymbolKind {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<SymbolKind, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __field6,
                        __field7,
                        __field8,
                        __field9,
                        __field10,
                        __field11,
                        __field12,
                        __field13,
                        __field14,
                        __field15,
                        __field16,
                        __field17,
                        __ignore,
                    }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        1usize => { Ok(__Field::__field1) }
                                        2usize => { Ok(__Field::__field2) }
                                        3usize => { Ok(__Field::__field3) }
                                        4usize => { Ok(__Field::__field4) }
                                        5usize => { Ok(__Field::__field5) }
                                        6usize => { Ok(__Field::__field6) }
                                        7usize => { Ok(__Field::__field7) }
                                        8usize => { Ok(__Field::__field8) }
                                        9usize => { Ok(__Field::__field9) }
                                        10usize => { Ok(__Field::__field10) }
                                        11usize => { Ok(__Field::__field11) }
                                        12usize => { Ok(__Field::__field12) }
                                        13usize => { Ok(__Field::__field13) }
                                        14usize => { Ok(__Field::__field14) }
                                        15usize => { Ok(__Field::__field15) }
                                        16usize => { Ok(__Field::__field16) }
                                        17usize => { Ok(__Field::__field17) }
                                        _ => {
                                            Err(_serde::de::Error::invalid_value("expected a variant"))
                                        }
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "File" => { Ok(__Field::__field0) }
                                        "Module" => { Ok(__Field::__field1) }
                                        "Namespace" => {
                                            Ok(__Field::__field2)
                                        }
                                        "Package" => { Ok(__Field::__field3) }
                                        "Class" => { Ok(__Field::__field4) }
                                        "Method" => { Ok(__Field::__field5) }
                                        "Property" => {
                                            Ok(__Field::__field6)
                                        }
                                        "Field" => { Ok(__Field::__field7) }
                                        "Constructor" => {
                                            Ok(__Field::__field8)
                                        }
                                        "Enum" => { Ok(__Field::__field9) }
                                        "Interface" => {
                                            Ok(__Field::__field10)
                                        }
                                        "Function" => {
                                            Ok(__Field::__field11)
                                        }
                                        "Variable" => {
                                            Ok(__Field::__field12)
                                        }
                                        "Constant" => {
                                            Ok(__Field::__field13)
                                        }
                                        "String" => { Ok(__Field::__field14) }
                                        "Number" => { Ok(__Field::__field15) }
                                        "Boolean" => {
                                            Ok(__Field::__field16)
                                        }
                                        "Array" => { Ok(__Field::__field17) }
                                        _ =>
                                        Err(_serde::de::Error::unknown_variant(value)),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"File" => { Ok(__Field::__field0) }
                                        b"Module" => { Ok(__Field::__field1) }
                                        b"Namespace" => {
                                            Ok(__Field::__field2)
                                        }
                                        b"Package" => {
                                            Ok(__Field::__field3)
                                        }
                                        b"Class" => { Ok(__Field::__field4) }
                                        b"Method" => { Ok(__Field::__field5) }
                                        b"Property" => {
                                            Ok(__Field::__field6)
                                        }
                                        b"Field" => { Ok(__Field::__field7) }
                                        b"Constructor" => {
                                            Ok(__Field::__field8)
                                        }
                                        b"Enum" => { Ok(__Field::__field9) }
                                        b"Interface" => {
                                            Ok(__Field::__field10)
                                        }
                                        b"Function" => {
                                            Ok(__Field::__field11)
                                        }
                                        b"Variable" => {
                                            Ok(__Field::__field12)
                                        }
                                        b"Constant" => {
                                            Ok(__Field::__field13)
                                        }
                                        b"String" => {
                                            Ok(__Field::__field14)
                                        }
                                        b"Number" => {
                                            Ok(__Field::__field15)
                                        }
                                        b"Boolean" => {
                                            Ok(__Field::__field16)
                                        }
                                        b"Array" => { Ok(__Field::__field17) }
                                        _ => {
                                            let value =
                                                ::std::string::String::from_utf8_lossy(value);
                                            Err(_serde::de::Error::unknown_variant(&value))
                                        }
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer>
                     _serde::de::EnumVisitor for __Visitor<__D> {
                        type
                        Value
                        =
                        SymbolKind;
                        fn visit<__V>(&mut self, mut visitor: __V)
                         -> ::std::result::Result<SymbolKind, __V::Error>
                         where __V: _serde::de::VariantVisitor {
                            match try!(visitor . visit_variant (  )) {
                                __Field::__field0 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(SymbolKind::File)
                                    }
                                }
                                __Field::__field1 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(SymbolKind::Module)
                                    }
                                }
                                __Field::__field2 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(SymbolKind::Namespace)
                                    }
                                }
                                __Field::__field3 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(SymbolKind::Package)
                                    }
                                }
                                __Field::__field4 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(SymbolKind::Class)
                                    }
                                }
                                __Field::__field5 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(SymbolKind::Method)
                                    }
                                }
                                __Field::__field6 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(SymbolKind::Property)
                                    }
                                }
                                __Field::__field7 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(SymbolKind::Field)
                                    }
                                }
                                __Field::__field8 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(SymbolKind::Constructor)
                                    }
                                }
                                __Field::__field9 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(SymbolKind::Enum)
                                    }
                                }
                                __Field::__field10 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(SymbolKind::Interface)
                                    }
                                }
                                __Field::__field11 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(SymbolKind::Function)
                                    }
                                }
                                __Field::__field12 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(SymbolKind::Variable)
                                    }
                                }
                                __Field::__field13 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(SymbolKind::Constant)
                                    }
                                }
                                __Field::__field14 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(SymbolKind::String)
                                    }
                                }
                                __Field::__field15 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(SymbolKind::Number)
                                    }
                                }
                                __Field::__field16 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(SymbolKind::Boolean)
                                    }
                                }
                                __Field::__field17 => {
                                    {
                                        try!(visitor . visit_unit (  ));
                                        Ok(SymbolKind::Array)
                                    }
                                }
                                __Field::__ignore => {
                                    Err(_serde::de::Error::end_of_stream())
                                }
                            }
                        }
                    }
                    const VARIANTS: &'static [&'static str] =
                        &["File", "Module", "Namespace", "Package", "Class",
                          "Method", "Property", "Field", "Constructor",
                          "Enum", "Interface", "Function", "Variable",
                          "Constant", "String", "Number", "Boolean", "Array"];
                    deserializer.deserialize_enum("SymbolKind", VARIANTS,
                                                  __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_SymbolKind: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for SymbolKind {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                match *self {
                    SymbolKind::File => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "SymbolKind",
                                                                        0usize,
                                                                        "File")
                    }
                    SymbolKind::Module => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "SymbolKind",
                                                                        1usize,
                                                                        "Module")
                    }
                    SymbolKind::Namespace => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "SymbolKind",
                                                                        2usize,
                                                                        "Namespace")
                    }
                    SymbolKind::Package => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "SymbolKind",
                                                                        3usize,
                                                                        "Package")
                    }
                    SymbolKind::Class => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "SymbolKind",
                                                                        4usize,
                                                                        "Class")
                    }
                    SymbolKind::Method => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "SymbolKind",
                                                                        5usize,
                                                                        "Method")
                    }
                    SymbolKind::Property => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "SymbolKind",
                                                                        6usize,
                                                                        "Property")
                    }
                    SymbolKind::Field => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "SymbolKind",
                                                                        7usize,
                                                                        "Field")
                    }
                    SymbolKind::Constructor => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "SymbolKind",
                                                                        8usize,
                                                                        "Constructor")
                    }
                    SymbolKind::Enum => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "SymbolKind",
                                                                        9usize,
                                                                        "Enum")
                    }
                    SymbolKind::Interface => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "SymbolKind",
                                                                        10usize,
                                                                        "Interface")
                    }
                    SymbolKind::Function => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "SymbolKind",
                                                                        11usize,
                                                                        "Function")
                    }
                    SymbolKind::Variable => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "SymbolKind",
                                                                        12usize,
                                                                        "Variable")
                    }
                    SymbolKind::Constant => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "SymbolKind",
                                                                        13usize,
                                                                        "Constant")
                    }
                    SymbolKind::String => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "SymbolKind",
                                                                        14usize,
                                                                        "String")
                    }
                    SymbolKind::Number => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "SymbolKind",
                                                                        15usize,
                                                                        "Number")
                    }
                    SymbolKind::Boolean => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "SymbolKind",
                                                                        16usize,
                                                                        "Boolean")
                    }
                    SymbolKind::Array => {
                        _serde::ser::Serializer::serialize_unit_variant(_serializer,
                                                                        "SymbolKind",
                                                                        17usize,
                                                                        "Array")
                    }
                }
            }
        }
    };
/**
 * The workspace symbol request is sent from the client to the server to list project-wide symbols 
 * matching the query string.
 */
pub fn request__WorkspaceSymbols(ls: Rc<LanguageServer>)
 ->
     FnLanguageServerRequest<WorkspaceSymbolParams, Vec<SymbolInformation>,
                             ()> {
    request("workspace/symbol",
            Box::new(move |params| { ls.workspaceSymbols(params) }))
}
/**
 * The parameters of a Workspace Symbol Request.
 */
#[derive(Debug, Clone)]
pub struct WorkspaceSymbolParams {
    /**
     * A non-empty query string
     */
    pub query: string,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_WorkspaceSymbolParams: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for WorkspaceSymbolParams {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<WorkspaceSymbolParams, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "query" => { Ok(__Field::__field0) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"query" => { Ok(__Field::__field0) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        WorkspaceSymbolParams;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<WorkspaceSymbolParams,
                                                   __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: < string > (
                                                )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(WorkspaceSymbolParams{query: __field0,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<WorkspaceSymbolParams,
                                                   __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0: Option<string> = None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("query"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          string > (  )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field ( "query"
                                             )),
                                    };
                                Ok(WorkspaceSymbolParams{query: __field0,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["query"];
                    deserializer.deserialize_struct("WorkspaceSymbolParams",
                                                    FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_WorkspaceSymbolParams: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for WorkspaceSymbolParams {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "WorkspaceSymbolParams" , 0 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "query" , &self.query ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
/**
 * The code action request is sent from the client to the server to compute commands for a given text document
 * and range. The request is triggered when the user moves the cursor into a problem marker in the editor or 
 * presses the lightbulb associated with a marker.
 */
pub fn request__CodeAction(ls: Rc<LanguageServer>)
 -> FnLanguageServerRequest<CodeActionParams, Vec<Command>, ()> {
    request("textDocument/codeAction",
            Box::new(move |params| { ls.codeAction(params) }))
}
/**
 * Params for the CodeActionRequest
 */
#[derive(Debug, Clone)]
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
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_CodeActionParams: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for CodeActionParams {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<CodeActionParams, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __field1, __field2, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        1usize => { Ok(__Field::__field1) }
                                        2usize => { Ok(__Field::__field2) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "textDocument" => {
                                            Ok(__Field::__field0)
                                        }
                                        "range" => { Ok(__Field::__field1) }
                                        "context" => { Ok(__Field::__field2) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"textDocument" => {
                                            Ok(__Field::__field0)
                                        }
                                        b"range" => { Ok(__Field::__field1) }
                                        b"context" => {
                                            Ok(__Field::__field2)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        CodeActionParams;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<CodeActionParams,
                                                   __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: <
                                               TextDocumentIdentifier > (  ))
                                        {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                let __field1 =
                                    match try!(visitor . visit :: < Range > (
                                               )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(1usize));
                                        }
                                    };
                                let __field2 =
                                    match try!(visitor . visit :: <
                                               CodeActionContext > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(2usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(CodeActionParams{textDocument: __field0,
                                                    range: __field1,
                                                    context: __field2,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<CodeActionParams,
                                                   __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0:
                                        Option<TextDocumentIdentifier> = None;
                                let mut __field1: Option<Range> = None;
                                let mut __field2: Option<CodeActionContext> =
                                    None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("textDocument"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          TextDocumentIdentifier
                                                          > (  )));
                                        }
                                        __Field::__field1 => {
                                            if __field1.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("range"));
                                            }
                                            __field1 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Range > (  )));
                                        }
                                        __Field::__field2 => {
                                            if __field2.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("context"));
                                            }
                                            __field2 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          CodeActionContext >
                                                          (  )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field (
                                             "textDocument" )),
                                    };
                                let __field1 =
                                    match __field1 {
                                        Some(__field1) => __field1,
                                        None =>
                                        try!(visitor . missing_field ( "range"
                                             )),
                                    };
                                let __field2 =
                                    match __field2 {
                                        Some(__field2) => __field2,
                                        None =>
                                        try!(visitor . missing_field (
                                             "context" )),
                                    };
                                Ok(CodeActionParams{textDocument: __field0,
                                                    range: __field1,
                                                    context: __field2,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["textDocument", "range", "context"];
                    deserializer.deserialize_struct("CodeActionParams",
                                                    FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_CodeActionParams: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for CodeActionParams {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "CodeActionParams" , 0 + 1 + 1 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "textDocument" , &self.textDocument
                             ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "range" , &self.range ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "context" , &self.context ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
/**
 * Contains additional diagnostic information about the context in which
 * a code action is run.
 */
#[derive(Debug, Clone)]
pub struct CodeActionContext {
    /**
     * An array of diagnostics.
     */
    pub diagnostics: Vec<Diagnostic>,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_CodeActionContext: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for CodeActionContext {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<CodeActionContext, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "diagnostics" => {
                                            Ok(__Field::__field0)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"diagnostics" => {
                                            Ok(__Field::__field0)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        CodeActionContext;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<CodeActionContext,
                                                   __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: <
                                               Vec<Diagnostic> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(CodeActionContext{diagnostics: __field0,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<CodeActionContext,
                                                   __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0: Option<Vec<Diagnostic>> =
                                    None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("diagnostics"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Vec<Diagnostic> > (
                                                          )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field (
                                             "diagnostics" )),
                                    };
                                Ok(CodeActionContext{diagnostics: __field0,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["diagnostics"];
                    deserializer.deserialize_struct("CodeActionContext",
                                                    FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_CodeActionContext: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for CodeActionContext {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "CodeActionContext" , 0 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "diagnostics" , &self.diagnostics
                             ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
/**
 * The code lens request is sent from the client to the server to compute code lenses for a given text document.
 */
pub fn request__CodeLens(ls: Rc<LanguageServer>)
 -> FnLanguageServerRequest<CodeLensParams, Vec<CodeLens>, ()> {
    request("textDocument/codeLens",
            Box::new(move |params| { ls.codeLens(params) }))
}
#[derive(Debug, Clone)]
pub struct CodeLensParams {
    /**
     * The document to request code lens for.
     */
    pub textDocument: TextDocumentIdentifier,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_CodeLensParams: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for CodeLensParams {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<CodeLensParams, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "textDocument" => {
                                            Ok(__Field::__field0)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"textDocument" => {
                                            Ok(__Field::__field0)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        CodeLensParams;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         -> ::std::result::Result<CodeLensParams, __V::Error>
                         where __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: <
                                               TextDocumentIdentifier > (  ))
                                        {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(CodeLensParams{textDocument: __field0,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         -> ::std::result::Result<CodeLensParams, __V::Error>
                         where __V: _serde::de::MapVisitor {
                            {
                                let mut __field0:
                                        Option<TextDocumentIdentifier> = None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("textDocument"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          TextDocumentIdentifier
                                                          > (  )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field (
                                             "textDocument" )),
                                    };
                                Ok(CodeLensParams{textDocument: __field0,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["textDocument"];
                    deserializer.deserialize_struct("CodeLensParams", FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_CodeLensParams: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for CodeLensParams {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "CodeLensParams" , 0 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "textDocument" , &self.textDocument
                             ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
/**
 * A code lens represents a command that should be shown along with
 * source text, like the number of references, a way to run tests, etc.
 *
 * A code lens is _unresolved_ when no command is associated to it. For performance
 * reasons the creation of a code lens and resolving should be done in two stages.
 */
#[derive(Debug, Clone)]
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
    pub data: Option<any>,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_CodeLens: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for CodeLens {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<CodeLens, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __field1, __field2, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        1usize => { Ok(__Field::__field1) }
                                        2usize => { Ok(__Field::__field2) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "range" => { Ok(__Field::__field0) }
                                        "command" => { Ok(__Field::__field1) }
                                        "data" => { Ok(__Field::__field2) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"range" => { Ok(__Field::__field0) }
                                        b"command" => {
                                            Ok(__Field::__field1)
                                        }
                                        b"data" => { Ok(__Field::__field2) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        CodeLens;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         -> ::std::result::Result<CodeLens, __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: < Range > (
                                               )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                let __field1 =
                                    match try!(visitor . visit :: <
                                               Option<Command> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(1usize));
                                        }
                                    };
                                let __field2 =
                                    match try!(visitor . visit :: <
                                               Option<any> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(2usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(CodeLens{range: __field0,
                                            command: __field1,
                                            data: __field2,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         -> ::std::result::Result<CodeLens, __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0: Option<Range> = None;
                                let mut __field1: Option<Option<Command>> =
                                    None;
                                let mut __field2: Option<Option<any>> = None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("range"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Range > (  )));
                                        }
                                        __Field::__field1 => {
                                            if __field1.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("command"));
                                            }
                                            __field1 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<Command> > (
                                                          )));
                                        }
                                        __Field::__field2 => {
                                            if __field2.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("data"));
                                            }
                                            __field2 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<any> > (
                                                          )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field ( "range"
                                             )),
                                    };
                                let __field1 =
                                    match __field1 {
                                        Some(__field1) => __field1,
                                        None =>
                                        try!(visitor . missing_field (
                                             "command" )),
                                    };
                                let __field2 =
                                    match __field2 {
                                        Some(__field2) => __field2,
                                        None =>
                                        try!(visitor . missing_field ( "data"
                                             )),
                                    };
                                Ok(CodeLens{range: __field0,
                                            command: __field1,
                                            data: __field2,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["range", "command", "data"];
                    deserializer.deserialize_struct("CodeLens", FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_CodeLens: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for CodeLens {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "CodeLens" , 0 + 1 + 1 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "range" , &self.range ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "command" , &self.command ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "data" , &self.data ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
/**
 * The code lens resolve request is sent from the client to the server to resolve the command for a 
 * given code lens item.
 */
pub fn request__CodeLensResolve(ls: Rc<LanguageServer>)
 -> FnLanguageServerRequest<CodeLens, CodeLens, ()> {
    request("codeLens/resolve",
            Box::new(move |params| { ls.codeLensResolve(params) }))
}
/**
 * The document formatting request is sent from the server to the client to format a whole document.
 */
pub fn request__Formatting(ls: Rc<LanguageServer>)
 -> FnLanguageServerRequest<DocumentFormattingParams, Vec<TextEdit>, ()> {
    request("textDocument/formatting",
            Box::new(move |params| { ls.formatting(params) }))
}
#[derive(Debug, Clone)]
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
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_DocumentFormattingParams: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for DocumentFormattingParams {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<DocumentFormattingParams, __D::Error>
             where __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __field1, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        1usize => { Ok(__Field::__field1) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "textDocument" => {
                                            Ok(__Field::__field0)
                                        }
                                        "options" => { Ok(__Field::__field1) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"textDocument" => {
                                            Ok(__Field::__field0)
                                        }
                                        b"options" => {
                                            Ok(__Field::__field1)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        DocumentFormattingParams;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<DocumentFormattingParams,
                                                   __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: <
                                               TextDocumentIdentifier > (  ))
                                        {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                let __field1 =
                                    match try!(visitor . visit :: <
                                               FormattingOptions > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(1usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(DocumentFormattingParams{textDocument:
                                                                __field0,
                                                            options:
                                                                __field1,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<DocumentFormattingParams,
                                                   __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0:
                                        Option<TextDocumentIdentifier> = None;
                                let mut __field1: Option<FormattingOptions> =
                                    None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("textDocument"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          TextDocumentIdentifier
                                                          > (  )));
                                        }
                                        __Field::__field1 => {
                                            if __field1.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("options"));
                                            }
                                            __field1 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          FormattingOptions >
                                                          (  )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field (
                                             "textDocument" )),
                                    };
                                let __field1 =
                                    match __field1 {
                                        Some(__field1) => __field1,
                                        None =>
                                        try!(visitor . missing_field (
                                             "options" )),
                                    };
                                Ok(DocumentFormattingParams{textDocument:
                                                                __field0,
                                                            options:
                                                                __field1,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["textDocument", "options"];
                    deserializer.deserialize_struct("DocumentFormattingParams",
                                                    FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_DocumentFormattingParams: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for DocumentFormattingParams {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "DocumentFormattingParams" , 0 + 1 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "textDocument" , &self.textDocument
                             ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "options" , &self.options ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
/**
 * Value-object describing what options formatting should use.
 */
#[derive(Debug, Clone)]
pub struct FormattingOptions {
    /**
     * Size of a tab in spaces.
     */
    pub tabSize: number,
    /**
     * Prefer spaces over tabs.
     */
    pub insertSpaces: boolean,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_FormattingOptions: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for FormattingOptions {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<FormattingOptions, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __field1, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        1usize => { Ok(__Field::__field1) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "tabSize" => { Ok(__Field::__field0) }
                                        "insertSpaces" => {
                                            Ok(__Field::__field1)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"tabSize" => {
                                            Ok(__Field::__field0)
                                        }
                                        b"insertSpaces" => {
                                            Ok(__Field::__field1)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        FormattingOptions;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<FormattingOptions,
                                                   __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: < number > (
                                                )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                let __field1 =
                                    match try!(visitor . visit :: < boolean >
                                               (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(1usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(FormattingOptions{tabSize: __field0,
                                                     insertSpaces: __field1,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<FormattingOptions,
                                                   __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0: Option<number> = None;
                                let mut __field1: Option<boolean> = None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("tabSize"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          number > (  )));
                                        }
                                        __Field::__field1 => {
                                            if __field1.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("insertSpaces"));
                                            }
                                            __field1 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          boolean > (  )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field (
                                             "tabSize" )),
                                    };
                                let __field1 =
                                    match __field1 {
                                        Some(__field1) => __field1,
                                        None =>
                                        try!(visitor . missing_field (
                                             "insertSpaces" )),
                                    };
                                Ok(FormattingOptions{tabSize: __field0,
                                                     insertSpaces: __field1,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["tabSize", "insertSpaces"];
                    deserializer.deserialize_struct("FormattingOptions",
                                                    FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_FormattingOptions: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for FormattingOptions {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "FormattingOptions" , 0 + 1 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "tabSize" , &self.tabSize ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "insertSpaces" , &self.insertSpaces
                             ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
/**
 * The document range formatting request is sent from the client to the server to format a given range in a document.
 */
pub fn request__RangeFormatting(ls: Rc<LanguageServer>)
 ->
     FnLanguageServerRequest<DocumentRangeFormattingParams, Vec<TextEdit>,
                             ()> {
    request("textDocument/rangeFormatting",
            Box::new(move |params| { ls.rangeFormatting(params) }))
}
#[derive(Debug, Clone)]
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
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_DocumentRangeFormattingParams: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for DocumentRangeFormattingParams {
            fn deserialize<__D>(deserializer: &mut __D)
             ->
                 ::std::result::Result<DocumentRangeFormattingParams,
                                       __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __field1, __field2, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        1usize => { Ok(__Field::__field1) }
                                        2usize => { Ok(__Field::__field2) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "textDocument" => {
                                            Ok(__Field::__field0)
                                        }
                                        "range" => { Ok(__Field::__field1) }
                                        "options" => { Ok(__Field::__field2) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"textDocument" => {
                                            Ok(__Field::__field0)
                                        }
                                        b"range" => { Ok(__Field::__field1) }
                                        b"options" => {
                                            Ok(__Field::__field2)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        DocumentRangeFormattingParams;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<DocumentRangeFormattingParams,
                                                   __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: <
                                               TextDocumentIdentifier > (  ))
                                        {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                let __field1 =
                                    match try!(visitor . visit :: < Range > (
                                               )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(1usize));
                                        }
                                    };
                                let __field2 =
                                    match try!(visitor . visit :: <
                                               FormattingOptions > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(2usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(DocumentRangeFormattingParams{textDocument:
                                                                     __field0,
                                                                 range:
                                                                     __field1,
                                                                 options:
                                                                     __field2,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<DocumentRangeFormattingParams,
                                                   __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0:
                                        Option<TextDocumentIdentifier> = None;
                                let mut __field1: Option<Range> = None;
                                let mut __field2: Option<FormattingOptions> =
                                    None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("textDocument"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          TextDocumentIdentifier
                                                          > (  )));
                                        }
                                        __Field::__field1 => {
                                            if __field1.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("range"));
                                            }
                                            __field1 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Range > (  )));
                                        }
                                        __Field::__field2 => {
                                            if __field2.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("options"));
                                            }
                                            __field2 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          FormattingOptions >
                                                          (  )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field (
                                             "textDocument" )),
                                    };
                                let __field1 =
                                    match __field1 {
                                        Some(__field1) => __field1,
                                        None =>
                                        try!(visitor . missing_field ( "range"
                                             )),
                                    };
                                let __field2 =
                                    match __field2 {
                                        Some(__field2) => __field2,
                                        None =>
                                        try!(visitor . missing_field (
                                             "options" )),
                                    };
                                Ok(DocumentRangeFormattingParams{textDocument:
                                                                     __field0,
                                                                 range:
                                                                     __field1,
                                                                 options:
                                                                     __field2,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["textDocument", "range", "options"];
                    deserializer.deserialize_struct("DocumentRangeFormattingParams",
                                                    FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_DocumentRangeFormattingParams: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for DocumentRangeFormattingParams {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "DocumentRangeFormattingParams" , 0 + 1 + 1 + 1
                             ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "textDocument" , &self.textDocument
                             ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "range" , &self.range ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "options" , &self.options ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
/**
 * The document on type formatting request is sent from the client to the server to format parts of 
 * the document during typing.
 */
pub fn request__OnTypeFormatting(ls: Rc<LanguageServer>)
 ->
     FnLanguageServerRequest<DocumentOnTypeFormattingParams, Vec<TextEdit>,
                             ()> {
    request("textDocument/onTypeFormatting",
            Box::new(move |params| { ls.onTypeFormatting(params) }))
}
#[derive(Debug, Clone)]
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
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_DocumentOnTypeFormattingParams: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for DocumentOnTypeFormattingParams {
            fn deserialize<__D>(deserializer: &mut __D)
             ->
                 ::std::result::Result<DocumentOnTypeFormattingParams,
                                       __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __ignore,
                    }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        1usize => { Ok(__Field::__field1) }
                                        2usize => { Ok(__Field::__field2) }
                                        3usize => { Ok(__Field::__field3) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "textDocument" => {
                                            Ok(__Field::__field0)
                                        }
                                        "position" => {
                                            Ok(__Field::__field1)
                                        }
                                        "ch" => { Ok(__Field::__field2) }
                                        "options" => { Ok(__Field::__field3) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"textDocument" => {
                                            Ok(__Field::__field0)
                                        }
                                        b"position" => {
                                            Ok(__Field::__field1)
                                        }
                                        b"ch" => { Ok(__Field::__field2) }
                                        b"options" => {
                                            Ok(__Field::__field3)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        DocumentOnTypeFormattingParams;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<DocumentOnTypeFormattingParams,
                                                   __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: <
                                               TextDocumentIdentifier > (  ))
                                        {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                let __field1 =
                                    match try!(visitor . visit :: < Position >
                                               (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(1usize));
                                        }
                                    };
                                let __field2 =
                                    match try!(visitor . visit :: < string > (
                                                )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(2usize));
                                        }
                                    };
                                let __field3 =
                                    match try!(visitor . visit :: <
                                               FormattingOptions > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(3usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(DocumentOnTypeFormattingParams{textDocument:
                                                                      __field0,
                                                                  position:
                                                                      __field1,
                                                                  ch:
                                                                      __field2,
                                                                  options:
                                                                      __field3,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         ->
                             ::std::result::Result<DocumentOnTypeFormattingParams,
                                                   __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0:
                                        Option<TextDocumentIdentifier> = None;
                                let mut __field1: Option<Position> = None;
                                let mut __field2: Option<string> = None;
                                let mut __field3: Option<FormattingOptions> =
                                    None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("textDocument"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          TextDocumentIdentifier
                                                          > (  )));
                                        }
                                        __Field::__field1 => {
                                            if __field1.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("position"));
                                            }
                                            __field1 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Position > (  )));
                                        }
                                        __Field::__field2 => {
                                            if __field2.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("ch"));
                                            }
                                            __field2 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          string > (  )));
                                        }
                                        __Field::__field3 => {
                                            if __field3.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("options"));
                                            }
                                            __field3 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          FormattingOptions >
                                                          (  )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field (
                                             "textDocument" )),
                                    };
                                let __field1 =
                                    match __field1 {
                                        Some(__field1) => __field1,
                                        None =>
                                        try!(visitor . missing_field (
                                             "position" )),
                                    };
                                let __field2 =
                                    match __field2 {
                                        Some(__field2) => __field2,
                                        None =>
                                        try!(visitor . missing_field ( "ch"
                                             )),
                                    };
                                let __field3 =
                                    match __field3 {
                                        Some(__field3) => __field3,
                                        None =>
                                        try!(visitor . missing_field (
                                             "options" )),
                                    };
                                Ok(DocumentOnTypeFormattingParams{textDocument:
                                                                      __field0,
                                                                  position:
                                                                      __field1,
                                                                  ch:
                                                                      __field2,
                                                                  options:
                                                                      __field3,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["textDocument", "position", "ch", "options"];
                    deserializer.deserialize_struct("DocumentOnTypeFormattingParams",
                                                    FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_DocumentOnTypeFormattingParams: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for DocumentOnTypeFormattingParams {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "DocumentOnTypeFormattingParams" ,
                             0 + 1 + 1 + 1 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "textDocument" , &self.textDocument
                             ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "position" , &self.position ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "ch" , &self.ch ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "options" , &self.options ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
/**
 * The rename request is sent from the client to the server to perform a workspace-wide rename of a symbol.
 */
pub fn request__Rename(ls: Rc<LanguageServer>)
 -> FnLanguageServerRequest<RenameParams, WorkspaceEdit, ()> {
    request("textDocument/rename",
            Box::new(move |params| { ls.rename(params) }))
}
#[derive(Debug, Clone)]
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
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_RenameParams: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for RenameParams {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<RenameParams, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __field1, __field2, __ignore, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        1usize => { Ok(__Field::__field1) }
                                        2usize => { Ok(__Field::__field2) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "textDocument" => {
                                            Ok(__Field::__field0)
                                        }
                                        "position" => {
                                            Ok(__Field::__field1)
                                        }
                                        "newName" => { Ok(__Field::__field2) }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"textDocument" => {
                                            Ok(__Field::__field0)
                                        }
                                        b"position" => {
                                            Ok(__Field::__field1)
                                        }
                                        b"newName" => {
                                            Ok(__Field::__field2)
                                        }
                                        _ => Ok(__Field::__ignore),
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
                        type
                        Value
                        =
                        RenameParams;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         -> ::std::result::Result<RenameParams, __V::Error>
                         where __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: <
                                               TextDocumentIdentifier > (  ))
                                        {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                let __field1 =
                                    match try!(visitor . visit :: < Position >
                                               (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(1usize));
                                        }
                                    };
                                let __field2 =
                                    match try!(visitor . visit :: < string > (
                                                )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(2usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(RenameParams{textDocument: __field0,
                                                position: __field1,
                                                newName: __field2,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         -> ::std::result::Result<RenameParams, __V::Error>
                         where __V: _serde::de::MapVisitor {
                            {
                                let mut __field0:
                                        Option<TextDocumentIdentifier> = None;
                                let mut __field1: Option<Position> = None;
                                let mut __field2: Option<string> = None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("textDocument"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          TextDocumentIdentifier
                                                          > (  )));
                                        }
                                        __Field::__field1 => {
                                            if __field1.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("position"));
                                            }
                                            __field1 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Position > (  )));
                                        }
                                        __Field::__field2 => {
                                            if __field2.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("newName"));
                                            }
                                            __field2 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          string > (  )));
                                        }
                                        _ => {
                                            try!(visitor . visit_value :: <
                                                 _serde :: de :: impls ::
                                                 IgnoredAny > (  ));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field (
                                             "textDocument" )),
                                    };
                                let __field1 =
                                    match __field1 {
                                        Some(__field1) => __field1,
                                        None =>
                                        try!(visitor . missing_field (
                                             "position" )),
                                    };
                                let __field2 =
                                    match __field2 {
                                        Some(__field2) => __field2,
                                        None =>
                                        try!(visitor . missing_field (
                                             "newName" )),
                                    };
                                Ok(RenameParams{textDocument: __field0,
                                                position: __field1,
                                                newName: __field2,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["textDocument", "position", "newName"];
                    deserializer.deserialize_struct("RenameParams", FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_RenameParams: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for RenameParams {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                {
                    let mut state =
                        try!(_serializer . serialize_struct (
                             "RenameParams" , 0 + 1 + 1 + 1 ));
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "textDocument" , &self.textDocument
                             ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "position" , &self.position ));
                    }
                    if !false {
                        try!(_serializer . serialize_struct_elt (
                             & mut state , "newName" , &self.newName ));
                    }
                    _serializer.serialize_struct_end(state)
                }
            }
        }
    };
