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


// Based on protocol: https://github.com/Microsoft/language-server-protocol/blob/master/protocol.md
// Last revision 03/08/2016

pub trait LanguageServerNotification<PARAMS> {
    fn method_name()
    -> &'static str;
    fn invoke(params: PARAMS);
}

pub trait LanguageServerRequest<PARAMS, RET, ERR> {
    fn method_name()
    -> &'static str;
    fn invoke(params: PARAMS)
    -> Result<RET, ERR>; /* FIXME: use error structure */
}

/* ----------------- Basic JSON Structures ----------------- */

pub type boolean = bool;
pub type string = String;
pub type number = u64;
pub type number_or_string = string;
 /* FIXME: */
pub type any = Value;

/// Position in a text document expressed as zero-based line and character offset.
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



                          /* ----------------- Protocol Structures ----------------- */









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
///A range in a text document expressed as (zero-based) start and end positions.
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
/// Represents a reference to a command. Provides a title which will be used to represent a command in the UI and, 
/// optionally, an array of arguments which will be passed to the command handler function when invoked.
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
pub trait InitializeRequest: LanguageServerRequest<InitializeParams,
                                                   InitializeResult,
                                                   InitializeError> {
    fn method_name() -> &'static str { "initialize" }
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
                                        "processId" => {
                                            Ok(__Field::__field0)
                                        }
                                        "rootPath" => {
                                            Ok(__Field::__field1)
                                        }
                                        "capabilities" => {
                                            Ok(__Field::__field2)
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
                                        b"capabilities" => {
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
                                               ClientCapabilities > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(2usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(InitializeParams{processId: __field0,
                                                    rootPath: __field1,
                                                    capabilities: __field2,})
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
                                let mut __field2: Option<ClientCapabilities> =
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
                                                               _serde::de::Error>::duplicate_field("capabilities"));
                                            }
                                            __field2 =
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
                                             "capabilities" )),
                                    };
                                Ok(InitializeParams{processId: __field0,
                                                    rootPath: __field1,
                                                    capabilities: __field2,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["processId", "rootPath", "capabilities"];
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
                             "InitializeParams" , 0 + 1 + 1 + 1 ));
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
pub struct InitializeResult {
    /**
     * The capabilities the language server provides.
     */
    pub capabilities: ServerCapabilities,
}
pub struct InitializeError {
    /**
     * Indicates whether the client should retry to send the
     * initilize request after showing the message provided
     * in the ResponseError.
     */
    pub retry: boolean,
}
/**
 * Defines how the host (editor) should sync document changes to the language server.
 */
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
/**
 * Completion options.
 */
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
/**
 * Signature help options.
 */
pub struct SignatureHelpOptions {
    /**
     * The characters that trigger signature help automatically.
     */
    pub triggerCharacters: Option<Vec<string>>,
}
/**
 * Code Lens options.
 */
pub struct CodeLensOptions {
    /**
     * Code lens has a resolve provider as well.
     */
    pub resolveProvider: Option<boolean>,
}
/**
 * Format document on type options
 */
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
/**
 * The shutdown request is sent from the client to the server. It asks the server to shut down,
 * but to not exit (otherwise the response might not be delivered correctly to the client).
 * There is a separate exit notification that asks the server to exit.
 */
pub trait ShutdownRequest: LanguageServerRequest<(), (), ()> {
    fn method_name() -> &'static str { "shutdown" }
}
/**
 * A notification to ask the server to exit its process.
 */
pub trait ExitNotification: LanguageServerNotification<()> {
    fn method_name() -> &'static str { "exit" }
}
/**
 * The show message notification is sent from a server to a client to ask the client to display a particular message
 * in the user interface.
 */
pub trait ShowMessageNotification: LanguageServerNotification<ShowMessageParams> {
    fn method_name() -> &'static str { "window/showMessage" }
}
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
/**
 * The show message request is sent from a server to a client to ask the client to display a particular message
 * in the user interface. In addition to the show message notification the request allows to pass actions and to
 * wait for an answer from the client.
 */
pub trait ShowMessageRequestNotification: LanguageServerNotification<ShowMessageRequestParams> {
    fn method_name() -> &'static str { "window/showMessageRequest" }
}
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
pub struct MessageActionItem {
    /**
     * A short title like 'Retry', 'Open Log' etc.
     */
    title: string,
}
/**
 * The log message notification is sent from the server to the client to ask the client to log a particular message.
 */
