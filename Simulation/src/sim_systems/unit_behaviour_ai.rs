use crate::common::*;
use crate::sim_components::order_queue_comp::OrderQueueComp;
use crate::sim_components::sim_unit_base_components::IdComp;
use crate::sim_components::sim_unit_base_components::PathComp;
use crate::sim_components::sim_unit_base_components::PositionComp;
use crate::sim_components::targeting_comp::TargetComp;
use crate::sim_components::unitstate_comp::UnitStateComp;
use crate::sim_fix_math::*;
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
            UnitOrder::Ability(..) => {
                // TBA
            }
            //_ => {}
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
    type ToQuery<'a> = (
        &'a IdComp,
        // &'a TargetComp,
        &'a OrderQueueComp,
        // &'a UnitStateComp,
    );

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
            UnitOrder::Ability(..) => {
                // TODO: here check range to target. If not in range then Move. If in range, then Use Ability.
            }
            //_ => {}
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
    use crate::messenger::*;
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
            &'a UnitStateComp,
            &'a OrderQueueComp,
            &'a TargetComp,
            &'a PathComp,
            &'a PositionComp,
        );

        let mut query = sim
            .ecs
            .query_one::<ToQuery>(*sim.res.id_map.get(e).unwrap())
            .unwrap();
        let (state, queue, trg, path, pos) = query.get().unwrap();
        println!("\n Tick: {:?} \n", sim.current_tick());
        println!("{:?} \n", state);
        println!("{:?} \n", queue);
        println!("{:?} \n", trg);
        println!("{:?} \n", path);
        println!("{:?} \n", pos);
    }

    #[test]
    fn update_order_schedule() {
        // cargo test -- --nocapture update_order_schedule

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

        let order = sim
            .ecs
            .get::<OrderQueueComp>(*sim.res.id_map.get(&0).unwrap())
            .unwrap();
        assert_eq!(
            *order.get_current_order(),
            UnitOrder::MoveTo(Pos::from_num(4, 1))
        );
    }

    #[test]
    fn moveto_state() {
        // cargo test -- --nocapture update_order_schedule

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

        // Print initial state
        print_components(&mut sim, &0);

        // Send order to move:
        let mut units: [Option<UId>; UNIT_GROUP_CAP] = [None; UNIT_GROUP_CAP];
        units[0] = Some(0);
        let msg = RenderMessage::InputOrder(0, units, UnitOrder::MoveTo(Pos::from_num(4, 1)));

        rend_messenger.send(vec![msg]);

        receive_messages(&mut sim);

        run_single_tick(&mut sim);

        print_components(&mut sim, &0);
        {
            let state = sim
                .ecs
                .get::<UnitStateComp>(*sim.res.id_map.get(&0).unwrap())
                .unwrap();
            assert_eq!(*state.get_state(), UnitState::PathfindAndMove);
        }

        run_single_tick(&mut sim);

        {
            let state = sim
                .ecs
                .get::<UnitStateComp>(*sim.res.id_map.get(&0).unwrap())
                .unwrap();
            assert_eq!(*state.get_state(), UnitState::Move);
        }
    }
}
