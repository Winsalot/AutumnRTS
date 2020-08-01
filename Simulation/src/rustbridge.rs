use std::thread::{JoinHandle};
use gdnative::*;
use crate::sim_fix_math::{Pos};
use crate::sim_gameloop;
use crate::messenger::*;
use crate::rustbridge_messages::*;
use crate::sim_unit_base_components::IdComp;




#[derive(gdnative::NativeClass)] 
#[inherit(gdnative::Node)] 
pub struct RustBridge {
	sim_handle: Option<JoinHandle<()>>,
	messenger: Option<RendMessenger>,
	message_batch: Vec<RenderMessage>,
	message_inbox: Vec<EngineMessage>,
}


#[gdnative::methods]
impl RustBridge {
	fn _init(_owner: gdnative::Node) -> Self {
		RustBridge{
			sim_handle: None,
			messenger: None,
			message_batch: vec![],
			message_inbox: vec![],
		}
	}

	#[export]
	fn _ready(&mut self, _owner: gdnative::Node){
	}

	#[export]
	fn start_loop(&mut self, _owner: gdnative::Node,
		n_fps_avg: i32,
		fps_limit: i32) {


		let (sim_handle, rend_messenger) = sim_gameloop::start_loop(
			fps_limit as u32);

		self.sim_handle = Some(sim_handle);
		self.messenger = Some(rend_messenger);

		// Debug part:
		godot_print!("Game loop succesfully started with params {:?}, {:?}",
			n_fps_avg, fps_limit);
	}


	#[export]
	fn _exit_tree(&mut self, _owner: gdnative::Node) {

		if let Some(rend_msg) = &self.messenger {
			rend_msg.send(vec![RenderMessage::Break]);
		}
		//self.messenger.send(vec![RenderMessage::Break]);
		if let Some(sim_handle) = std::mem::replace(&mut self.sim_handle, None){
			sim_handle.join().unwrap();
		}
		godot_print!("Node termination succesful");
	}

	#[export]
	fn deliver_input(&mut self, _owner: gdnative::Node) {
		
		let msg = std::mem::replace(&mut self.message_batch, vec![]);
		let len_tmp = msg.len();
		if let Some(rend_msg) = &self.messenger {
			rend_msg.send(msg);
		}
		if len_tmp > 0 {
			godot_print!("message of length {} sent to simulation", len_tmp);
		}
	}

	#[export]
	fn receive_sim_messages(&mut self, _owner: gdnative::Node){
		if let Some(rend_msg) = &self.messenger {
			self.message_inbox.append(&mut rend_msg.rec());
			godot_print!("{:?}", self.message_inbox);
		}

	}

	#[export]
	fn get_msg_spawn(&mut self, _owner: gdnative::Node) -> Variant {
		inbox_drain_spawn(&mut self.message_inbox).to_variant()
	}

	#[export]
	fn get_msg_move(&mut self, _owner: gdnative::Node) -> Variant {
		inbox_drain_move(&mut self.message_inbox).to_variant()
	}

	#[export]
	fn get_msg_dest(&mut self, _owner: gdnative::Node) -> Variant {
		inbox_drain_dest(&mut self.message_inbox).to_variant()
	}


	#[export]
	fn send_msg_move(&mut self, _owner: gdnative::Node,
		id: u64, xy: Vector2){
		let pos: Pos = Pos::from_num(xy.x, xy.y);
		let msg = RenderMessage::Destination(IdComp::from(id), pos);
		self.message_batch.push(msg);
	}

	#[export]
	fn get_msg_fps(&mut self, _owner: gdnative::Node) -> Variant {
		inbox_drain_fps(&mut self.message_inbox).to_variant()
	}

	#[export]
	fn get_msg_map(&mut self, _owner: gdnative::Node) -> Variant {
		inbox_drain_map_layout(&mut self.message_inbox).to_variant()
	}

	#[export]
	fn clear_inbox(&mut self, _owner: gdnative::Node) -> usize {
		let ret = self.message_inbox.len();
		if ret > 0 {
			godot_print!("messages to clear: {:?}", self.message_inbox);
		}
		self.message_inbox = vec![];
		ret
	}

	#[export]
	fn tmp_spawn_obj(&mut self, _owner: gdnative::Node, xy: Vector2) {
		let pos: Pos = Pos::from_num(xy.x, xy.y);
		let msg = RenderMessage::Spawn(pos);
		self.message_batch.push(msg);
	}


}
