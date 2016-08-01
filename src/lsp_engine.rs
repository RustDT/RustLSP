struct InitializeParams {
    /**
     * The process Id of the parent process that started
     * the server.
     */
    pub processId: u64,

    /**
     * The rootPath of the workspace. Is null
     * if no folder is open.
     */
    pub rootPath: String,

//    /**
//     * The capabilities provided by the client (editor)
//     */
//    capabilities: ClientCapabilities,
}