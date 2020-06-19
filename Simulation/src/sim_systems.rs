use crate::sim_unit_base_components::*;
//use crate::sim_state_components::*;
use crate::sim_ecs::*;
use hecs::*;
use crate::messenger::*;
use crate::sim_fix_math::{Pos, FixF};

// this module contains ALL used systems (for now)


/*

// Initialise singleton entity
pub fn init_simulation(ecs: &mut World, fps_limit: u32) -> Entity {

	let mut sim_state = EntityBuilder::new();

	sim_state.add(FpsComp::new(20, fps_limit));
	sim_state.add(TickComp::new());
	sim_state.add(SimStateComp::new());
	sim_state.add(MessageComp::new());

	let state_entity = ecs.spawn(sim_state.build());
	
	state_entity
}

*/
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
				let mut new_unit = plc_unit(pos, FixF::from_num(2));
				let e = sim.ecs.spawn(new_unit.build());

				let msg = EngineMessage::ObjSpawn(e.to_bits(), pos);

				sim.send_batch.push(msg);
			},
			_ => {}
		}
	}
}

pub fn input_update_destinations(sim: &mut SimState){

	let inbox = &mut sim.inbox;

	let (dest_msg, rest): (Vec<RenderMessage>, Vec<RenderMessage>) = inbox
		.clone()
		.iter()
		.partition(|&msg| match msg {
			RenderMessage::Destination(..) => true,
			_ => false,
		});

	*inbox = rest;
	for i in 0..dest_msg.len(){
		match dest_msg[i]{
			RenderMessage::Destination(id, pos) => {
				let dest_comp = sim.ecs.get_mut::<DestinationComp>(Entity::from_bits(id));
				if let Ok(mut dest_comp) = dest_comp {
					dest_comp.set_dest(pos);
					let msg = EngineMessage::ObjDest(id, pos);
					sim.send_batch.push(msg);
				}
			},
			_ => {}
		}
	}
}

pub fn update_positions(sim: &mut SimState){
	// Updates unit positions
	type ToQuery<'a> = (&'a mut PositionComp, &'a DestinationComp, &'a SpeedComponent);
	let ecs = &mut sim.ecs;
	'query_loop: for (id, (pos, dest, speed)) in &mut ecs.query::<ToQuery>(){
		if dest.get_dest() == pos.get_pos() {
			continue 'query_loop;
		}

		let msg: EngineMessage;

		let distance = Pos::dist(pos.get_pos(), dest.get_dest());

		if distance == 0 {
			let new_pos = dest.get_dest();
			pos.set_pos(*new_pos);
			msg = EngineMessage::ObjMove(id.to_bits(), *new_pos);
			sim.send_batch.push(msg);
			continue 'query_loop;
		}

		let dx = (*pos.get_pos() - *dest.get_dest()) / distance;
		let new_pos = *pos.get_pos() - dx * (*speed.get_speed()).min(distance);

		msg = EngineMessage::ObjMove(id.to_bits(), new_pos);
		sim.send_batch.push(msg);
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

pub fn send_messages(sim: &mut SimState) {
	sim.messenger.send(sim.send_batch.clone());
	sim.send_batch = vec![];
}

pub fn end_tick(sim: &mut SimState) {
	sim.current_tick += 1;
	sim.fps_counter.limit_fps(sim.fps_limit);
	sim.fps_counter.tick();
}