// Note: Rust newbie code ahead (-_-)'


//use ::util::core::*;

extern crate serde_json;

use self::serde_json::Map;
use self::serde_json::Value;

pub type ClientCapabilities = Map<String, Value>;

pub struct InitializeParams {
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

	/**
	 * The capabilities provided by the client (editor)
	 */
	pub capabilities: ClientCapabilities,
	
}
