use crate::sim_components::active_ability_comp::*;
use crate::sim_components::sim_unit_base_components::*;
use crate::sim_components::structure_comp::*;
use crate::sim_fix_math::Pos;
use hecs::*;

//use crate::sim_state_components::*;
use crate::common::*;
use crate::messenger::*;
use crate::sim_ecs::*;
use crate::sim_fix_math::FixF;
//use hecs::*;

// this module contains ALL used systems (for now)

pub fn update_fps_info(sim: &mut SimState) {
    let fps = sim.fps_counter.get_fps();
    sim.send_batch.push(EngineMessage::Fps(fps.0, fps.1));
}

pub fn receive_messages(sim: &mut SimState) {
    let mut rend_msg = sim.messenger.rec();
    sim.inbox.append(&mut rend_msg);
}

pub fn input_break_check(sim: &mut SimState) -> bool {
    let inbox = &sim.inbox;

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

pub fn plc_unit(pos: Pos, speed: FixF, id_counter: &mut UId) -> EntityBuilder {
    let mut unit_builder = EntityBuilder::new();

    unit_builder.add(TypeNameComp::new("placeholder"));
    unit_builder.add(PositionComp::new(pos));
    unit_builder.add(NextPosComp::new(pos));
    unit_builder.add(DestinationComp::new(pos));
    unit_builder.add(SpeedComponent::new(speed));
    unit_builder.add(CollComp::new(FixF::from_num(0.5)));
    unit_builder.add(IdComp::new(id_counter));
    unit_builder.add(PathComp::new());
    unit_builder.add(ActiveAbilityComp::builder());

    unit_builder
}

pub fn plc_building(pos: Pos, id_counter: &mut UId) -> EntityBuilder {
    let mut unit_builder = EntityBuilder::new();

    unit_builder.add(TypeNameComp::new("placeholder_building"));
    unit_builder.add(IdComp::new(id_counter));
    unit_builder.add(StructureComp::new(pos));

    unit_builder
}

pub fn input_spawn_unit(sim: &mut SimState) {
    // Reads messages, removes spawn messages from inbox. Spawns units and egnerates messages

    let inbox = &mut sim.inbox;

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

                //TODO: coll_rad_tmp should not be hardcoded
                let coll_rad_tmp = FixF::from_num(0.5);

                let mut new_unit = plc_unit(pos, coll_rad_tmp, &mut sim.id_counter);
                let e = sim.ecs.spawn(new_unit.build());

                let id = sim.ecs.get::<IdComp>(e).unwrap();

                let msg = EngineMessage::ObjPosColl(*id.get(), pos, coll_rad_tmp);
                sim.send_batch.push(msg);
            }
            _ => {}
        }
    }
}

pub fn input_spawn_structure(sim: &mut SimState) {
    // Reads messages, removes spawn messages from inbox. Spawns units and egnerates messages

    let inbox = &mut sim.inbox;

    let (spawn_msg, rest): (Vec<RenderMessage>, Vec<RenderMessage>) =
        inbox.iter().partition(|&msg| match msg {
            RenderMessage::SpawnStructureTmp(..) => true,
            _ => false,
        });

    *inbox = rest;

    for i in 0..spawn_msg.len() {
        match spawn_msg[i] {
            RenderMessage::SpawnStructureTmp(pos) => {
                // Prevent from spawning outside map:
                if !sim.map.within(pos) {
                    continue;
                }

                if sim.map.tile_from_pos(pos).blocks_path() {
                    continue;
                }

                if sim.map.map_mem.get_blocked().contains(&pos.round()) {
                    continue;
                }

                let mut new_structure = plc_building(pos, &mut sim.id_counter);
                let e = sim.ecs.spawn(new_structure.build());

                sim.map.add_structure(vec![pos]);

                let id = sim.ecs.get::<IdComp>(e).unwrap();

                let msg = EngineMessage::StructurePosTmp(*id.get(), pos.round());
                sim.send_batch.push(msg);
            }
            _ => {}
        }
    }
}

pub fn clear_inbox(sim: &mut SimState) -> Option<Vec<RenderMessage>> {
    // clears unread rendermessages.
    // Sends returns unused messages
    let mut ret: Option<Vec<RenderMessage>> = None;
    if sim.inbox.len() != 0 {
        ret = Some(sim.inbox.clone());
        sim.inbox = vec![];
    }
    ret
}

pub fn sys_init_send_map(sim: &mut SimState) {
    let mut msg = sim.map.to_message();
    sim.send_batch.append(&mut msg);
}

pub fn send_messages(sim: &mut SimState) {
    sim.messenger.send(sim.send_batch.clone());
    sim.send_batch = vec![];
}
