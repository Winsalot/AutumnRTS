
use crate::messenger::*;
use crate::simulation::*;
use std::thread::JoinHandle;

// Starts game loop and returns all the control handles


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



