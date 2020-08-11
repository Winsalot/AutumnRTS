use crate::sim_components::sim_unit_base_components::*;
use crate::common::*;
use crate::sim_ecs::*;
use crate::sim_fix_math::*;
use hecs::*;


pub fn sys_input_dest(sim: &mut SimState) {
    let inbox = &mut sim.res.inbox;

    let (dest_msg, rest): (Vec<RenderMessage>, Vec<RenderMessage>) =
        inbox.clone().iter().partition(|&msg| match msg {
            RenderMessage::Destination(..) => true,
            _ => false,
        });

    *inbox = rest;

    for i in 0..dest_msg.len() {
        match dest_msg[i] {
            RenderMessage::Destination(id, mut pos) => {

                let entity = sim.res.id_map.get(&id);

                if entity.is_none(){
                    // This makes sure that .unwrap() won't panic
                    continue;
                }

                let dest_comp = sim.ecs.get_mut::<DestinationComp>(*entity.unwrap());
                if let Ok(mut dest_comp) = dest_comp {
                    // Prevent destination from happening outside mapo
                    sim.map.constrain_pos(&mut pos);

                    dest_comp.set_dest(pos, sim.current_tick());
                    let msg = EngineMessage::ObjDest(id, pos);
                    sim.res.send_batch.push(msg);

                }
            }
            _ => {}
        }
    }
}

// update next position, that moves unit closer to destination
pub fn sys_set_next_pos(sim: &mut SimState) {
    type ToQuery<'a> = (
        &'a IdComp,
        &'a PositionComp,
        &'a mut NextPosComp,
        &'a mut PathComp,
        &'a SpeedComponent,
    );

    let ecs = &mut sim.ecs;

    'query_loop: for (_, (id, pos, next_pos, path, speed)) in &mut ecs.query::<ToQuery>() {

        let path_next_pos = path.get_next_pos(pos.get_pos());

        if let Some(move_to) = path_next_pos {
            let distance = pos.get_pos().dist(move_to);

            // This can happen because fixed point math is used.
            if distance == 0 {
                next_pos.set_pos(*move_to);
                continue 'query_loop;
            }

            let dx = (*pos.get_pos() - *move_to) / distance;
            let n_next_pos = *pos.get_pos() - dx * (*speed.get_speed()).min(distance);

            next_pos.set_pos(n_next_pos);
            let msg = EngineMessage::ObjNextPos(*id.get(), n_next_pos);
            sim.res.send_batch.push(msg)
        }
    }
}

pub fn sys_collision_pred(sim: &mut SimState) {
    // brute force collision detection. Should probably optimize this sometime in the future
    // basically go over every entity with collision and position and make sure it doesnt collide with anything on next move
    type ToQuery0<'a> = (&'a NextPosComp, &'a CollComp);
    type ToQuery1<'a> = (&'a PositionComp, &'a CollComp);

    let ecs = &mut sim.ecs;

    let mut non_move_entities: Vec<Entity> = vec![];

    for (id0, (next_pos0, coll0)) in &mut ecs.query::<ToQuery0>() {
        'child_loop: for (id1, (next_pos1, coll1)) in &mut ecs.query::<ToQuery1>() {
            if id1 == id0 {
                continue 'child_loop;
            }
            // does id0 collide with anything?
            let dist = Pos::dist(next_pos0.get_pos(), next_pos1.get_pos());
            if dist <= (coll0.get_r() + coll1.get_r()) {
                //next_pos0.set_pos(*pos0.get_pos());
                non_move_entities.push(id0.clone());
            }
        }
    }

    type ToQuery2<'a> = (&'a PositionComp, &'a mut NextPosComp);

    for i in 0..non_move_entities.len() {
        let entity = non_move_entities[i];

        let mut query = ecs.query_one::<ToQuery2>(entity).unwrap();
        let (pos, next_pos) = query.get().unwrap();

        next_pos.set_pos(*pos.get_pos());
    }
}

// Move to valid next position
pub fn sys_set_pos(sim: &mut SimState) {
    // Updates unit positions to next position
    // Also generates engine messages
    type ToQuery<'a> = (&'a IdComp, &'a mut PositionComp, &'a NextPosComp);

    let ecs = &mut sim.ecs;

    'query_loop: for (_, (id, pos, next_pos)) in &mut ecs.query::<ToQuery>() {
        if next_pos.get_pos() == pos.get_pos() {
            continue 'query_loop;
        }

        let msg: EngineMessage;

        pos.set_pos(*next_pos.get_pos());
        msg = EngineMessage::ObjMove(*id.get(), *pos.get_pos());

        sim.res.send_batch.push(msg);
    }
}
