use crate::sim_components::sim_unit_base_components::PositionComp;
use crate::placeholder_entities::plc_projectile;
use crate::common::*;
use crate::sim_components::sim_unit_base_components::IdComp;
use crate::sim_components::targeting_comp::TargetComp;
use crate::sim_components::unitstate_comp::UnitStateComp;
use crate::sim_components::weapon_comp::WeaponComp;
use crate::sim_ecs::SimState;
use crate::sim_weapon_list::*;
use crate::sim_fix_math::*;

pub fn sys_fire_weapons(sim: &mut SimState) {
    // First update weapon states:
    type ToQuery<'a> = (
        &'a IdComp,
        &'a UnitStateComp,
        &'a PositionComp,
        &'a TargetComp,
        &'a mut WeaponComp,
    );

    // Firing entity, weapon effect, target:
    let mut to_fire_weapon: Vec<(UId, Pos, WeaponType, ObjTarget)> = vec![];

    for (_, (id_comp, state_comp, pos_comp, trg_comp, wep_comp)) in sim.ecs.query::<ToQuery>().iter() {
        match state_comp.get_state() {
            UnitState::FireWeapons(weps_to_fire) => {
                wep_comp.update_wep_states(*weps_to_fire);

                // fire each available weapon:
                'wep_loop: for i in 0..(N_WEAPON_CAP as usize) {
                    match (weps_to_fire[i], wep_comp.get_wep(i).get_state()) {
                        (true, WeaponState::Firing { .. }) => {
                            // FIRE WEAPON HERE!!!!!
                            to_fire_weapon.push((
                                *id_comp.get_id(),
                                *pos_comp.get_pos(),
                                *wep_comp.get_wep(i).get_type(),
                                *trg_comp.get_trg(),
                            ));
                        }
                        _ => continue 'wep_loop,
                    }
                }
            }
            _ => {
                // nothing firing this turn:
                wep_comp.update_wep_states([false; N_WEAPON_CAP as usize]);
                continue;
            }
        }
    }

    // iterate over weapons to fire and spawn projectiles!
    for (shooter_id, spawn_pos, wep_type, fire_trg) in to_fire_weapon.iter() {
        plc_projectile(sim, shooter_id, spawn_pos, wep_type, fire_trg, FixF::from_num(2));
    }
}
