[![Build Status](https://travis-ci.org/RustDT/RustLSP.svg?branch=master)](https://travis-ci.org/RustDT/RustLSP)

# RustLSP
A Language Server Protocol implementation in Rust (LSP protocol only, not tied to any particular language engine)

Work in progress. Not quite usable yet.

## Note:

Serde code_gen is not run by default. To run it, invoke `cargo build --features "gen_sources"`.

#### Projects using RustLSP:
* [MockLS](https://github.com/RustDT/MockLS)
* [RLS](https://github.com/jonathandturner/rls/pull/96) (in PR only)
