use crate::sim_systems::targeting::target_to_pos;
use crate::sim_components::sim_unit_base_components::IdComp;
use crate::sim_components::unitstate_comp::UnitStateComp;
use crate::sim_components::targeting_comp::TargetComp;
use crate::sim_abilities_list::*;
use crate::common::*;

use crate::sim_components::active_ability_comp::ActiveAbilityComp;

use crate::sim_ecs::SimState;

use crate::sim_systems::input_systems::plc_building;

// use num_traits::identities::Zero;
//use hecs::*;

pub fn use_ability(
    sim: &mut SimState, 
    uid: &UId, 
    target: &ObjTarget, 
    ability: &AbilityEffect
    ) -> bool {
    match ability {
        AbilityEffect::BuildSimpleStructure => {
            let ret =  build_simple_structure(sim, uid, target);
            return ret;
        },
        _ => {return false},
    }
}


pub fn sys_abilities_smart(sim: &mut SimState){
    // query units with ability, target and unitstate components.

    type ToQuery<'a> = (
        &'a IdComp,
        &'a UnitStateComp,
        &'a TargetComp,
        &'a ActiveAbilityComp,
    );

    let mut to_cast_abilities: Vec<(UId, AbilityEffect, AbilityID,  ObjTarget)> = vec![];

    for (_, (id_comp, state_comp, trg_comp, abil_comp)) in sim.ecs.query::<ToQuery>().iter() {

        // Only do shit if state is the following.
        if let UnitState::UseAbility(abil_id) = state_comp.get_state(){
            let abil = abil_comp.get_ability(*abil_id);

            // Ability on Cooldown
            // TODO: this check should be performed when validating Order.
            if abil.get_cd_end() >= &sim.current_tick() {
                let msg = SimMsg::Warn(*id_comp.get_owner(), SimWarnMsg::AbilOnCD);
                sim.res.send_batch.push(msg);
                continue;
            }

            // Now all good and lets cast the abilities:
            to_cast_abilities.push((
                *id_comp.get_id(),
                *abil.get_effect(),
                *abil_id,
                *trg_comp.get_trg()
                ));
        } 
    }

    let mut to_update_abil_cds: Vec<(&UId, &AbilityID)> = vec![];

    // now cast abilities:
    for (uid, abil_ef, abil_id, trg) in to_cast_abilities.iter(){
        if use_ability(sim, uid, trg, abil_ef) {

            to_update_abil_cds.push((
                uid, 
                abil_id
                ));

        }
    }

    for (uid, abil_id) in to_update_abil_cds.iter(){
        let entity = sim.res.id_map.get(uid).unwrap();
        let mut abil_comp = sim.ecs.get_mut::<ActiveAbilityComp>(*entity).unwrap();
        abil_comp.start_cooldown(abil_id, &sim.current_tick());
    }
    
}

fn build_simple_structure(
    sim: &mut SimState, 
    id: &UId, 
    trg: &ObjTarget) -> bool {

    // rewrite logic:
    if let Some(pos) = target_to_pos(sim, trg){

        if !sim.map.within(pos) {
            return false;
        }

        let pos0 = pos.round();

        // Likely redundant, but whatever
        let entity = sim.res.id_map.get(&id);
        if entity.is_none() {
            // This makes sure that .unwrap() won't panic
            return false;
        }

        // DOnt build on already blocked tile.
        if sim.map.tile_from_pos(pos0).blocks_path() {
                return false;
        }

        // Don't build on another structure
        if sim.map.map_mem.get_blocked().contains(&pos0.round()) {
            return false;
        }

        // Place building. Using function from input_systems is shitty.
        // But whatever. Gonna fix later.
        plc_building(sim, 0, pos);

        return true;
    }
    return false;
}
