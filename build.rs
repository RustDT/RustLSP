extern crate serde_codegen;

#[cfg(feature = "gen_sources")]
pub fn main() {
	use std::env;
	use std::path::Path;

    serde_codegen::expand(&Path::new("src/lsp.IN.rs"), &Path::new("src/lsp.rs")).unwrap();
}

#[cfg(not(feature = "gen_sources"))]
pub fn main() {
}
