# DummyLSP
A Language Server Protocol server implementation for a fake/dummy language. 

The purpose of this tool is to help test LSP clients. **WORK IN PROGRESS**

DummyLSP is compiled into an executable that communicates with LSP client via stdin/stdout or via TCP sockets. 
Command line:
 * `dummy_lsp`: communicate via stdin/stdout
 * `dummy_lsp <port number>`: listen on localhost, on given port number 
