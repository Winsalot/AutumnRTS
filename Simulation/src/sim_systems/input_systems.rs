use crate::sim_components::order_queue_comp::OrderQueueComp;
use crate::sim_systems::validate_order::is_valid;
use crate::common::SimStateChng::*;
use crate::common::SimMsg::StateChange;

use crate::sim_components::active_ability_comp::*;
use crate::sim_components::sim_unit_base_components::*;
use crate::sim_components::structure_comp::*;
use crate::sim_components::targeting_comp::*;
use crate::sim_components::unitstate_comp::*;
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
    sim.res.send_batch.push(SimMsg::SimInfo(SimStateInfo::Fps(fps.0, fps.1)));
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
        } )
        .next()
        .is_some();

    if do_break {
        sim.break_loop = true;
        return true;
    }

    false
}

pub fn plc_unit(
    sim: &mut SimState, 
    owner: PId, 
    pos: Pos, 
    speed: FixF, 
    coll_r: FixF
    ){

    let mut unit_builder = EntityBuilder::new();
    let player = sim.res.players.get(owner);

    if let Some(player) = player {
        unit_builder.add(TypeNameComp::new("placeholder"));
        unit_builder.add(PositionComp::new(pos));
        unit_builder.add(NextPosComp::new(pos));
        unit_builder.add(DestinationComp::new(pos));
        unit_builder.add(SpeedComponent::new(speed, 1));
        unit_builder.add(CollComp::new(coll_r));
        unit_builder.add(
            IdComp::new(
                &mut sim.res.id_counter, 
                player,
                )
            );
        unit_builder.add(PathComp::new());
        unit_builder.add(TargetComp::new(FixF::from_num(3)));
        unit_builder.add(ActiveAbilityComp::builder());
        unit_builder.add(UnitStateComp::new());
    
        let new_entity = sim.ecs.spawn(unit_builder.build());
    
    
        // let msg = EngineMessage::ObjPosColl(sim.res.id_counter - 1, pos, coll_r);
        // let msg = StateChange(ObjPosColl(sim.res.id_counter - 1, pos, coll_r));
        let msg = StateChange(
            ObjSpawn(
                sim.res.id_counter - 1,
                *player, 
                pos, coll_r
                )
            );
        sim.res.send_batch.push(msg);
    
        sim.res.id_map.insert(sim.res.id_counter - 1, new_entity);
    }
}

pub fn plc_building(sim: &mut SimState, owner: PId, pos: Pos) {
    let mut unit_builder = EntityBuilder::new();

    unit_builder.add(TypeNameComp::new("placeholder_building"));
        unit_builder.add(
        IdComp::new(
            &mut sim.res.id_counter, 
            sim.res.players.get(owner).unwrap()
            )
        );
    unit_builder.add(StructureComp::new(pos));

    let new_entity = sim.ecs.spawn(unit_builder.build());

    // let msg = EngineMessage::StructurePosTmp(sim.res.id_counter - 1, pos.round());
    let msg = StateChange(StructurePosTmp(sim.res.id_counter - 1, pos.round()));
    sim.res.send_batch.push(msg);

    sim.res.id_map.insert(sim.res.id_counter - 1, new_entity);
    sim.map.map_mem.add(vec![pos]);
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

                plc_unit(sim, player,  pos, speed, coll_rad_tmp);
                

            }
            _ => {}
        }
    }
}

// Takes inputs and turns them into UnitOrders.
// This system will grow and use multiple subsystems in the future.
// TODO: Add order validation in this part
pub fn sys_input_to_order(sim: &mut SimState){

    let inbox = &mut sim.res.inbox;

        let (input_orders, rest): (Vec<RenderMessage>, Vec<RenderMessage>) =
        inbox.iter().partition(|&msg|
            // This should use all messages that are orders to units.
            match msg {
                RenderMessage::InputOrder(..) => true,
                _ => false,
            }
        );

    *inbox = rest;

    for i in 0..input_orders.len() {
        match input_orders[i]{
            RenderMessage::InputOrder(player_id, unit_ids, UnitOrder::MoveTo(moveto_pos)) => {
                set_moveto_order(sim, &player_id, unit_ids, &moveto_pos);
            },
            RenderMessage::InputOrder(player_id, unit_ids,
                UnitOrder::Ability(abil_id, abil_target)
                ) => {
                set_ability_order(sim, &player_id, unit_ids, &abil_id, &abil_target);
            },
            _ => {},
        }
    }
}


