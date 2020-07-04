use hecs::*;
use crate::messenger::*;
use crate::fpscounter::*;

// Basically a struct that contains ECS system and system state.
pub struct SimState {
	pub ecs: World,
	pub messenger: SimMessenger,
	pub inbox: Vec<RenderMessage>,
	pub send_batch: Vec<EngineMessage>,
	pub fps_counter: FpsCounter,
	pub fps_limit: u32,
	pub break_loop: bool,
	pub current_tick: u32,
	pub id_counter: u64,
}

impl SimState {
	pub fn new(sim_messenger: SimMessenger, fps_limit: u32) -> Self {
		SimState {
			ecs: World::new(),
			messenger: sim_messenger,
			inbox: vec![],
			send_batch: vec![],
			fps_counter: FpsCounter::new(10),
			fps_limit: fps_limit,
			break_loop: false,
			current_tick: 0,
			id_counter: 0,

		}
	}
}