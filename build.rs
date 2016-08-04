extern crate serde_codegen;

use std::env;
use std::path::Path;

pub fn main() {
    serde_codegen::expand(&Path::new("src/lsp.IN.rs"), &Path::new("src/lsp.rs")).unwrap();
}