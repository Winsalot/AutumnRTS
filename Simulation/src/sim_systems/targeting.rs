use crate::sim_components::unitstate_comp::UnitStateComp;
use crate::sim_fix_math::Pos;

use crate::common::*;
use crate::sim_components::sim_unit_base_components::IdComp;
use crate::sim_components::sim_unit_base_components::PositionComp;
use crate::sim_components::targeting_comp::*;
use crate::sim_ecs::SimState;

// TODO: this function should be moved to somewhere else. This is not a system, but rather a helper function aka. SUBSYSTEM.
/// Helper function that returns the position of target
pub fn target_to_pos(sim: &SimState, trg: &ObjTarget) -> Option<Pos> {
    match trg {
        ObjTarget::None => {
            return Option::None;
        }
        ObjTarget::Position(pos) => return Some(*pos),
        ObjTarget::Entity(uid) => {
            // Assume this doesn't panic because target was validated in behaviour AI.
            if let Some(entity) = sim.res.id_map.get(uid) {
                // Return Position if entity has Position component
                if let Ok(pos_comp) = sim.ecs.get::<PositionComp>(*entity) {
                    return Some(*pos_comp.get_pos());
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }
    }
}

#[cfg(test)]
mod order_and_state_tests {
    use crate::common::*;
    use crate::messenger::*;
    use crate::sim_ecs::*;
    use crate::sim_fix_math::*;
    use crate::sim_gameloop::first_tick;
    use crate::sim_gameloop::run_single_tick;
    use crate::sim_map::Map;
    use crate::sim_systems::targeting::target_to_pos;

    #[test]
    fn pos_trg_to_pos() {
        // cargo test pos_trg_to_pos

        let (sim_messenger, rend_messenger) = create_messenger();

        let map = Map::make_test_map();
        let mut sim = SimState::new(map, sim_messenger, 1, 10);

        //run first 2 ticks:
        first_tick(&mut sim);
        rend_messenger.rec();
        run_single_tick(&mut sim);

        let msg0 = RenderMessage::SpawnSmart(0, Pos::from_num(1, 1));
        rend_messenger.send(vec![msg0]);
        run_single_tick(&mut sim);

        let pos1 = target_to_pos(&sim, &ObjTarget::Position(Pos::from_num(1, 1)));

        assert_eq!(pos1, Some(Pos::from_num(1, 1)));

        let pos2 = target_to_pos(&sim, &ObjTarget::Entity(0));
        assert_eq!(pos2, Some(Pos::from_num(1, 1)));

        let pos3 = target_to_pos(&sim, &ObjTarget::Entity(10));
        assert_eq!(pos3, None);
    }
}

// #[deprecated(since = "yesterday", note = "Targeting no longer works independently")]
// pub fn _auto_assign_targets(sim: &mut SimState){
// 	// Loop over units with Pos And Targeting Comp.
// 	// For each:
// 	// 	If trg_mode order then check if target still exists.
// 	// 	Find closest enemy unit and add its UId as target
// 	// 	Else None
// 	// That's it :)

// 	type UnitQuery<'a> = (
//         &'a IdComp,
//         &'a PositionComp,
//         &'a TargetComp,
//         );

// 	type TergetQuery<'a> = (
//         &'a IdComp,
//         &'a PositionComp,
//         );

// 	// Here (Entity, Entity) = (unit, new_target)
// 	let mut trg_list: Vec<(UId, Option<UId>, Option<Pos>)> = vec![];

// 	'outer: for (_, (id0, pos0, trg)) in sim.ecs.query::<UnitQuery>().without::<UnitStateComp>().iter() {
// 		if trg.mode_is_order(){
// 			// check if target exists:
// 			match trg.get_trg() {
// 				ObjTarget::None => {},
// 				ObjTarget::Position(..) => {continue 'outer;},
// 				ObjTarget::Entity(trg_id) => {

// 					// Id map should contain only existing entities:
// 					if sim.res.id_map.contains_key(trg_id){
// 						continue 'outer;
// 					}

// 					// In theory this part should also make sure that target is visible.
// 					// But there is no LOS yet, so whatever.

// 				}
// 			}
// 		}

// 		// Yes, this is grid search:
// 		// TODO: optimise this shit:
// 		let mut min_dist = trg.get_range().clone();
// 		let mut trg_new: Option<UId> = None;
// 		let mut trg_pos: Option<Pos> = None;
// 		'inner: for (_, (id1, pos1)) in sim.ecs.query::<TergetQuery>().iter() {
// 			if id0.get_owner().get_team() == id1.get_owner().get_team(){
// 				// Units are allies
// 				continue 'inner;
// 			}

// 			let dist = pos0.get_pos().dist(pos1.get_pos());
// 			if dist <= min_dist {
// 				min_dist = dist;
// 				trg_new = Some(*id1.get());
// 				trg_pos = Some(*pos1.get_pos());
// 			}
// 		}

// 		trg_list.push((
// 			//*sim.res.id_map.get(id0.get()).unwrap(),
// 			*id0.get(),
// 			trg_new,
// 			trg_pos,
// 			));
// 	}

// 	// Update targets (separated because borrow checker is mean)
// 	for (unit, trg_new, trg_pos) in trg_list {
// 		let entity = *sim.res.id_map.get(&unit).unwrap();
// 		let mut trg_comp = sim.ecs.get_mut::<TargetComp>(entity).unwrap();
// 		match trg_new {
// 			Some(trg_uid) => {

// 				// Commented out bcuase targets position can change. Thus update is sent every frame regardless.
// 				// if trg_comp.get_trg() != &ObjTarget::Entity(trg_uid) {
// 				// 	trg_comp.set_trg(ObjTarget::Entity(trg_uid));
// 				// 	let msg = SimMsg::StateChange(
// 				// 		SimStateChng::ObjTargetPos(unit,trg_pos.unwrap())
// 				// 		);
// 				// }

// 				trg_comp.set_trg(ObjTarget::Entity(trg_uid));
// 				let msg = SimMsg::StateChange(
// 					SimStateChng::ObjTargetPos(unit,trg_pos.unwrap())
// 					);

// 				sim.res.send_batch.push(msg);

// 			},
// 			None => {
// 				if trg_comp.get_trg() != &ObjTarget::None {
// 					trg_comp.set_trg(ObjTarget::None);
// 					let msg = SimMsg::StateChange(
// 						SimStateChng::ObjTargetNone(unit)
// 						);

// 					sim.res.send_batch.push(msg);
// 				}
// 			}
// 		}

// 	}
// }
