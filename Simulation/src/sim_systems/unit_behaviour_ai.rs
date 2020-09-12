use crate::common::*;
use crate::sim_components::active_ability_comp::ActiveAbilityComp;
use crate::sim_components::order_queue_comp::OrderQueueComp;
use crate::sim_components::sim_unit_base_components::IdComp;
use crate::sim_components::sim_unit_base_components::PathComp;
use crate::sim_components::sim_unit_base_components::PositionComp;
use crate::sim_components::targeting_comp::TargetComp;
use crate::sim_components::unitstate_comp::UnitStateComp;
use crate::sim_components::weapon_comp::WeaponComp;
use crate::sim_fix_math::*;
use crate::sim_systems::targeting::target_to_pos;
use hecs::Entity;
//use crate::sim_fix_math::*;
use crate::sim_ecs::*;

pub fn sys_unit_behaviour_ai(sim: &mut SimState) {
    check_current_order_completion(sim);
    order_to_unitstate(sim);
}

// Big system that takes order and sets it into state according to the current order.
fn check_current_order_completion(sim: &mut SimState) {
    type ToQuery<'a> = (&'a IdComp, &'a OrderQueueComp);

    let mut to_update_orders: Vec<UId> = vec![];

    for (entity, (id, unit_orders)) in &mut sim.ecs.query::<ToQuery>() {
        match unit_orders.get_current_order() {
            UnitOrder::None => {}
            UnitOrder::MoveTo(moveto_pos) => {
                // TODO: account for cases where position is unreachable (eg. occupied or on blocking tile). However, this validation should probably happen in RenderMessage -> UnitOrder step.
                if let Ok(pos_comp) = sim.ecs.get::<PositionComp>(entity) {
                    if pos_comp.get_pos() == (moveto_pos) {
                        to_update_orders.push(*id.get_id());
                    }
                }
            }
            UnitOrder::Ability(_abil_id, _abil_trg) => {
                // Assume that if unit started turn with UseAbility, then it was done last turn.
                // Though I find that I really dislike this implementation.
                // Actually this solution is dogshit. Really sucks for abilities that take multiple frames.
                if let Ok(state_comp) = sim.ecs.get::<UnitStateComp>(entity) {
                    match state_comp.get_state() {
                        UnitState::UseAbility(..) => {
                            to_update_orders.push(*id.get_id());
                        }
                        UnitState::UseAbilityFailed => {
                            to_update_orders.push(*id.get_id());
                        }
                        _ => {}
                    }
                    // if *state_comp.get_state() == UnitState::UseAbility(*abil_id){
                    //     to_update_orders.push(*id.get_id());
                    // }
                }
            }
            UnitOrder::ForceAttack(trg) => {
                match trg {
                    ObjTarget::None => {
                        // wtf is even the order?
                        to_update_orders.push(*id.get_id());
                    }
                    ObjTarget::Position(..) => {
                        // Keep attacking ground until ordered otherwise :)
                        continue;
                    }
                    ObjTarget::Entity(uid) => {
                        // Only stop if entity dead:
                        if sim.res.id_map.get(uid).is_none() {
                            to_update_orders.push(*id.get_id());
                        }
                    }
                }
            } // _ => {}
        }
    }

    for unit_id in to_update_orders.iter() {
        if let Some(entity) = sim.res.id_map.get(&unit_id) {
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
    type ToQuery<'a> = (&'a IdComp, &'a OrderQueueComp);

    let mut to_update_states: Vec<(UId, UnitState)> = vec![];
    let mut to_update_targets: Vec<(UId, ObjTarget)> = vec![];

    let mut query = sim.ecs.query::<ToQuery>();

    for (_, (id, unit_orders)) in query.iter() {
        match unit_orders.get_current_order() {
            UnitOrder::None => {
                no_order_behaviour(
                    &sim,
                    id.get_id(),
                    &mut to_update_states,
                    &mut to_update_targets,
                );
            }
            UnitOrder::MoveTo(dest) => {
                moveto_order_behaviour(
                    &sim,
                    id.get_id(),
                    &dest,
                    &mut to_update_states,
                    &mut to_update_targets,
                );
            }
            UnitOrder::Ability(abil_id, abil_trg) => {
                ability_order_behaviour(
                    &sim,
                    id.get_id(),
                    &abil_id,
                    &abil_trg,
                    &mut to_update_states,
                    &mut to_update_targets,
                );
            }
            UnitOrder::ForceAttack(atk_trg) => forceattack_order_behaviour(
                &sim,
                id.get_id(),
                &atk_trg,
                &mut to_update_states,
                &mut to_update_targets,
            ),

            _ => {}
        }
    }

    for (unit_id, new_state) in to_update_states.iter() {
        if let Some(entity) = sim.res.id_map.get(&unit_id) {
            if let Ok(mut unit_state) = sim.ecs.get_mut::<UnitStateComp>(*entity) {
                unit_state.set_state(*new_state);
            }
        }
    }

    for (unit_id, new_target) in to_update_targets.iter() {
        if let Some(entity) = sim.res.id_map.get(&unit_id) {
            if let Ok(mut unit_target) = sim.ecs.get_mut::<TargetComp>(*entity) {
                unit_target.set_trg(*new_target);
            }
        }
    }
}

fn forceattack_order_behaviour(
    sim: &SimState,
    uid: &UId,
    atk_trg: &ObjTarget,
    new_states: &mut Vec<(UId, UnitState)>,
    new_targets: &mut Vec<(UId, ObjTarget)>,
) {
    // basically: 1. if not in range then pathfind and move.
    // 2. If in range then kill.
    // 3. If target moves then keep shooting or rerun pathfinding.
    type ToQuery<'a> = (&'a WeaponComp, &'a PositionComp);

    let mut query = sim
        .ecs
        .query_one::<ToQuery>(*sim.res.id_map.get(uid).unwrap())
        .unwrap();
    let (wep_comp, pos_comp) = query.get().unwrap();

    // check if range is enough:
    if let Some(atk_trg_pos) = target_to_pos(sim, atk_trg) {
        // Update target either way:
        new_targets.push((*uid, *atk_trg));

        let dist_to_trg = pos_comp.get_pos().dist(&atk_trg_pos);

        match dist_to_trg <= wep_comp.get_max_range() {
            true => {
                let weapons_to_fire = wep_comp.get_weapons_in_range(&dist_to_trg);
                new_states.push((*uid, UnitState::FireWeapons(weapons_to_fire)));
            }
            false => {
                match knows_path_to_dest(&sim, sim.res.id_map.get(uid).unwrap(), &atk_trg_pos) {
                    true => {
                        new_states.push((*uid, UnitState::Move));
                    }
                    false => {
                        new_states.push((*uid, UnitState::PathfindAndMove));
                    }
                }
            }
        }
    }
}

fn moveto_order_behaviour(
    sim: &SimState,
    uid: &UId,
    dest: &Pos,
    new_states: &mut Vec<(UId, UnitState)>,
    new_targets: &mut Vec<(UId, ObjTarget)>,
) {
    type ToQuery<'a> = (&'a TargetComp, &'a UnitStateComp);

    let mut query = sim
        .ecs
        .query_one::<ToQuery>(*sim.res.id_map.get(uid).unwrap())
        .unwrap();
    let (unit_target, unit_state) = query.get().unwrap();

    // Update target:
    if unit_target.get_trg() != &ObjTarget::Position(*dest) {
        new_targets.push((*uid, ObjTarget::Position(*dest)));
    }

    match knows_path_to_dest(&sim, sim.res.id_map.get(uid).unwrap(), &dest) {
        true => {
            if unit_state.get_state() != &UnitState::Move {
                new_states.push((*uid, UnitState::Move));
            }
        }
        false => {
            if unit_state.get_state() != &UnitState::PathfindAndMove {
                new_states.push((*uid, UnitState::PathfindAndMove));
            }
        }
    }
}

fn ability_order_behaviour(
    sim: &SimState,
    uid: &UId,
    abil_id: &AbilityID,
    abil_trg: &ObjTarget,
    new_states: &mut Vec<(UId, UnitState)>,
    new_targets: &mut Vec<(UId, ObjTarget)>,
) {
    // what if unit has no id? Then set to Ability Unavailable and waste 1 frame :)
    let entity = sim.res.id_map.get(uid).unwrap();

    type ToQuery<'a> = (
        // &'a UnitStateComp,
        &'a ActiveAbilityComp,
        &'a PositionComp,
        // &'a TargetComp,
    );

    let mut query = sim.ecs.query_one::<ToQuery>(*entity).unwrap();

    // Don't run if entity doesn't have either of the components
    if let Some((abil_comp, pos_comp)) = query.get() {
        if let ObjTarget::None = abil_trg {
            new_states.push((*uid, UnitState::UseAbility(*abil_id)));
            // new_targets.push((*uid, ObjTarget::Entity(*uid)));
            new_targets.push((*uid, ObjTarget::None));

            return;
        }

        let abil_range = abil_comp.get_ability(*abil_id).get_range();

        // CHeck if target can be converted to position:
        let trg_pos = target_to_pos(sim, abil_trg);

        match trg_pos {
            None => {
                new_states.push((
                    *uid,
                    UnitState::UseAbilityFailed,
                ));
                new_targets.push((*uid, ObjTarget::None));
                return;
            }
            Some(trg_pos) => {
                // Is unit in range?
                if abil_range >= &pos_comp.get_pos().dist(&trg_pos) {
                    // Unit is in range, therefore set state to use ability
                    new_states.push((*uid, UnitState::UseAbility(*abil_id)));
                    // and target to same as in given order:
                    new_targets.push((*uid, *abil_trg));

                    return;
                } else {
                    match knows_path_to_dest(&sim, entity, &trg_pos) {
                        true => {
                            new_states.push((*uid, UnitState::Move));
                        }
                        false => {
                            new_states.push((*uid, UnitState::PathfindAndMove));
                        }
                    }
                    new_targets.push((*uid, ObjTarget::Position(trg_pos)));
                    return;
                }
            }
        }
    }
}

fn no_order_behaviour(
    sim: &SimState,
    uid: &UId,
    new_states: &mut Vec<(UId, UnitState)>,
    _new_targets: &mut Vec<(UId, ObjTarget)>,
) {
    // Behaviorus when unit has no specific orders
    type ToQuery<'a> = (&'a UnitStateComp,);

    let mut query = sim
        .ecs
        .query_one::<ToQuery>(*sim.res.id_map.get(uid).unwrap())
        .unwrap();
    let (unit_state,) = query.get().unwrap();

    if unit_state.get_state() != &UnitState::Idle {
        new_states.push((*uid, UnitState::Idle));
    }
}

