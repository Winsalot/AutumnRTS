use crate::sim_fix_math::Pos;
use hecs::Entity;
use crate::sim_components::sim_unit_base_components::IdComp;
use crate::sim_components::sim_unit_base_components::PositionComp;
use crate::sim_components::targeting_comp::*;
use crate::common::*;
use crate::sim_ecs::SimState;

pub fn auto_assign_targets(sim: &mut SimState){
	// Loop over units with Pos And Targeting Comp.
	// For each:
	// 	If trg_mode order then check if target still exists.
	// 	Find closest enemy unit and add its UId as target
	// 	Else None
	// That's it :)

	type UnitQuery<'a> = (
        &'a IdComp,
        &'a PositionComp,
        &'a TargetComp,
        );

	type TergetQuery<'a> = (
        &'a IdComp,
        &'a PositionComp,
        );

	// Here (Entity, Entity) = (unit, new_target)
	let mut trg_list: Vec<(UId, Option<UId>, Option<Pos>)> = vec![]; 

	'outer: for (_, (id0, pos0, trg)) in sim.ecs.query::<UnitQuery>().iter() {
		if trg.mode_is_order(){
			// check if target exists:
			match trg.get_trg() {
				ObjTarget::None => {},
				ObjTarget::Position(..) => {continue 'outer;},
				ObjTarget::Entity(trg_id) => {

					// Id map should contain only existing entities:
					if sim.res.id_map.contains_key(trg_id){
						continue 'outer;
					}

					// In theory this part should also make sure that target is visible.
					// But there is no LOS yet, so whatever.

				}
			}
		}


		// Yes, this is grid search:
		// TODO: optimise this shit:
		let mut min_dist = trg.get_range().clone();
		let mut trg_new: Option<UId> = None;
		let mut trg_pos: Option<Pos> = None;
		'inner: for (_, (id1, pos1)) in sim.ecs.query::<TergetQuery>().iter() {
			if id0.get_owner().get_team() == id1.get_owner().get_team(){
				// Units are allies
				continue 'inner;
			}

			let dist = pos0.get_pos().dist(pos1.get_pos());
			if dist <= min_dist {
				min_dist = dist;
				trg_new = Some(*id1.get());
				trg_pos = Some(*pos1.get_pos());
			}
		}

		trg_list.push((
			//*sim.res.id_map.get(id0.get()).unwrap(),
			*id0.get(),
			trg_new, 
			trg_pos,
			));
	}

	// Update targets (separated because borrow checker is mean)
	for (unit, trg_new, trg_pos) in trg_list {
		let entity = *sim.res.id_map.get(&unit).unwrap();
		let mut trg_comp = sim.ecs.get_mut::<TargetComp>(entity).unwrap();
		match trg_new {
			Some(trg_uid) => {

				// Commented out bcuase targets position can change. Thus update is sent every frame regardless.
				// if trg_comp.get_trg() != &ObjTarget::Entity(trg_uid) {
				// 	trg_comp.set_trg(ObjTarget::Entity(trg_uid));
				// 	let msg = SimMsg::StateChange(
				// 		SimStateChng::ObjTargetPos(unit,trg_pos.unwrap())
				// 		);
				// }

				trg_comp.set_trg(ObjTarget::Entity(trg_uid));
				let msg = SimMsg::StateChange(
					SimStateChng::ObjTargetPos(unit,trg_pos.unwrap())
					);

				sim.res.send_batch.push(msg);

			},
			None => {
				if trg_comp.get_trg() != &ObjTarget::None {
					trg_comp.set_trg(ObjTarget::None);
					let msg = SimMsg::StateChange(
						SimStateChng::ObjTargetNone(unit)
						);
					
					sim.res.send_batch.push(msg);
				}
			}
		}
		
	}
}

// pub fn order_assign_targets(sim: &mut SimState){
// 	// Use message to set targets. Make sure order are valid and targets exist.
// }

#[cfg(test)]
mod targeting_test {

	#[test]
	fn find_targets() {

		// run with:
        // cargo test -- --nocapture find_targets
	        use crate::common::*;
	        use crate::sim_fix_math::*;
	        use crate::sim_gameloop::*;
	        use std::time::Duration;
	        use crate::common::SimMsg::StateChange;
	        use crate::common::SimStateChng::ObjTargetPos;

	        let (sim_handle, rend_msg) = start_loop(2, 30);

	        rend_msg.send(vec![RenderMessage::Spawn(0, Pos::from_num(1, 2))]);
	        rend_msg.send(vec![RenderMessage::Spawn(1, Pos::from_num(1, 3))]);

	        // wait 0.5 seconds:
	        ::std::thread::sleep(Duration::from_secs_f32(0.5));

	        // // send messages:
	        // let msg = vec![RenderMessage::UseAbility(
	        //     0,
	        //     0,
	        //     ObjTarget::Position(Pos::from_num(3, 7)),
	        // )];

	        // rend_msg.send(msg);

	        // ::std::thread::sleep(Duration::from_secs_f32(1.0));

	        let inbox = rend_msg.rec();

	        for i in 0..inbox.len() {
	            match inbox[i] {
	                //EngineMessage::StructurePosTmp(..) => {
	                StateChange(ObjTargetPos(..)) => {
	                    println!("{:?}", inbox[i]);
	                }
	                _ => (),
	            }
	        }

	        // end game loop
	        rend_msg.send(vec![RenderMessage::Break]);
	        sim_handle.join().unwrap();

	        println!("test ended");
	    }
}
