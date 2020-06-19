// game loop struct. This is implemented as class in GDScript
use std::thread::{JoinHandle};
use gdnative::*;
use crate::sim_fix_math::{Pos, FixF};
use snowflake::ProcessUniqueId;
use crate::sim_gameloop;
use crate::messenger::*;


#[derive(gdnative::NativeClass)] // probably important
#[inherit(gdnative::Node)] // idk, gonna keep it here though
pub struct RustBridge_GameLoop {
	sim_handle: Option<JoinHandle<()>>,
	messenger: Option<RendMessenger>,
}

#[gdnative::methods]
impl RustBridge_GameLoop {

	fn _init(_owner: gdnative::Node) -> Self {

		RustBridge_GameLoop{
			sim_handle: None,
			messenger: None,
		}
	}

	#[export]
	fn _ready(&mut self, owner: gdnative::Node){
	}

	#[export]
	fn _process(&mut self, _owner: gdnative::Node, _delta: f64){
	}

	#[export]
	fn start_loop(&mut self, _owner: gdnative::Node,
		n_fps_avg: i32,
		fps_limit: i32) {

		let (sim_handle, rend_messenger) = sim_gameloop::start_loop(
			n_fps_avg as usize,
			fps_limit as u32);

		self.sim_handle = Some(sim_handle);
		self.messenger = Some(rend_messenger);

		// Debug part:
		godot_print!("Game loop succesfully started with params {:?}, {:?}",
			n_fps_avg, fps_limit);

	}

// Return type must be convertible into variant. Little problematic :)
/*
	#[export]
	fn give_messenger(&mut self, owner: gdnative::Node) ->
		Option<RendMessenger> {

		std::mem::replace(&mut self.messenger, None)

	}
*/

	#[export]
	fn _exit_tree(&mut self, _owner: gdnative::Node) {

		//TODO: should send Break message
		//self.messenger.send(vec![RenderMessage::Break]);
		if let Some(sim_handle) = std::mem::replace(&mut self.sim_handle, None){
			sim_handle.join();
		}
		godot_print!("Node termination succesful");
	}

} 