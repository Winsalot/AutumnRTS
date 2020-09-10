use crate::common::SimMsg::StateChange;
use crate::common::SimStateChng::*;
use crate::placeholder_entities::*;
use crate::sim_components::order_queue_comp::OrderQueueComp;
use crate::sim_systems::validate_order::is_valid;

use crate::sim_fix_math::Pos;
use hecs::*;

//use crate::sim_state_components::*;
use crate::common::*;

use crate::sim_ecs::*;
use crate::sim_fix_math::FixF;
//use hecs::*;

// this module contains ALL used systems (for now)

pub fn update_fps_info(sim: &mut SimState) {
    let fps = sim.res.fps_counter.get_fps();
    // sim.res.send_batch.push(EngineMessage::Fps(fps.0, fps.1));
    sim.res
        .send_batch
        .push(SimMsg::SimInfo(SimStateInfo::Fps(fps.0, fps.1)));
}

pub fn receive_messages(sim: &mut SimState) {
    let mut rend_msg = sim.messenger.rec();
    sim.res.inbox.append(&mut rend_msg);
}

pub fn input_break_check(sim: &mut SimState) -> bool {
    let inbox = &sim.res.inbox;

    let do_break = inbox
        .iter()
        // .filter(|x| **x == RenderMessage::Break)
        .filter(|x| match **x {
            RenderMessage::Break => true,
            _ => false,
        })
        .next()
        .is_some();

    if do_break {
        sim.break_loop = true;
        return true;
    }

    false
}

pub fn input_spawn_unit(sim: &mut SimState) {
    // Reads messages, removes spawn messages from inbox. Spawns units and egnerates messages

    let inbox = &mut sim.res.inbox;

    let (spawn_msg, rest): (Vec<RenderMessage>, Vec<RenderMessage>) =
        inbox.iter().partition(|&msg| match msg {
            RenderMessage::Spawn(..) => true,
            _ => false,
        });

    *inbox = rest;

    for i in 0..spawn_msg.len() {
        match spawn_msg[i] {
            RenderMessage::Spawn(player, pos) => {
                // Prevent from spawning outside map:
                if !sim.map.within(pos) {
                    continue;
                }

                if sim.map.tile_from_pos(pos).blocks_path() {
                    continue;
                }

                //TODO: these values should be taken from data files.
                let coll_rad_tmp = FixF::from_num(0.5);
                let speed = FixF::from_num(0.5);

                plc_unit(sim, player, pos, speed, coll_rad_tmp);
            }
            _ => {}
        }
    }
}

pub fn input_spawn_smart_unit(sim: &mut SimState) {
    // Reads messages, removes spawn messages from inbox. Spawns units and egnerates messages

    let inbox = &mut sim.res.inbox;

    let (spawn_msg, rest): (Vec<RenderMessage>, Vec<RenderMessage>) =
        inbox.iter().partition(|&msg| match msg {
            RenderMessage::SpawnSmart(..) => true,
            _ => false,
        });

    *inbox = rest;

    for i in 0..spawn_msg.len() {
        match spawn_msg[i] {
            RenderMessage::SpawnSmart(player, pos) => {
                // Prevent from spawning outside map:
                if !sim.map.within(pos) {
                    continue;
                }

                if sim.map.tile_from_pos(pos).blocks_path() {
                    continue;
                }

                //TODO: these values should be taken from data files.
                let coll_rad_tmp = FixF::from_num(0.5);
                let speed = FixF::from_num(0.5);

                plc_smart_unit(sim, player, pos, speed, coll_rad_tmp);
            }
            _ => {}
        }
    }
}

// Takes inputs and turns them into UnitOrders.
// This system will grow and use multiple subsystems in the future.
// TODO: Add order validation in this part
pub fn sys_input_to_order(sim: &mut SimState) {
    let inbox = &mut sim.res.inbox;

    let (input_orders, rest): (Vec<RenderMessage>, Vec<RenderMessage>) =
        inbox.iter().partition(|&msg|
            // This should use all messages that are orders to units.
            match msg {
                RenderMessage::InputOrder(..) => true,
                _ => false,
            });

    *inbox = rest;

    for i in 0..input_orders.len() {
        match input_orders[i] {
            RenderMessage::InputOrder(player_id, unit_ids, UnitOrder::MoveTo(moveto_pos)) => {
                set_moveto_order(sim, &player_id, unit_ids, &moveto_pos);
            }
            RenderMessage::InputOrder(
                player_id,
                unit_ids,
                UnitOrder::Ability(abil_id, abil_target),
            ) => {
                set_ability_order(sim, &player_id, unit_ids, &abil_id, &abil_target);
            }
            RenderMessage::InputOrder(player_id, unit_ids, UnitOrder::ForceAttack(target)) => {
                set_forceattack_order(sim, &player_id, &unit_ids, &target);
            }
            _ => {}
        }
    }
}

