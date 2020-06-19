use crate::messenger::{RenderMessage, RendMessenger};
use gdnative::*;

#[derive(gdnative::NativeClass)] // probably important
#[inherit(gdnative::Node)] // idk, gonna keep it here though

struct Communicator {
	fps: u64, // temporary field
	messenger: RendMessenger,
}


// This was in example. Idk why it's here
unsafe impl Send for Communicator {}

#[gdnative::methods]
impl Communicator {

	// bacisally new() function. 
	//Owner is some node which will initialise the communicator
	fn _init(_owner: gdnative::Node) -> Self {
		Communicator{}
	}

	fn _ready(_owner: gdnative::Node){
		// Fuckin does nothing in my case
	}

	#[export]
	unsafe fn sim_fps(&mut self, _owner: gdnative::Node) -> u64{
		self.fps
	}

	pub fn update_fps_info(&mut self, msg)

}


fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<Communicator>();
}


godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();