pub trait LogMessageNotification: LanguageServerNotification<LogMessageParams> {
    fn method_name() -> &'static str { "window/logMessage" }
}
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
/**
 * The telemetry notification is sent from the server to the client to ask the client to log a telemetry event.
 */
pub trait TelemetryEventNotification: LanguageServerNotification<any> {
    fn method_name() -> &'static str { "telemetry/event" }
}
/**
 * A notification sent from the client to the server to signal the change of configuration settings.
 */
pub trait WorkspaceChangeConfigurationNotification: LanguageServerNotification<DidChangeConfigurationParams> {
    fn method_name() -> &'static str { "workspace/didChangeConfiguration" }
}
pub struct DidChangeConfigurationParams {
    /**
     * The actual changed settings
     */
    pub settings: any,
}
/**
 * The document open notification is sent from the client to the server to signal newly opened text documents.
 * The document's truth is now managed by the client and the server must not try to read the document's truth
 * using the document's uri.
 */
pub trait DidOpenTextDocumentNotification: LanguageServerNotification<DidOpenTextDocumentParams> {
    fn method_name() -> &'static str { "textDocument/didOpen" }
}
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
pub trait DidChangeTextDocumentNotification: LanguageServerNotification<DidChangeTextDocumentParams> {
    fn method_name() -> &'static str { "textDocument/didChange" }
}
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
/**
 * The document close notification is sent from the client to the server when the document got closed in the client.
 * The document's truth now exists where the document's uri points to (e.g. if the document's uri is a file uri
 * the truth now exists on disk).
 */
pub trait DidCloseTextDocumentNotification: LanguageServerNotification<DidCloseTextDocumentParams> {
    fn method_name() -> &'static str { "textDocument/didClose" }
}
pub struct DidCloseTextDocumentParams {
    /**
     * The document that was closed.
     */
    pub textDocument: TextDocumentIdentifier,
}
/**
 * The document save notification is sent from the client to the server when the document was saved in the client.
 */
pub trait DidSaveTextDocumentNotification: LanguageServerNotification<DidSaveTextDocumentParams> {
    fn method_name() -> &'static str { "textDocument/didSave" }
}
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
pub trait DidChangeWatchedFilesNotification: LanguageServerNotification<DidChangeWatchedFilesParams> {
    fn method_name() -> &'static str { "workspace/didChangeWatchedFiles" }
}
pub struct DidChangeWatchedFilesParams {
    /**
     * The actual file events.
     */
    pub changes: Vec<FileEvent>,
}
/**
 * The file event type.
 */
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
/**
 * An event describing a file change.
 */
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
/**
 * Diagnostics notification are sent from the server to the client to signal results of validation runs.
 */
pub trait PublishDiagnosticsNotification: LanguageServerNotification<PublishDiagnosticsParams> {
    fn method_name() -> &'static str { "textDocument/publishDiagnostics" }
}
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
 * The Completion request is sent from the client to the server to compute completion items at a given cursor position.
 * Completion items are presented in the IntelliSense user interface. If computing full completion items is expensive,
 * servers can additionally provide a handler for the completion item resolve request. 
 * This request is sent when a completion item is selected in the user interface. 
 */
pub trait CompletionRequest: LanguageServerRequest<TextDocumentPositionParams,
                                                   CompletionList, ()> {
    fn method_name() -> &'static str { "textDocument/completion" }
}
/**
 * Represents a collection of [completion items](#CompletionItem) to be presented
 * in the editor.
 */
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
/**
 * The request is sent from the client to the server to resolve additional information for a given completion item. 
 */
pub trait ResolveCompletionItemRequest: LanguageServerRequest<CompletionItem,
                                                              CompletionItem,
                                                              ()> {
    fn method_name() -> &'static str { "completionItem/resolve" }
}
/**
 * The hover request is sent from the client to the server to request hover information at a given text 
 * document position.
 */
pub trait HoverRequest: LanguageServerRequest<TextDocumentPositionParams,
                                              Hover, ()> {
    fn method_name() -> &'static str { "textDocument/hover" }
}
/**
 * The result of a hover request.
 */
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
pub type MarkedString = string;
/**
 * The signature help request is sent from the client to the server to request signature information at 
 * a given cursor position.
 */
