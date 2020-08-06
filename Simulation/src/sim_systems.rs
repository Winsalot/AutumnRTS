use crate::sim_unit_base_components::*;
//use crate::sim_state_components::*;
use crate::sim_ecs::*;
use hecs::*;
use crate::messenger::*;
use crate::sim_fix_math::{Pos, FixF};

// this module contains ALL used systems (for now)


pub fn update_fps_info(sim: &mut SimState){
	let fps = sim.fps_counter.get_fps_simple();
	sim.send_batch.push(EngineMessage::Fps(fps));
}


pub fn receive_messages(sim: &mut SimState) {

	let mut rend_msg = sim.messenger.rec();
	sim.inbox.append(&mut rend_msg);
}

pub fn input_break_check(sim: &mut SimState) -> bool {

	let inbox = &sim.inbox;

	let do_break = inbox.iter().filter(|x| **x == RenderMessage::Break).next().is_some();

	if do_break {
		sim.break_loop = true;
		return true;
	}

	false
}

pub fn input_spawn_unit(sim: &mut SimState) {
	// Reads messages, removes spawn messages from inbox. Spawns units and egnerates messages

	let inbox = &mut sim.inbox;

	let (spawn_msg, rest): (Vec<RenderMessage>, Vec<RenderMessage>) = inbox
		.clone()
		.iter()
		.partition(|&msg| match msg {
			RenderMessage::Spawn(..) => true,
			_ => false,
		});

	*inbox = rest;

	for i in 0..spawn_msg.len(){
		match spawn_msg[i]{
			RenderMessage::Spawn(pos) => {

				// Prevent from spawning outside map:
				if (pos.x < 0) |
					(pos.y < 0) |
					(pos.x > (sim.map.size().0 - 1))|
					(pos.y > (sim.map.size().1 - 1)){
						continue;
					}

				//TODO: coll_rad_tmp should not be hardcoded
				let coll_rad_tmp = FixF::from_num(0.5);

				let mut new_unit = plc_unit(pos, coll_rad_tmp, &mut sim.id_counter);
				let e = sim.ecs.spawn(new_unit.build());

				let id = sim.ecs.get::<IdComp>(e).unwrap();


				let msg = EngineMessage::ObjPosColl(*id, pos, coll_rad_tmp);
				sim.send_batch.push(msg);
				
			},
			_ => {}
		}
	}
}


pub fn clear_inbox(sim: &mut SimState) -> Option<Vec<RenderMessage>>{
	// clears unread rendermessages.
	// Sends returns unused messages
	let mut  ret: Option<Vec<RenderMessage>> = None;
	if sim.inbox.len() != 0 {
		ret = Some(sim.inbox.clone());
		sim.inbox = vec![];
	}
	ret
}

pub fn sys_init_send_map(sim: &mut SimState){
	let mut msg = sim.map.to_message();
	sim.send_batch.append(&mut msg);
}

pub fn send_messages(sim: &mut SimState) {
	sim.messenger.send(sim.send_batch.clone());
	sim.send_batch = vec![];
}

pub fn end_tick(sim: &mut SimState) {
	sim.current_tick += 1;
	sim.fps_counter.limit_fps(sim.fps_limit);
	sim.fps_counter.tick();
}

