use std::thread::{JoinHandle};
use gdnative::*;
use crate::sim_fix_math::{Pos};
//use snowflake::ProcessUniqueId;
use crate::sim_gameloop;
use crate::messenger::*;
use crate::rustbridge_messages::*;
use crate::sim_object::*;




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
			n_fps_avg as usize,
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
		id: ObjectID, xy: Vector2){
		let pos: Pos = Pos::from_num(xy.x, xy.y);
		let msg = RenderMessage::Destination(id, pos);
		self.message_batch.push(msg);
	}

	#[export]
	fn get_msg_fps(&mut self, _owner: gdnative::Node) -> Variant {
		inbox_drain_fps(&mut self.message_inbox).to_variant()
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

/*


	#[export]
	fn tmp_print_messages(&mut self, _owner: gdnative::Node){
		godot_print!("{:?}", inbox_drain_spawn(&mut self.message_inbox));
		//self.message_inbox = vec![];
	}

	#[export]
	fn tmp_test(&mut self, _owner: gdnative::Node) -> Variant{
		inbox_drain_spawn(&mut self.message_inbox).to_variant()
	}
*/


/*
	#[export]
	fn tmp_get_object_update(&mut self, _owner: gdnative::Node){

		let (objects, rest): (Vec<EngineMessage>, Vec<EngineMessage>) = self
			.message_inbox
			.clone()
			.iter()
			.partition(|&msg| match msg {
				EngineMessage::Object(..) => true,
				_ => false,
			});

		
		/*
		if let Some(rend_msg) = &self.messenger {
			let msg = rend_msg.rec();
			godot_print!("Received message of length {:?}", msg);
			for i in 0..msg.len(){
				match msg[i]{
					EngineMessage::Object(id, pos, ..) => {
						//godot_print!("Object {:?} exists at {:?}",id, pos);
					},
					_ => {}
				}
			}
		}
	*/
	}
	*/



// LOTS of old methods. Might be used as examples in the future:


/*

	#[export]
	unsafe fn print_mouse_coord(&mut self, _owner: gdnative::Node){
		let mouse_pos = _owner.get_viewport().unwrap().get_mouse_position();
		godot_print!("{:?}, {}", mouse_pos.x, mouse_pos.y);
	}

	#[export]
	unsafe fn print_relative_mouse_coord(&mut self, owner: gdnative::Node){
		// Camera node must be created on rust side :)
		let camera = owner.get_node(NodePath::from_str("Camera2D")).unwrap();
		if let Some(mut camera2d) = camera.cast::<Camera2D>(){
			camera2d.set_anchor_mode(0);
			let mouse_pos = owner.get_viewport().unwrap().get_mouse_position();
			//godot_print!("{:?}", camera2d.get_camera_position());
			godot_print!("Relative mouse pos: {:?}",mouse_pos + camera2d.get_camera_position());
		}
		//let mouse_pos = owner.get_viewport().unwrap().get_mouse_position();
		//godot_print!("{:?}, {}", mouse_pos.x, mouse_pos.y);
	}

	#[export]
	unsafe fn get_relative_mouse_coord(&mut self, owner: gdnative::Node)-> Vector2{
		// Camera node must be created on rust side :). Why though
		let camera = owner.get_node(NodePath::from_str("Camera2D")).unwrap();
		let mut camera2d = camera.cast::<Camera2D>().unwrap();
		camera2d.set_anchor_mode(0);
		let mouse_pos = owner.get_viewport().unwrap().get_mouse_position();
		mouse_pos + camera2d.get_camera_position()
		
	}

	#[export]
	unsafe fn get_autoload_var(&mut self, owner: gdnative::Node){
		let nodepath = NodePath::from_str("/root/PresentationParams");
		let node = owner.get_node(nodepath).unwrap();
		let object = node.to_object();
		let var = object.get(GodotString::from_str("presentation_scale"));
		let var_vec2d = var.try_to_vector2();
		godot_print!("{:?}", var_vec2d);
	}
*/

// same problem with Variant type
/*
	#[export]
	fn give_messenger(&mut self,
		owner: gdnative::Node, 
		messenger: Option<RendMessenger>){

		match messenger {
			Some(msg) => {
				self.messenger = Some(msg);
				godot_print!("Messenger succesfuly added");
			},
			None => {
				godot_print!("'None' Messenger supplied");
			},
			_ => {}
		}
	}
*/
	
/*
	#[export]
	fn _process(&mut self, _owner: gdnative::Node, _delta: f64){
		// send messenges to simulation.
		let msg = std::mem::replace(&mut self.message_batch, vec![]);
		let len_tmp = msg.len();
		self.messenger.unwrap().send(msg);
		if len_tmp > 0 {
			godot_print!("message of length {} sent to simulation", len_tmp);
		}
	}
*/

/*
	#[export]
	unsafe fn process_messages(&mut self, _owner: gdnative::Node){
		let engine_msg = self.messenger.rec();
		for i in 0..engine_msg.len() {
			match engine_msg[i] {
				EngineMessage::Object(id, pos, ..) => {
					godot_print!("Object mesage received");
					let scene = ResourceLoader::godot_singleton()
						.load(
							GodotString::from_str("res://Presentation_Godot/Placeholder_Unit.tscn"),
							GodotString::from_str(""),
							false
							);
						scene
							.and_then(|s| s.cast::<PackedScene>())
							.unwrap().instance(0).unwrap()
							.cast::<gdnative::Node2D>();
							//id.default().test();
					//scene.unwrap().set_name(GodotString::from_str(format!("{}",id)));
					godot_print!("Scene should be spawned");
				},
				_ => {}
			}
		}
	}
*/

	
	/*
	#[export]
	fn message_spawn(&mut self, _owner: gdnative::Node, pos: Vector2){
		let (x, y) = (pos.x/self.scale.0, pos.y/self.scale.0);
		let spawn_pos = Pos::new(FixF::from_num(x), FixF::from_num(y));
		self.message_batch.push(RenderMessage::Spawn(spawn_pos));

	}
	*/

/*
	#[export]
	fn sim_fps(&mut self, _owner: gdnative::Node){
		let sim_msg = self.messenger.unwrap().rec();
		let fps = sim_msg.iter().last();
		match fps {
			Some(EngineMessage::Fps(fps)) => godot_print!("{}", fps),
			_ => {}
		}
	}
*/


/*
let (sim_messenger, rend_messenger) = create_messenger();
// spawn simulation thread:
let sim_handle = std::thread::spawn(move || {
	let messenger = sim_messenger;
	let mut simulator = Simulation::new(20);

		'running: loop {
			let rend_msg = messenger.rec();
			
			let (break_loop, sim_msg) = simulator.update_full(rend_msg);

			messenger.send(sim_msg);

		    if break_loop {
		    	break 'running
		    }
			simulator.fps_counter.limit_fps(60);
		   	simulator.fps_counter.tick();
		}

	});
*/