pub trait SignatureHelpRequest: LanguageServerRequest<TextDocumentPositionParams,
                                                      SignatureHelp, ()> {
    fn method_name() -> &'static str { "textDocument/signatureHelp" }
}
/**
 * Signature help represents the signature of something
 * callable. There can be multiple signature but only one
 * active and only one active parameter.
 */
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
pub trait GotoDefinitionRequest: LanguageServerRequest<TextDocumentPositionParams,
                                                       Vec<Location>, ()> {
    fn method_name() -> &'static str { "textDocument/definition" }
}
/**
 * The references request is sent from the client to the server to resolve project-wide references for the 
 * symbol denoted by the given text document position.
 */
pub trait ReferencesRequest: LanguageServerRequest<ReferenceParams,
                                                   Vec<Location>, ()> {
    fn method_name() -> &'static str { "textDocument/references" }
}
pub struct ReferenceParams {
    pub context: ReferenceContext,
}
pub struct ReferenceContext {
    /**
     * Include the declaration of the current symbol.
     */
    pub includeDeclaration: boolean,
}
/**
 * The document highlight request is sent from the client to the server to resolve a document highlights 
 * for a given text document position. 
 */
pub trait DocumentHighlightRequest: LanguageServerRequest<TextDocumentPositionParams,
                                                          DocumentHighlight,
                                                          ()> {
    fn method_name() -> &'static str { "textDocument/documentHighlight" }
}
/**
 * A document highlight is a range inside a text document which deserves
 * special attention. Usually a document highlight is visualized by changing
 * the background color of its range.
 */
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
/**
 * The document symbol request is sent from the client to the server to list all symbols found in a given 
 * text document.
 */
pub trait DocumentSymbolsRequest: LanguageServerRequest<DocumentSymbolParams,
                                                        Vec<SymbolInformation>,
                                                        ()> {
    fn method_name() -> &'static str { "textDocument/documentSymbol" }
}
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
/**
 * A symbol kind.
 */
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
pub trait WorkspaceSymbolsRequest: LanguageServerRequest<WorkspaceSymbolParams,
                                                         Vec<SymbolInformation>,
                                                         ()> {
    fn method_name() -> &'static str { "workspace/symbol" }
}
/**
 * The parameters of a Workspace Symbol Request.
 */
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
pub trait CodeActionRequest: LanguageServerRequest<CodeActionParams,
                                                   Vec<Command>, ()> {
    fn method_name() -> &'static str { "textDocument/codeAction" }
}
/**
 * Params for the CodeActionRequest
 */
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
pub struct CodeActionContext {
    /**
     * An array of diagnostics.
     */
    pub diagnostics: Vec<Diagnostic>,
}
/**
 * The code lens request is sent from the client to the server to compute code lenses for a given text document.
 */
pub trait CodeLensRequest: LanguageServerRequest<CodeLensParams,
                                                 Vec<CodeLens>, ()> {
    fn method_name() -> &'static str { "textDocument/codeLens" }
}
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
/**
 * The code lens resolve request is sent from the client to the server to resolve the command for a 
 * given code lens item.
 */
pub trait CodeLensResolveRequest: LanguageServerRequest<CodeLens, CodeLens,
                                                        ()> {
    fn method_name() -> &'static str { "codeLens/resolve" }
}
/**
 * The document formatting request is sent from the server to the client to format a whole document.
 */
pub trait FormattingRequest: LanguageServerRequest<DocumentFormattingParams,
                                                   Vec<TextEdit>, ()> {
    fn method_name() -> &'static str { "textDocument/formatting" }
}
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
/**
 * The document range formatting request is sent from the client to the server to format a given range in a document.
 */
pub trait RangeFormattingRequest: LanguageServerRequest<DocumentRangeFormattingParams,
                                                        Vec<TextEdit>, ()> {
    fn method_name() -> &'static str { "textDocument/rangeFormatting" }
}
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
pub trait OnTypeFormattingRequest: LanguageServerRequest<DocumentOnTypeFormattingParams,
                                                         Vec<TextEdit>, ()> {
    fn method_name() -> &'static str { "textDocument/onTypeFormatting" }
}
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
pub trait RenameRequest: LanguageServerRequest<RenameParams, WorkspaceEdit,
                                               ()> {
    fn method_name() -> &'static str { "textDocument/rename" }
}
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
