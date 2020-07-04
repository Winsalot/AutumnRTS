
use crate::messenger::*;

use std::thread::JoinHandle;
use crate::sim_ecs::*;
use crate::sim_systems::*;
use crate::sim_sys_movement::*;

// Starts game loop and returns all the control handles


pub fn start_loop(fps: u32) -> 
	(JoinHandle<()>, RendMessenger) {

	let (sim_messenger, rend_messenger) = create_messenger();

	let sim_handle = std::thread::spawn(move || {

		let messenger = sim_messenger;

		let mut sim = SimState::new(messenger, fps);

		'running: loop {
			// run all systems:
			update_fps_info(&mut sim);
			receive_messages(&mut sim);

			if input_break_check(&mut sim) {
			    break 'running
			}

			input_spawn_unit(&mut sim);
			sys_input_dest(&mut sim);
			sys_set_next_pos(&mut sim);
			sys_collision_pred(&mut sim);
			sys_set_pos(&mut sim);
			clear_inbox(&mut sim);
			send_messages(&mut sim);
			end_tick(&mut sim);


		}


	});

	return (sim_handle, rend_messenger)

}
