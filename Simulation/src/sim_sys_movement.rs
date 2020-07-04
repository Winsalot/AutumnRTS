// File contains main movement syustems
use crate::sim_unit_base_components::*;
use crate::sim_ecs::*;
use hecs::*;
use crate::messenger::*;
use crate::sim_fix_math::*;


/// Simple destination update from messages
pub fn sys_input_dest(sim: &mut SimState){

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
				let dest_comp = sim.ecs.get_mut::<DestinationComp>(Entity::from_bits(id.get().clone()));
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

// update next position, that moves unit closer to destination
pub fn sys_set_next_pos(sim: &mut SimState){

	type ToQuery<'a> = (
		// &'a IdComp, 
		&'a PositionComp, 
		&'a mut NextPosComp, 
		&'a DestinationComp, 
		&'a SpeedComponent
		);
	
	let ecs = &mut sim.ecs;

	'query_loop: for (_id, (pos, next_pos, dest, speed)) in &mut ecs.query::<ToQuery>(){
		
		// is there somewhere to move?
		if dest.get_dest() == pos.get_pos() {
			continue 'query_loop;
		}

		let distance = Pos::dist(pos.get_pos(), dest.get_dest());

		if distance == 0 {
			next_pos.set_pos(*dest.get_dest());
			continue 'query_loop;
		}

		let dx = (*pos.get_pos() - *dest.get_dest()) / distance;
		let n_next_pos = *pos.get_pos() - dx * (*speed.get_speed()).min(distance);

		next_pos.set_pos(n_next_pos);

	}
}

pub fn sys_collision_pred(sim: &mut SimState){
	// brute force collision detection. Should probably optimize this sometime in the future
	// basically go over every entity with collision and position and make sure it doesnt collide with anything on next move
	type ToQuery0<'a> = (
		&'a NextPosComp, 
		&'a CollComp,
		);

	let ecs = &mut sim.ecs;

	let mut non_move_entities: Vec<Entity> = vec![];

	'nested_loop: for (id0, (next_pos0, coll0)) in &mut ecs.query::<ToQuery0>(){
		for (id1, (next_pos1, coll1)) in &mut ecs.query::<ToQuery0>(){
			if id1==id0{
				continue 'nested_loop;
			}
			// does id0 collide with anything?
			let dist = Pos::dist(next_pos0.get_pos(), next_pos1.get_pos());
			if dist <= (coll0.get_r() + coll1.get_r()){
				//next_pos0.set_pos(*pos0.get_pos());
				non_move_entities.push(id0.clone());
			}
		}
	}

	type ToQuery1<'a> = ( 
		&'a PositionComp,
		&'a mut NextPosComp,
		);

	for i in 0..non_move_entities.len(){
		let entity = non_move_entities[i];

		let mut query =  ecs.query_one::<ToQuery1>(entity).unwrap();
		let (pos, next_pos) =query.get().unwrap();

		// set next pos to current position:u
		next_pos.set_pos(*pos.get_pos());
	}

}

// Move to valid next position
pub fn sys_set_pos(sim: &mut SimState){
	// Updates unit positions to next position
	// Also generates engine messages
	type ToQuery<'a> = (
		&'a IdComp, 
		&'a mut PositionComp, 
		&'a NextPosComp, 
		);

	let ecs = &mut sim.ecs;

	'query_loop: for (_, (id, pos, next_pos)) in &mut ecs.query::<ToQuery>(){
		
		if next_pos.get_pos() == pos.get_pos() {
			continue 'query_loop;
		}

		let msg: EngineMessage;

		pos.set_pos(*next_pos.get_pos());
		msg = EngineMessage::ObjMove(*id, *pos.get_pos());

		sim.send_batch.push(msg);
	}
}