// right now only set evey unit's State to that same pos
fn set_moveto_order(
    sim: &mut SimState, 
    player_id: &PId,
    unit_ids: [Option<UId>; UNIT_GROUP_CAP], 
    moveto_pos: &Pos
    ){
    for id in unit_ids.iter() {
        if let Some(id) = id {

            if !is_valid(sim, player_id, id){
                sim.res.send_batch.push(
                    SimMsg::Warn(
                        *sim.res.players.get(*player_id).unwrap(),
                        SimWarnMsg::UnitUnavailable)
                    );
                return;
            }

            if let Some(entity) = sim.res.id_map.get(&id){

                type ToQuery<'a> = (
                    &'a mut OrderQueueComp,
                    &'a mut DestinationComp,
                    );

                if let Ok(mut query) = sim.ecs.query_one::<ToQuery>(*entity){
                    if let Some((order_queue, dest)) = query.get(){

                        let mut moveto_pos_valid = moveto_pos.clone();
                        sim.map.constrain_pos(&mut moveto_pos_valid);
                        order_queue.set_single_order(UnitOrder::MoveTo(moveto_pos_valid));

                        dest.set_dest(moveto_pos_valid, sim.current_tick());
                        
                        let msg = StateChange(ObjDest(*id, moveto_pos_valid));
                        sim.res.send_batch.push(msg);

                    }
                }

                // if let Ok(mut state) = sim.ecs.get_mut::<OrderQueueComp>(*entity) {

                //     // Order should be validated here:
                //     let mut moveto_pos_valid = moveto_pos.clone();

                //     sim.map.constrain_pos(&mut moveto_pos_valid);


                //     state.set_single_order(UnitOrder::MoveTo(moveto_pos_valid));

                //     // // Probably need different message:
                //     // let msg = StateChange(ObjDest(id, pos));
                //     // sim.res.send_batch.push(msg);
                // }                        
            }    
        }
    }
}

// TODO: for groups of units only the closest unit to the target should use the ability.
fn set_ability_order(
    sim: &mut SimState, 
    player_id: &PId,
    unit_ids: [Option<UId>; UNIT_GROUP_CAP], 
    abil_id: &AbilityID,
    abil_trg: &ObjTarget,
    ){
    for id in unit_ids.iter() {
        if let Some(id) = id {
            if is_valid(sim, player_id, id){
                if let Some(entity) = sim.res.id_map.get(&id){
                    if let Ok(mut state) = sim.ecs.get_mut::<OrderQueueComp>(*entity) {

                        state.set_single_order(UnitOrder::Ability(*abil_id, *abil_trg));
                    }
                }
            }
        }
    }
}

// pub fn input_spawn_structure(sim: &mut SimState) {

//     let inbox = &mut sim.res.inbox;

//     let (spawn_msg, rest): (Vec<RenderMessage>, Vec<RenderMessage>) =
//         inbox.iter().partition(|&msg| match msg {
//             RenderMessage::SpawnStructureTmp(..) => true,
//             _ => false,
//         });

//     *inbox = rest;

//     for i in 0..spawn_msg.len() {
//         match spawn_msg[i] {
//             RenderMessage::SpawnStructureTmp(pos) => {

//                 if !sim.map.within(pos) {
//                     continue;
//                 }

//                 if sim.map.tile_from_pos(pos).blocks_path() {
//                     continue;
//                 }

//                 if sim.map.map_mem.get_blocked().contains(&pos.round()) {
//                     continue;
//                 }

//                 plc_building(sim, 0,  pos);
//             }
//             _ => {}
//         }
//     }
// }

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
