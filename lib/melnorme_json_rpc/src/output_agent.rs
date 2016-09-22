// Copyright 2016 Bruno Medeiros
//
// Licensed under the Apache License, Version 2.0 
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>. 
// This file may not be copied, modified, or distributed
// except according to those terms.

use std;

use util::core::*;

use std::thread;
use std::sync::mpsc;
use std::io;

/* ----------------- Output_Agent ----------------- */

pub type OutputAgentTask = Box<Fn(&mut io::Write) + Send>;

pub enum OutputAgentMessage {
	Shutdown,
	Task(OutputAgentTask),
}

const ERR_SEND_TASK_FAILED : &'static str 
	= "Failed to send task, Agent receive channel is closed.";

/**
 * Actor-like, dedicated worker thread that handles writing to an output stream.
 * Accepts tasks as messages, which are executed by the agent.
 */
pub struct OutputAgent {
	is_shutdown : bool,
	output_thread : Option<thread::JoinHandle<()>>,
	task_queue : mpsc::Sender<OutputAgentMessage>,
}

impl OutputAgent {
	
	pub fn spawn_new<OUT, OUT_P>(out_stream_provider: OUT_P) 
		-> OutputAgent
	where 
		OUT: io::Write + 'static, 
		OUT_P : FnOnce() ->Box<OUT> + Send + 'static 
	{
		let (tx, rx) = mpsc::channel::<OutputAgentMessage>();
		
		let output_thread = thread::spawn(move || {
			
			let mut out_stream : Box<OUT> = out_stream_provider();
			
			loop {
				let task_message = rx.recv();
				if let Err(err) = task_message {
					// BM: Should we really panic if agent has not shutdown explicitly?
					panic!("Error, task queue channel closed without explicit agent shutdown: {:?}", err);
				}
				let task_message = task_message.unwrap();
				
				match task_message {
					OutputAgentMessage::Shutdown => { 
						return; 
					}
					OutputAgentMessage::Task(task) => {
						task(&mut out_stream);
					}
				}
			}
			
        });
		
		OutputAgent { is_shutdown : false, task_queue : tx,  output_thread : Some(output_thread) } 	
	}
	
	pub fn is_shutdown(&self) -> bool {
		self.is_shutdown
	}
	
	pub fn submit_task(& self, task : OutputAgentTask) {
		self.task_queue.send(OutputAgentMessage::Task(task))
			.expect(ERR_SEND_TASK_FAILED);
	}
	
	pub fn request_shutdown(&mut self) {
		if !self.is_shutdown {
			self.is_shutdown = true;
			// send shutdown message
			self.task_queue.send(OutputAgentMessage::Shutdown)
				.expect(ERR_SEND_TASK_FAILED);
		}
	}
	
	pub fn shutdown_and_soft_join(&mut self) -> thread::Result<()> {
		self.request_shutdown();
		
		let output_thread = std::mem::replace(&mut self.output_thread, None);
		
		if let Some(output_thread) = output_thread {
			output_thread.join()
		} else {
			Ok(())
		}
	}
	
	pub fn shutdown_and_join(&mut self) {
		if let Err(err) = self.shutdown_and_soft_join() {
			// re-panic
			panic!(err);
		}
	}
	
}

impl Drop for OutputAgent {
	
	fn drop(&mut self) {
		assert!(self.is_shutdown());
		// We shutdown ourselves, but I don't that a good style to do in drop,
		// since shutdown is a blocking operation
	}
	
}

/* -----------------  ----------------- */

#[cfg(test)]
use util::tests::*;

#[test]
fn test_OutputAgent() {
	// FIXME: try to make Arc
	let output = new(vec![]);
	let mut agent = OutputAgent::spawn_new(move || output);
	
	agent.submit_task(Box::new(| out_stream | { 
		writeln!(out_stream, "Writing response.").unwrap();
	}));
	
	agent.shutdown_and_join();
	// Test re-entrance
	agent.shutdown_and_join();
//	assert_equal(String::new(), String::from_utf8(output).unwrap());

	{
		// Test with stdout
		let mut agent = OutputAgent::spawn_new(|| Box::new(std::io::stdout()));
		agent.shutdown_and_join();
	}
	
}