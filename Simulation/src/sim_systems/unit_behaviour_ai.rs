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
        &'a UnitStateComp,
    );
    // This is all borrow checker's fault. A list for entities who have fulfilled the requirement of their current order.
    let mut to_update_orders: Vec<UId> = vec![];

    for (entity, (id, unit_state)) in &mut sim.ecs.query::<ToQuery>(){
    	// Three parts in this:

    	// 1. Check if current order's conditions are satisfied and update to next order in queue:
    	match unit_state.get_current_order() {
    		UnitOrder::None => {
    			// Nothing happens here
    		},
    		UnitOrder::MoveTo(moveto_pos) => {
    			// TODO: account for cases where position is unreachable (eg. occupied or on blocking tile). However, this validation should probably happen in RenderMessage -> UnitOrder step.
    			if let Ok(pos_comp) = sim.ecs.get::<PositionComp>(entity){
    				if pos_comp.get_pos().dist(moveto_pos) == 0 {
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
    		if let Ok(mut unit_state) = sim.ecs.get_mut::<UnitStateComp>(*entity) {
    			unit_state.current_order_completed();
    		}
    	}
    }
}


fn order_to_unitstate(sim: &mut SimState) {
	// Takes current order and sets appropriate unit state.
	type ToQuery<'a> = (
        &'a IdComp,
        &'a UnitStateComp,
    );

    let mut to_update_states: Vec<(UId, UnitState)> = vec![];

	for (_, (id, unit_state)) in &mut sim.ecs.query::<ToQuery>(){
		match unit_state.get_current_order() {
    		UnitOrder::None => {
    			if unit_state.get_state() != &UnitState::Idle {
    				to_update_states.push((*id.get_id(), UnitState::Idle));
    			}
    		},
    		UnitOrder::MoveTo(..) => {
    			if unit_state.get_state() != &UnitState::Move {
    				to_update_states.push((*id.get_id(), UnitState::Move));
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
}
