use crate::sim_components::sim_unit_base_components::IdComp;
use crate::sim_ecs::SimState;
use crate::common::*;

pub fn is_valid(sim: &SimState, player: &PId, unit: &UId) -> bool {
	let entity = sim.res.id_map.get(&unit);
	match entity {
		None => {
			return false;
		},
		Some(entity) => {
			let id_comp = sim.ecs.get::<IdComp>(*entity);
			if let Ok(id_comp) = id_comp {
				if player == &id_comp.get_owner().get_id(){
					return true;
				} 
			}
			return false;
		}
	}
}