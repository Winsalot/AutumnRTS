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
	// 	If nothing found then:
	// 		Find closest enemy structure and target it.
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
	let mut trg_list: Vec<(Entity, Option<UId>)> = vec![]; 

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
		'inner: for (_, (id1, pos1)) in sim.ecs.query::<TergetQuery>().iter() {
			if id0.get_owner().get_team() == id1.get_owner().get_team(){
				// Units are allies
				continue 'inner;
			}

			let dist = pos0.get_pos().dist(pos1.get_pos());
			if dist < min_dist {
				min_dist = dist;
				trg_new = Some(*id1.get());
			}
		}

		trg_list.push((
			*sim.res.id_map.get(id0.get()).unwrap(),
			trg_new,
			));
	}

	// Update targets (separated because borrow checker is mean)
	for (unit, new_target) in trg_list {
		let mut trg_comp = sim.ecs.get_mut::<TargetComp>(unit).unwrap();
		match new_target {
			Some(trg_uid) => {
				trg_comp.set_trg(ObjTarget::Entity(trg_uid));
			},
			None => {
				trg_comp.set_trg(ObjTarget::None);
			}
		}
		
	}
}

pub fn order_assign_targets(sim: &mut SimState){
	// Use message to set targets. Make sure order are valid and targets exist.
}