fn knows_path_to_dest(sim: &SimState, entity_id: &Entity, dest: &Pos) -> bool {
    // Check how far away from first node in path

    type ToQuery<'a> = (&'a PositionComp, &'a PathComp);

    let mut query = sim.ecs.query_one::<ToQuery>(*entity_id).unwrap();
    if let Some((curr_pos, path)) = query.get() {
        if path.get_path().len() == 0 {
            return false;
        }

        // Unwraps won't panic, because previous IF checks for length:
        if (curr_pos.get_pos().dist(path.get_path().front().unwrap()) <= FixF::from_num(2))
            & (dest == path.get_path().back().unwrap())
        {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod order_and_state_tests {
    use crate::sim_rend_message::*;
    use crate::sim_components::sim_unit_base_components::IdComp;
    use crate::sim_components::sim_unit_base_components::NextPosComp;
    use crate::sim_components::weapon_comp::WeaponComp;

    use crate::sim_ecs::*;
    use crate::sim_gameloop::first_tick;
    use crate::sim_gameloop::run_single_tick;
    use crate::sim_systems::input_systems::receive_messages;
    use crate::sim_systems::input_systems::sys_input_to_order;

    use crate::common::*;
    use crate::sim_components::order_queue_comp::OrderQueueComp;
    use crate::sim_components::sim_unit_base_components::PathComp;
    use crate::sim_components::sim_unit_base_components::PositionComp;
    use crate::sim_components::targeting_comp::TargetComp;
    use crate::sim_components::unitstate_comp::UnitStateComp;
    use crate::sim_fix_math::*;
    use crate::sim_map::Map;

    fn print_components(sim: &mut SimState, e: &UId) {
        type ToQuery<'a> = (
            &'a IdComp,
            &'a UnitStateComp,
            &'a OrderQueueComp,
            &'a TargetComp,
            &'a PathComp,
            &'a PositionComp,
            &'a NextPosComp,
            &'a WeaponComp,
        );

        let mut query = sim
            .ecs
            .query_one::<ToQuery>(*sim.res.id_map.get(e).unwrap())
            .unwrap();
        let (id, state, queue, trg, path, pos, nextpos, weapon) = query.get().unwrap();
        println!("\n Tick: {:?} \n", sim.current_tick());
        println!("{:?} \n", id);
        println!("{:?} \n", state);
        println!("{:?} \n", queue);
        println!("{:?} \n", trg);
        println!("{:?} \n", path);
        println!("{:?} \n", pos);
        println!("{:?} \n", nextpos);
        println!("{:?} \n", weapon);
    }

    fn make_game_state() -> (SimState, RendMessenger) {
        let (sim_messenger, rend_messenger) = create_messenger();

        let map = Map::make_test_map();
        let mut sim = SimState::new(map, sim_messenger, 2, 10);

        //run first 2 ticks:
        first_tick(&mut sim);
        rend_messenger.rec();
        run_single_tick(&mut sim);
        sim.end_tick_debug();

        (sim, rend_messenger)
    }

    #[test]
    fn update_order_schedule() {
        // cargo test -- --nocapture update_order_schedule

        let (mut sim, rend_messenger) = make_game_state();

        let msg0 = RenderMessage::SpawnSmart(0, Pos::from_num(1, 1));
        rend_messenger.send(vec![msg0]);
        run_single_tick(&mut sim);

        // Print initial state
        print_components(&mut sim, &0);

        // Send order to move:
        let mut units: [Option<UId>; UNIT_GROUP_CAP] = [None; UNIT_GROUP_CAP];
        units[0] = Some(0);
        let msg = RenderMessage::InputOrder(0, units, UnitOrder::MoveTo(Pos::from_num(4, 1)));

        rend_messenger.send(vec![msg]);

        receive_messages(&mut sim);

        sys_input_to_order(&mut sim);

        print_components(&mut sim, &0);

        {
            let order = sim
                .ecs
                .get::<OrderQueueComp>(*sim.res.id_map.get(&0).unwrap())
                .unwrap();

            assert_eq!(
                *order.get_current_order(),
                UnitOrder::MoveTo(Pos::from_num(4, 1))
            );
        }

        let msg = RenderMessage::InputOrder(
            0,
            units,
            UnitOrder::Ability(0, ObjTarget::Position(Pos::from_num(10, 10))),
        );

        rend_messenger.send(vec![msg]);

        receive_messages(&mut sim);

        sys_input_to_order(&mut sim);

        {
            let order = sim
                .ecs
                .get::<OrderQueueComp>(*sim.res.id_map.get(&0).unwrap())
                .unwrap();

            assert_eq!(
                *order.get_current_order(),
                UnitOrder::Ability(0, ObjTarget::Position(Pos::from_num(10, 10)))
            );
        }
    }

    #[test]
    fn moveto_state() {
        // cargo test -- --nocapture moveto_state

        let (mut sim, rend_messenger) = make_game_state();

        let msg0 = RenderMessage::SpawnSmart(0, Pos::from_num(1, 1));
        rend_messenger.send(vec![msg0]);
        run_single_tick(&mut sim);
        sim.end_tick_debug();

        // Print initial state
        //print_components(&mut sim, &0);

        // Send order to move:
        let mut units: [Option<UId>; UNIT_GROUP_CAP] = [None; UNIT_GROUP_CAP];
        units[0] = Some(0);

        let msg = RenderMessage::InputOrder(0, units, UnitOrder::MoveTo(Pos::from_num(4, 1)));

        rend_messenger.send(vec![msg]);

        receive_messages(&mut sim);

        run_single_tick(&mut sim);
        sim.end_tick_debug();

        print_components(&mut sim, &0);
        {
            let state = sim
                .ecs
                .get::<UnitStateComp>(*sim.res.id_map.get(&0).unwrap())
                .unwrap();
            assert_eq!(*state.get_state(), UnitState::PathfindAndMove);
        }

        run_single_tick(&mut sim);
        sim.end_tick_debug();

        print_components(&mut sim, &0);

        {
            let state = sim
                .ecs
                .get::<UnitStateComp>(*sim.res.id_map.get(&0).unwrap())
                .unwrap();
            assert_eq!(*state.get_state(), UnitState::Move);
        }
    }

    #[test]
    fn use_abilities_states() {
        // cargo test -- --nocapture use_abilities_states

        let (mut sim, rend_messenger) = make_game_state();

        let msg0 = RenderMessage::SpawnSmart(0, Pos::from_num(1, 1));
        rend_messenger.send(vec![msg0]);
        run_single_tick(&mut sim);
        sim.end_tick_debug();

        // selection of units:
        let mut units: [Option<UId>; UNIT_GROUP_CAP] = [None; UNIT_GROUP_CAP];
        units[0] = Some(0);

        // Send message to use ability:
        let msg = RenderMessage::InputOrder(
            0,
            units,
            UnitOrder::Ability(0, ObjTarget::Position(Pos::from_num(3.9, 1.0))),
        );
        rend_messenger.send(vec![msg]);

        run_single_tick(&mut sim);
        sim.end_tick_debug();
        print_components(&mut sim, &0);

        {
            let state = sim
                .ecs
                .get::<UnitStateComp>(*sim.res.id_map.get(&0).unwrap())
                .unwrap();

            assert_eq!(*state.get_state(), UnitState::PathfindAndMove);
        }

        run_single_tick(&mut sim);
        sim.end_tick_debug();
        print_components(&mut sim, &0);

        {
            let state = sim
                .ecs
                .get::<UnitStateComp>(*sim.res.id_map.get(&0).unwrap())
                .unwrap();

            assert_eq!(*state.get_state(), UnitState::Move);
        }

        run_single_tick(&mut sim);
        sim.end_tick_debug();
        print_components(&mut sim, &0);

        {
            let state = sim
                .ecs
                .get::<UnitStateComp>(*sim.res.id_map.get(&0).unwrap())
                .unwrap();

            assert_eq!(*state.get_state(), UnitState::UseAbility(0));
        }

        run_single_tick(&mut sim);
        sim.end_tick_debug();
        print_components(&mut sim, &0);

        {
            let state = sim
                .ecs
                .get::<UnitStateComp>(*sim.res.id_map.get(&0).unwrap())
                .unwrap();

            assert_eq!(*state.get_state(), UnitState::Idle);
        }
    }

    #[test]
    fn force_attack_states() {
        // cargo test -- --nocapture force_attack_states

        let (mut sim, rend_messenger) = make_game_state();

        let msg0 = RenderMessage::SpawnSmart(0, Pos::from_num(1, 1));
        let msg1 = RenderMessage::SpawnSmart(1, Pos::from_num(1.0, 2.0));
        rend_messenger.send(vec![msg0, msg1]);
        run_single_tick(&mut sim);
        sim.end_tick_debug();

        // selection of units:
        let mut units: [Option<UId>; UNIT_GROUP_CAP] = [None; UNIT_GROUP_CAP];
        units[0] = Some(0);
        units[1] = Some(1);

        // Send message to use fire
        let msg = RenderMessage::InputOrder(
            0,
            units,
            UnitOrder::ForceAttack(ObjTarget::Position(Pos::from_num(2, 1))),
        );
        let msg2 =
            RenderMessage::InputOrder(1, units, UnitOrder::ForceAttack(ObjTarget::Entity(0)));
        rend_messenger.send(vec![msg, msg2]);

        run_single_tick(&mut sim);
        sim.end_tick_debug();
        // print_components(&mut sim, &0);
        print_components(&mut sim, &1);
        //println!("{:?}",rend_messenger.rec() );

        {
            let state = sim
                .ecs
                .get::<UnitStateComp>(*sim.res.id_map.get(&0).unwrap())
                .unwrap();

            match state.get_state() {
                UnitState::FireWeapons(..) => {}
                _ => {
                    panic!("wrong state");
                }
            }
        }

        {
            let state = sim
                .ecs
                .get::<UnitStateComp>(*sim.res.id_map.get(&1).unwrap())
                .unwrap();

            match state.get_state() {
                UnitState::FireWeapons(..) => {}
                _ => {
                    panic!("wrong state");
                }
            }
        }

        let msg = RenderMessage::InputOrder(
            1,
            units,
            UnitOrder::ForceAttack(ObjTarget::Position(Pos::from_num(7.0, 2.0))),
        );
        rend_messenger.send(vec![msg]);

        run_single_tick(&mut sim);
        sim.end_tick_debug();
        print_components(&mut sim, &1);

        {
            let state = sim
                .ecs
                .get::<UnitStateComp>(*sim.res.id_map.get(&1).unwrap())
                .unwrap();

            match state.get_state() {
                UnitState::PathfindAndMove => {}
                _ => {
                    panic!("wrong state");
                }
            }
        }
        run_single_tick(&mut sim);
        sim.end_tick_debug();

        print_components(&mut sim, &1);

        {
            let state = sim
                .ecs
                .get::<UnitStateComp>(*sim.res.id_map.get(&1).unwrap())
                .unwrap();

            match state.get_state() {
                UnitState::Move => {}
                _ => {
                    panic!("wrong state");
                }
            }
        }

        run_single_tick(&mut sim);
        sim.end_tick_debug();

        print_components(&mut sim, &1);

        {
            let state = sim
                .ecs
                .get::<UnitStateComp>(*sim.res.id_map.get(&1).unwrap())
                .unwrap();

            match state.get_state() {
                UnitState::FireWeapons(..) => {}
                _ => {
                    panic!("wrong state");
                }
            }
        }
    }
}
