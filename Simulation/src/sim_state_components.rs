//use hecs::*;
//use crate::sim_fix_math::{Pos, FixF};
use crate::fpscounter::*;
use crate::messenger::*;


// current tick component
pub struct TickComp {
	tick: u32,
}

// Fps component
pub struct FpsComp {
	counter: FpsCounter,
	n_fps_avg: usize,
	fps_limit: u32,
}

/// Messenger component
/// Unfortunately, messenger itself will exist outside ECS
pub struct MessageComp {
	inbox: Vec<RenderMessage>,
	send_batch: Vec<EngineMessage>,
}

// Is simulation still running?
pub struct SimStateComp {
	pub running: bool,
}

impl TickComp {
	pub fn new() -> Self {
		TickComp{tick: 0}
	}
}

impl FpsComp {
	pub fn new(n_avg: usize, fps_limit: u32) -> Self {
		let fps_counter = FpsCounter::new(n_avg);
		FpsComp{
			counter: fps_counter,
			n_fps_avg: n_avg,
			fps_limit: fps_limit,
		}
	}
}

impl SimStateComp {
	pub fn new() -> Self {
		SimStateComp{running: true}
	}
}

impl MessageComp {
	pub fn new() -> Self {
		MessageComp{
			inbox: vec![],
			send_batch: vec![],

		}
	}

	pub fn add_msg(&mut self, mut msg: Vec<RenderMessage>){
		self.inbox.append(&mut msg);
	}

	pub fn get_inbox(&mut self) -> &mut Vec<RenderMessage> {
		&mut self.inbox
	}

	pub fn add_outgoing(&mut self, mut msg: Vec<EngineMessage>) {
		self.send_batch.append(&mut msg);
	}
}