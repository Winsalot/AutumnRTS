
use crate::messenger::*;
use crate::simulation::*;
use std::thread::JoinHandle;
use crate::sim_ecs::*;
use crate::sim_systems::*;

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
			input_update_destinations(&mut sim);
			update_positions(&mut sim);
			clear_inbox(&mut sim);
			send_messages(&mut sim);
			end_tick(&mut sim);


		}


	});

	return (sim_handle, rend_messenger)

}

/*
pub fn start_loop(fps_n_avg: usize, fps: u32) -> 
	(JoinHandle<()>, RendMessenger){

	let (sim_messenger, rend_messenger) = create_messenger();

	let sim_handle = std::thread::spawn(move || {

		let messenger = sim_messenger;

		let mut simulator = Simulation::new(fps_n_avg);

			'running: loop {
				let rend_msg = messenger.rec();
				
				let (break_loop, sim_msg) = simulator.update_full(rend_msg);

				messenger.send(sim_msg);

			    if break_loop {
			    	break 'running
			    }
				simulator.fps_counter.limit_fps(fps);
			   	simulator.fps_counter.tick();

			   	
			}

		});

	return (sim_handle, rend_messenger)
}

*/


