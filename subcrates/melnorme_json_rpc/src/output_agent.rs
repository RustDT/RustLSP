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
use std::io;

#[allow(unused_imports)]
use util::core::*;

use service_util::Handler;

/* ----------------- Output_Agent ----------------- */

pub type OutputAgentTask = Box<Fn(&mut Handler<String, GError>) + Send>;

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
	
	pub fn start_with_provider<OUT, OUT_P>(out_stream_provider: OUT_P) 
		-> OutputAgent
	where 
		OUT: Handler<String, GError> + 'static, 
		OUT_P : FnOnce() -> OUT + Send + 'static 
	{
		Self::start(move |loop_runner: AgentLoopRunner| {
			let mut out_stream : OUT = out_stream_provider();
			
			loop_runner.enter_agent_loop(&mut move |task| {
				task(&mut out_stream); 
			});
		})
	}
	
	pub fn start_with_task_runner<TASK_RUNNER, TASK_RUNNER_P>(task_runner_provider: TASK_RUNNER_P) 
		-> OutputAgent
	where 
		TASK_RUNNER_P : FnOnce() -> TASK_RUNNER, 
		TASK_RUNNER_P : Send + 'static,
		TASK_RUNNER : FnMut(OutputAgentTask),
	{
		Self::start(move |loop_runner: AgentLoopRunner| {
			let mut task_runner = task_runner_provider();
			loop_runner.enter_agent_loop(&mut task_runner);
		})
	}
	
	pub fn start<AGENT_RUNNER>(agent_runner: AGENT_RUNNER) 
		-> OutputAgent
	where 
		AGENT_RUNNER : FnOnce(AgentLoopRunner),
		AGENT_RUNNER : Send + 'static,
	{
		let (tx, rx) = mpsc::channel::<OutputAgentMessage>();
		
		let output_thread = thread::spawn(move || {
			agent_runner(AgentLoopRunner{ rx : rx });
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

pub struct AgentLoopRunner {
	rx: Receiver<OutputAgentMessage>,
}
impl AgentLoopRunner {
	
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

impl<T : io::Write> Handler<String, GError> for IoWriteHandler<T> {
	fn supply(&mut self, msg: &str) -> Result<(), GError> {
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
		msg_writer.supply("First responde.").unwrap();
	}));
	
	agent.shutdown_and_join();
	// Test re-entrance
	agent.shutdown_and_join();
	
	
	let output = newArcMutex(vec![] as Vec<u8>);
	let output2 = output.clone();
	let mut agent = OutputAgent::start_with_task_runner(|| {
		move |task: OutputAgentTask| {
			let mut lock : std::sync::MutexGuard<Vec<u8>> = output2.lock().unwrap();
			task(&mut IoWriteHandler(&mut *lock));
		}
	});
	agent.submit_task(new(|msg_writer| {
		msg_writer.supply("First response.").unwrap();
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
	
	
	// Test with Stdout
	let mut agent = OutputAgent::start_with_provider(|| IoWriteHandler(std::io::stdout()));
	agent.shutdown_and_join();
	
	
	// Test with StdoutLock
	let mut agent = OutputAgent::start_with_task_runner(|| {
		let stdout = io::stdout();
		move |task: OutputAgentTask| {
			task(&mut IoWriteHandler(stdout.lock()));
		}
	});
	agent.shutdown_and_join();
	

	// Test with StdoutLock - lock entire loop
	let mut agent = OutputAgent::start(|loop_runner| {
		let stdout = io::stdout();
		let mut stdoutlock = stdout.lock();
		
		loop_runner.enter_agent_loop(&mut |task: OutputAgentTask| {
			task(&mut IoWriteHandler(&mut stdoutlock));
		});
	});
	agent.shutdown_and_join();
	
	
	// Test with tcp stream
	let stream = Arc::new(Mutex::new(TcpStream::connect("127.0.0.1:34254").unwrap()));
	let stream2 = stream.clone();
	let task_runner = move |task : OutputAgentTask| {
		let mut stream = stream2.lock().expect("Re-entered mutex lock");
		task(&mut IoWriteHandler(&mut *stream));
	};
	
	let mut agent = OutputAgent::start_with_task_runner(|| task_runner );
	agent.shutdown_and_join();
	
	{
		let mut stream = stream.lock().expect("Re-entered mutex lock");
		stream.read_to_string(&mut String::new()).expect("failed to read stream");
	}
}