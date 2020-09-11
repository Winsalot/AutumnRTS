use crate::sim_components::health_comp::HealthComp;

use crate::sim_components::sim_unit_base_components::*;

use crate::sim_ecs::SimState;

use hecs::Entity;
// use crate::common::*;

// System that regenerates entities
// ANd despawns dead entities

pub fn sys_health(sim: &mut SimState) {
    update_regen(sim);
    remove_dead(sim);
}

fn update_regen(sim: &mut SimState) {
    type ToQuery<'a> = (&'a mut HealthComp,);

    for (_, (hp_comp,)) in sim.ecs.query::<ToQuery>().iter() {
        hp_comp.regen();
    }
}

fn remove_dead(sim: &mut SimState) {
    type ToQuery<'a> = (&'a mut HealthComp,);

    let mut to_despawn: Vec<Entity> = vec![];

    for (e_id, (hp_comp,)) in sim.ecs.query::<ToQuery>().iter() {
        if !hp_comp.is_alive() {
            to_despawn.push(e_id);
        }
    }

    for entity in to_despawn.iter() {
        // Sep brackets to drop immutable borrow after using it.
        {
            // If enty has id component remove it from id_map:
            let id_comp_query = sim.ecs.get::<IdComp>(*entity);
            if let Ok(id_comp) = id_comp_query {
                sim.res.id_map.remove(id_comp.get_id());
            }
        }

        #[allow(unused_must_use)]
        {
            sim.ecs.despawn(*entity);
        }
    }
}
