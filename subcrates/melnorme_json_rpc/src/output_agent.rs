// Copyright 2016 Bruno Medeiros
//
// Licensed under the Apache License, Version 2.0 
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>. 
// This file may not be copied, modified, or distributed
// except according to those terms.

use std;

use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::SendError;
use std::io;

#[allow(unused_imports)]
use util::core::*;

use service_util::MessageWriter;


/* -----------------  ----------------- */

/// Functional interface representing the execution of the Agent
pub trait AgentRunnable {
	
	/// Run the Agent. Must end with a call to `agent_inner.enter_agent_loop()`
	fn run_agent(self, agent_inner: AgentInnerRunner);
	
}

impl<FN : FnOnce(AgentInnerRunner)> AgentRunnable for FN {
	fn run_agent(self, agent_lr: AgentInnerRunner) {
		self(agent_lr)
	}
}

/* ----------------- Output_Agent ----------------- */

pub type OutputAgentTask = Box<Fn(&mut MessageWriter) + Send>;

pub enum OutputAgentMessage {
	Shutdown,
	Task(OutputAgentTask),
}

const ERR_SEND_TASK_FAILED : &'static str 
	= "Failed to send task, Agent receive channel is closed.";

/**

Actor-like, dedicated worker thread that handles writing to an output stream.
Accepts tasks as messages, which are executed by the agent.

Note that the OutputAgent type is not meant to be Sync, it is meant to be synchronized externally,
or more typically, used by one controlling thread only. 

 */
pub struct OutputAgent {
	is_shutdown : bool,
	output_thread : Option<thread::JoinHandle<()>>,
	task_queue : mpsc::Sender<OutputAgentMessage>,
}

impl OutputAgent {
	
	pub fn start_with_provider<OUT, OUT_P>(out_stream_provider: OUT_P) 
		-> OutputAgent
	where 
		OUT: MessageWriter + 'static, 
		OUT_P : FnOnce() -> OUT + Send + 'static 
	{
		Self::start(move |inner_runner: AgentInnerRunner| {
			let mut out_stream : OUT = out_stream_provider();
			
			inner_runner.enter_agent_loop(&mut move |task| {
				task(&mut out_stream); 
			});
		})
	}
	
	
	pub fn start<AGENT_RUNNER : Sized>(agent_runner: AGENT_RUNNER) 
		-> OutputAgent
	where 
		AGENT_RUNNER : AgentRunnable,
		AGENT_RUNNER : Send + 'static,
	{
		let (tx, rx) = mpsc::channel::<OutputAgentMessage>();
		
		let output_thread = thread::spawn(move || {
			agent_runner.run_agent(AgentInnerRunner{ rx : rx });
        });
		
		OutputAgent { is_shutdown : false, task_queue : tx,  output_thread : Some(output_thread) } 	
	}
	
	pub fn is_shutdown(&self) -> bool {
		self.is_shutdown
	}
	
	pub fn try_submit_task(& self, task : OutputAgentTask) -> Result<(), SendError<OutputAgentMessage>> {
		self.task_queue.send(OutputAgentMessage::Task(task))
	}
	
	pub fn submit_task(& self, task : OutputAgentTask) {
		assert!(!self.is_shutdown);
		self.try_submit_task(task).expect(ERR_SEND_TASK_FAILED);
	}
	
	pub fn request_shutdown(&mut self) {
		if !self.is_shutdown {
			self.is_shutdown = true;
			// send shutdown message
			self.task_queue.send(OutputAgentMessage::Shutdown).ok();
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
		if !thread::panicking() {
			// User must have taken care of shutdown itself, otherwise thread is leaked.
			assert!(self.is_shutdown());
		}
	}
	
}

pub struct AgentInnerRunner {
	rx: Receiver<OutputAgentMessage>,
}
impl AgentInnerRunner {
	
	/// Enter agent loop, with given task runner
	pub fn enter_agent_loop<TASK_RUNNER>(self, mut task_runner: &mut TASK_RUNNER,)
	where
		 TASK_RUNNER : FnMut(OutputAgentTask) 
	{
		let mut rx = self.rx;
		Self::run_agent_loop(&mut rx, &mut task_runner);
	}
	
	pub fn run_agent_loop<TASK_RUNNER>(rx: &mut Receiver<OutputAgentMessage>, mut task_runner: &mut TASK_RUNNER,)
	where
		 TASK_RUNNER : FnMut(OutputAgentTask) 
	{
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
					task_runner(task);
				}
			}
		}
	}
}


/* -----------------  ----------------- */

/// Handle a message simply by writing to a io::Write
pub struct IoWriteHandler<T: io::Write>(pub T);

impl<T : io::Write> MessageWriter for IoWriteHandler<T> {
	fn write_message(&mut self, msg: &str) -> Result<(), GError> {
		try!(self.0.write_all(msg.as_bytes()));
		Ok(())
	}
}

#[test]
fn test_OutputAgent() {
	
	use util::tests::*;
	
	let output = vec![];
	let mut agent = OutputAgent::start_with_provider(move || IoWriteHandler(output));
	
	agent.submit_task(new(|msg_writer| {
		msg_writer.write_message("First responde.").unwrap();
	}));
	
	agent.shutdown_and_join();
	// Test re-entrance
	agent.shutdown_and_join();
	
	
	let output = newArcMutex(vec![] as Vec<u8>);
	let output2 = output.clone();
	
	let mut agent = OutputAgent::start(move |inner_runner: AgentInnerRunner| {
		inner_runner.enter_agent_loop(&mut move |task: OutputAgentTask| {
			let mut lock : std::sync::MutexGuard<Vec<u8>> = output2.lock().unwrap();
			task(&mut IoWriteHandler(&mut *lock));
		});
	});
	
	agent.submit_task(new(|msg_writer| {
		msg_writer.write_message("First response.").unwrap();
	}));
	
	agent.shutdown_and_join();
	
	assert_equal(String::from_utf8(unwrap_ArcMutex(output)).unwrap(), "First response.".to_string());
}

// The following code we don't want to run, we just want to test that it compiles
#[cfg(test)]
pub fn test_OutputAgent_API() {
	use std::sync::Arc;
	use std::net::TcpStream;
	use std::sync::Mutex;
	use std::io::Read;
	
	
	// Test with StdOut
	let mut agent = OutputAgent::start_with_provider(|| IoWriteHandler(std::io::stdout()));
	agent.shutdown_and_join();
	
	
	// Test with StdoutLock - lock entire agent loop
	let mut agent = OutputAgent::start(|inner_runner: AgentInnerRunner| {
		let stdout = io::stdout();
		let mut stdoutlock = stdout.lock();
		
		inner_runner.enter_agent_loop(&mut |task: OutputAgentTask| {
			task(&mut IoWriteHandler(&mut stdoutlock));
		});
	});
	agent.shutdown_and_join();
	
	
	// Test with tcp stream
	let stream = Arc::new(Mutex::new(TcpStream::connect("127.0.0.1:34254").unwrap()));
	let stream2 = stream.clone();
	let mut agent = OutputAgent::start(move |inner_runner: AgentInnerRunner| {
		inner_runner.enter_agent_loop(&mut |task: OutputAgentTask| {
			let mut stream = stream2.lock().expect("Re-entered mutex lock");
			task(&mut IoWriteHandler(&mut *stream));
		});
	});
	agent.shutdown_and_join();
	
	{
		let mut stream = stream.lock().expect("Re-entered mutex lock");
		stream.read_to_string(&mut String::new()).expect("failed to read stream");
	}
}