fn set_forceattack_order(
    sim: &mut SimState,
    player_id: &PId,
    unit_ids: &[Option<UId>; UNIT_GROUP_CAP],
    trg: &ObjTarget,
) {
    for id in unit_ids.iter() {
        if let Some(id) = id {
            if !is_valid(sim, player_id, id) {
                sim.res.send_batch.push(SimMsg::Warn(
                    *sim.res.players.get(*player_id).unwrap(),
                    SimWarnMsg::UnitUnavailable,
                ));
                continue;
            }
            if let Some(entity) = sim.res.id_map.get(id) {
                type ToQuery<'a> = (&'a mut OrderQueueComp,);
                if let Ok(mut query) = sim.ecs.query_one::<ToQuery>(*entity) {
                    if let Some((order_queue,)) = query.get() {
                        order_queue.set_single_order(UnitOrder::ForceAttack(*trg));

                        // TODO: make attack order message
                    }
                }
            }
        }
    }
}

// right now only set evey unit's State to that same pos
fn set_moveto_order(
    sim: &mut SimState,
    player_id: &PId,
    unit_ids: [Option<UId>; UNIT_GROUP_CAP],
    moveto_pos: &Pos,
) {
    for id in unit_ids.iter() {
        if let Some(id) = id {
            if !is_valid(sim, player_id, id) {
                sim.res.send_batch.push(SimMsg::Warn(
                    *sim.res.players.get(*player_id).unwrap(),
                    SimWarnMsg::UnitUnavailable,
                ));
                continue;
            }

            if let Some(entity) = sim.res.id_map.get(&id) {
                type ToQuery<'a> = (
                    &'a mut OrderQueueComp,
                    // &'a mut DestinationComp,
                );

                if let Ok(mut query) = sim.ecs.query_one::<ToQuery>(*entity) {
                    if let Some((order_queue,)) = query.get() {
                        let mut moveto_pos_valid = moveto_pos.clone();
                        sim.map.constrain_pos(&mut moveto_pos_valid);
                        order_queue.set_single_order(UnitOrder::MoveTo(moveto_pos_valid));

                        // dest.set_dest(moveto_pos_valid, sim.current_tick());

                        let msg = StateChange(ObjDest(*id, moveto_pos_valid));
                        sim.res.send_batch.push(msg);
                    }
                }
            }
        }
    }
}

// TODO: for groups of units only the closest unit to the target should use the ability. Probably best to place this under behaviour AI module, instead of input systems.
fn set_ability_order(
    sim: &mut SimState,
    player_id: &PId,
    unit_ids: [Option<UId>; UNIT_GROUP_CAP],
    abil_id: &AbilityID,
    abil_trg: &ObjTarget,
) {
    for id in unit_ids.iter() {
        if let Some(id) = id {
            if is_valid(sim, player_id, id) {
                if let Some(entity) = sim.res.id_map.get(&id) {
                    if let Ok(mut state) = sim.ecs.get_mut::<OrderQueueComp>(*entity) {
                        state.set_single_order(UnitOrder::Ability(*abil_id, *abil_trg));
                    }
                }
            }
        }
    }
}

pub fn clear_inbox(sim: &mut SimState) -> Option<Vec<RenderMessage>> {
    let mut ret: Option<Vec<RenderMessage>> = None;
    if sim.res.inbox.len() != 0 {
        ret = Some(sim.res.inbox.clone());
        sim.res.inbox = vec![];
    }
    ret
}

pub fn sys_init_send_map(sim: &mut SimState) {
    let mut msg = sim.map.to_message();
    sim.res.send_batch.append(&mut msg);
}

pub fn send_messages(sim: &mut SimState) {
    sim.messenger.send(sim.res.send_batch.clone());
    sim.res.send_batch = vec![];
}
