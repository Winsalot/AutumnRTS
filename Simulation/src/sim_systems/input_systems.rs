use crate::common::SimStateChng::StructurePosTmp;
use crate::common::SimMsg::StateChange;
use crate::common::SimStateChng::ObjPosColl;
use crate::sim_components::active_ability_comp::*;
use crate::sim_components::sim_unit_base_components::*;
use crate::sim_components::structure_comp::*;
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
        .filter(|x| **x == RenderMessage::Break)
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

    unit_builder.add(TypeNameComp::new("placeholder"));
    unit_builder.add(PositionComp::new(pos));
    unit_builder.add(NextPosComp::new(pos));
    unit_builder.add(DestinationComp::new(pos));
    unit_builder.add(SpeedComponent::new(speed));
    unit_builder.add(CollComp::new(coll_r));
    unit_builder.add(
        IdComp::new(
            &mut sim.res.id_counter, 
            sim.res.players.get(owner).unwrap()
            )
        );
    unit_builder.add(PathComp::new());
    unit_builder.add(ActiveAbilityComp::builder());

    let new_entity = sim.ecs.spawn(unit_builder.build());


    // let msg = EngineMessage::ObjPosColl(sim.res.id_counter - 1, pos, coll_r);
    let msg = StateChange(ObjPosColl(sim.res.id_counter - 1, pos, coll_r));
    sim.res.send_batch.push(msg);

    sim.res.id_map.insert(sim.res.id_counter - 1, new_entity);
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
        inbox.clone().iter().partition(|&msg| match msg {
            RenderMessage::Spawn(..) => true,
            _ => false,
        });

    *inbox = rest;

    for i in 0..spawn_msg.len() {
        match spawn_msg[i] {
            RenderMessage::Spawn(pos) => {
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

                plc_unit(sim, 0,  pos, speed, coll_rad_tmp);
                

            }
            _ => {}
        }
    }
}

pub fn input_spawn_structure(sim: &mut SimState) {

    let inbox = &mut sim.res.inbox;

    let (spawn_msg, rest): (Vec<RenderMessage>, Vec<RenderMessage>) =
        inbox.iter().partition(|&msg| match msg {
            RenderMessage::SpawnStructureTmp(..) => true,
            _ => false,
        });

    *inbox = rest;

    for i in 0..spawn_msg.len() {
        match spawn_msg[i] {
            RenderMessage::SpawnStructureTmp(pos) => {

                if !sim.map.within(pos) {
                    continue;
                }

                if sim.map.tile_from_pos(pos).blocks_path() {
                    continue;
                }

                if sim.map.map_mem.get_blocked().contains(&pos.round()) {
                    continue;
                }

                plc_building(sim, 0,  pos);
            }
            _ => {}
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
