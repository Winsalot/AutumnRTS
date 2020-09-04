use crate::sim_components::targeting_comp::TargetComp;
use crate::sim_components::sim_unit_base_components::PathComp;
use hecs::Entity;
use crate::sim_fix_math::*;
use crate::sim_components::order_queue_comp::OrderQueueComp;
use crate::sim_components::sim_unit_base_components::PositionComp;
use crate::sim_components::sim_unit_base_components::IdComp;
use crate::sim_components::unitstate_comp::UnitStateComp;
use crate::common::*;
//use crate::sim_fix_math::*;
use crate::sim_ecs::*;


pub fn sys_unit_behaviour_ai(sim: &mut SimState) {
	check_current_order_completion(sim);
	order_to_unitstate(sim);
}

// Big system that takes order and sets it into state according to the current order.
fn check_current_order_completion(sim: &mut SimState){

	type ToQuery<'a> = (
        &'a IdComp,
        // &'a UnitStateComp,
        &'a OrderQueueComp,
    );
    // This is all borrow checker's fault. A list for entities who have fulfilled the requirement of their current order.
    let mut to_update_orders: Vec<UId> = vec![];


    for (entity, (id, unit_orders)) in &mut sim.ecs.query::<ToQuery>(){
    	// Three parts in this:

    	// 1. Check if current order's conditions are satisfied and update to next order in queue:
    	match unit_orders.get_current_order() {
    		UnitOrder::None => {
    			// Nothing happens here
    		},
    		UnitOrder::MoveTo(moveto_pos) => {
    			// TODO: account for cases where position is unreachable (eg. occupied or on blocking tile). However, this validation should probably happen in RenderMessage -> UnitOrder step.
    			if let Ok(pos_comp) = sim.ecs.get::<PositionComp>(entity){
    				if pos_comp.get_pos() == (moveto_pos) {
    					to_update_orders.push(*id.get_id());
    				}
    			}
    		},
    		UnitOrder::Ability(..) => {
    			// Oh boy. Seems like order completion check will spill to other systems.
    		},
    		//_ => {}
    	}
    }

    for unit_id in to_update_orders.iter() {
    	if let Some(entity) = sim.res.id_map.get(&unit_id){
    		if let Ok(mut unit_orders) = sim.ecs.get_mut::<OrderQueueComp>(*entity) {
    			unit_orders.current_order_completed();
    		}

    		if let Ok(mut unit_target) = sim.ecs.get_mut::<TargetComp>(*entity) {
    			unit_target.set_trg(ObjTarget::None);
    		}
    	}
    }
}


fn order_to_unitstate(sim: &mut SimState) {
	// Takes current order and sets appropriate unit state.
	type ToQuery<'a> = (
        &'a IdComp,
        &'a TargetComp,
        &'a OrderQueueComp,
        &'a UnitStateComp,
    );

    let mut to_update_states: Vec<(UId, UnitState)> = vec![];
    let mut to_update_targets: Vec<(UId, ObjTarget)> = vec![];

	for (e_id, (id, unit_target, unit_orders, unit_state)) in /*&mut*/ sim.ecs.query::<ToQuery>().iter(){

		match unit_orders.get_current_order() {

    		UnitOrder::None => {
    			// TODO: here should go check for nerably enemies and shit.
    			// If there are things to do for unit then it should not be idle :D
    			if unit_state.get_state() != &UnitState::Idle {
    				to_update_states.push((*id.get_id(), UnitState::Idle));
    			}

    		},
    		UnitOrder::MoveTo(dest) => {
    			// Check is target corresponds to destination:
    			if unit_target.get_trg() != &ObjTarget::Position(*dest){
    				to_update_targets.push((*id.get_id(), ObjTarget::Position(*dest)));
    			}

    			// check if pathfinding needs to be rerun:
    			match knows_path_to_dest(&sim, &e_id, &dest) {
    				true => {
		    			if unit_state.get_state() != &UnitState::Move {
		    				to_update_states.push((*id.get_id(), UnitState::Move));
		    			}
    				},
    				false => {
		    			if unit_state.get_state() != &UnitState::PathfindAndMove {
		    				to_update_states.push((*id.get_id(), UnitState::PathfindAndMove));
		    			}
    				}
    			}

    		},
    		UnitOrder::Ability(..) => {
    			// TODO: here check range to target. If not in range then Move. If in range, then Use Ability.
    		},
    		//_ => {}
    	}
	}

	for (unit_id, new_state) in to_update_states.iter() {
    	if let Some(entity) = sim.res.id_map.get(&unit_id){
    		if let Ok(mut unit_state) = sim.ecs.get_mut::<UnitStateComp>(*entity) {
    			unit_state.set_state(*new_state);
    		}
    	}
    }

    for (unit_id, new_target) in to_update_targets.iter() {
    	if let Some(entity) = sim.res.id_map.get(&unit_id){
    		if let Ok(mut unit_target) = sim.ecs.get_mut::<TargetComp>(*entity) {
    			unit_target.set_trg(*new_target);
    		}
    	}    	
    }
}




fn knows_path_to_dest(sim: &SimState, entity_id: &Entity, dest: &Pos) -> bool{
	// Check how far away from first node in path 

	type ToQuery<'a> = (
        &'a PositionComp,
        &'a PathComp,
    );

	let mut query = sim.ecs.query_one::<ToQuery>(*entity_id).unwrap();
	if let Some((curr_pos, path)) = query.get() {

		if path.get_path().len() == 0 {
			return false;
		}

		// Unwraps won't panic, because previous IF checks for length:
		if (curr_pos.get_pos().dist(path.get_path().front().unwrap()) <= FixF::from_num(2)) 
			& (dest == path.get_path().back().unwrap()) {
				return true;
			}
	}

	false
}
