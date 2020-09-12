use crate::sim_rend_message::*;
// use crate::common::SimStateChng::*;
// use crate::common::SimStateInfo::*;
use crate::common::*;

use crate::sim_fix_math::Pos;

//use crate::sim_unit_base_components::IdComp;
use gdnative::*;
use itertools::Itertools;

// this small module offers functions to process Simulation messages
// and prepaes them for godot

pub fn pos_to_vector2(pos: Pos) -> Vector2 {
    let (x, y): (f32, f32) = (pos.x.to_num::<f32>(), pos.y.to_num::<f32>());
    Vector2::new(x, y)
}

pub fn _vector2_to_pos(vec: Vector2) -> Pos {
    Pos::from_num(vec.x, vec.y)
}

//pub fn inbox_drain_spawn(inbox: &mut Vec<EngineMessage>) -> Vec<(UId, f32, f32, f32)> {
pub fn inbox_drain_spawn(inbox: &mut Vec<SimMsg>) -> Vec<(UId, PId, String, f32, f32, f32)> {
    // let (target, rest): (Vec<EngineMessage>, Vec<EngineMessage>) =
    let (target, rest): (Vec<SimMsg>, Vec<SimMsg>) = inbox.iter().partition(|&msg| match msg {
        // EngineMessage::ObjPosColl(..) => true,
        // SimMsg::StateChange(ObjPosColl(..)) => true,
        SimMsg::StateChange(SimStateChng::ObjSpawn(..)) => true,
        _ => false,
    });

    *inbox = rest;

    // turn messages into tuples:
    let mut ret: Vec<(UId, PId, String, f32, f32, f32)> = vec![];
    for i in 0..target.len() {
        // if let EngineMessage::ObjPosColl(id, pos, radius) = target[i] {
        // if let  SimMsg::StateChange(ObjPosColl(id, pos, radius)) = target[i] {
        if let SimMsg::StateChange(SimStateChng::ObjSpawn(id, player, pos, radius)) = target[i] {
            ret.push((
                //id.get().clone(),
                id,
                player.get_id(),
                player.get_team().to_str(),
                pos.x.to_num::<f32>(),
                pos.y.to_num::<f32>(),
                radius.to_num::<f32>(),
            ));
        }
    }

    return ret;
}

// pub fn inbox_drain_spawn_structure(inbox: &mut Vec<EngineMessage>) -> Vec<(UId, f32, f32)> {
pub fn inbox_drain_spawn_structure(inbox: &mut Vec<SimMsg>) -> Vec<(UId, f32, f32)> {
    // let (target, rest): (Vec<EngineMessage>, Vec<EngineMessage>) =
    let (target, rest): (Vec<SimMsg>, Vec<SimMsg>) = inbox.iter().partition(|&msg| match msg {
        // EngineMessage::StructurePosTmp(..) => true,
        SimMsg::StateChange(SimStateChng::StructurePosTmp(..)) => true,
        _ => false,
    });

    *inbox = rest;

    // turn messages into tuples:
    let mut ret: Vec<(UId, f32, f32)> = vec![];
    for i in 0..target.len() {
        // if let EngineMessage::StructurePosTmp(id, pos) = target[i] {
        if let SimMsg::StateChange(SimStateChng::StructurePosTmp(id, pos)) = target[i] {
            ret.push((id, pos.x.to_num::<f32>(), pos.y.to_num::<f32>()));
        }
    }

    return ret;
}

// pub fn inbox_drain_move(inbox: &mut Vec<EngineMessage>) -> Vec<(UId, f32, f32)> {
pub fn inbox_drain_move(inbox: &mut Vec<SimMsg>) -> Vec<(UId, f32, f32)> {
    // let (target, rest): (Vec<EngineMessage>, Vec<EngineMessage>) =
    let (target, rest): (Vec<SimMsg>, Vec<SimMsg>) = inbox.iter().partition(|&msg| match msg {
        // EngineMessage::ObjMove(..) => true,
        SimMsg::StateChange(SimStateChng::ObjMove(..)) => true,
        _ => false,
    });

    *inbox = rest;

    // turn messages into tuples:
    let mut ret: Vec<(UId, f32, f32)> = vec![];
    for i in 0..target.len() {
        // if let EngineMessage::ObjMove(id, pos) = target[i] {
        if let SimMsg::StateChange(SimStateChng::ObjMove(id, pos)) = target[i] {
            ret.push((
                //id.get().clone(),
                id,
                pos.x.to_num::<f32>(),
                pos.y.to_num::<f32>(),
            ));
        }
    }

    return ret;
}

// pub fn inbox_drain_next_pos(inbox: &mut Vec<EngineMessage>) -> Vec<(UId, f32, f32)> {
pub fn inbox_drain_next_pos(inbox: &mut Vec<SimMsg>) -> Vec<(UId, f32, f32)> {
    // let (target, rest): (Vec<EngineMessage>, Vec<EngineMessage>) =
    let (target, rest): (Vec<SimMsg>, Vec<SimMsg>) = inbox.iter().partition(|&msg| match msg {
        // EngineMessage::ObjNextPos(..) => true,
        SimMsg::StateChange(SimStateChng::ObjNextPos(..)) => true,
        _ => false,
    });

    *inbox = rest;

    // turn messages into tuples:
    let mut ret: Vec<(UId, f32, f32)> = vec![];
    for i in 0..target.len() {
        // if let EngineMessage::ObjNextPos(id, pos) = target[i] {
        if let SimMsg::StateChange(SimStateChng::ObjNextPos(id, pos)) = target[i] {
            ret.push((
                //id.get().clone(),
                id,
                pos.x.to_num::<f32>(),
                pos.y.to_num::<f32>(),
            ));
        }
    }

    return ret;
}

// pub fn inbox_drain_fps(inbox: &mut Vec<EngineMessage>) -> Vec<(u64, u64)> {
pub fn inbox_drain_fps(inbox: &mut Vec<SimMsg>) -> Vec<(u64, u64)> {
    // let (target, rest): (Vec<EngineMessage>, Vec<EngineMessage>) =
    let (target, rest): (Vec<SimMsg>, Vec<SimMsg>) = inbox.iter().partition(|&msg| match msg {
        // EngineMessage::Fps(..) => true,
        SimMsg::SimInfo(SimStateInfo::Fps(..)) => true,
        _ => false,
    });

    *inbox = rest;

    // turn messages into tuples:
    let mut ret: Vec<(u64, u64)> = vec![];
    for i in 0..target.len() {
        // if let EngineMessage::Fps(fps, fps_r) = target[i] {
        if let SimMsg::SimInfo(SimStateInfo::Fps(fps, fps_r)) = target[i] {
            ret.push((fps, fps_r));
        }
    }

    return ret;
}

// pub fn inbox_drain_dest(inbox: &mut Vec<EngineMessage>) -> Vec<(UId, f32, f32)> {
pub fn inbox_drain_dest(inbox: &mut Vec<SimMsg>) -> Vec<(UId, f32, f32)> {
    // let (target, rest): (Vec<EngineMessage>, Vec<EngineMessage>) =
    let (target, rest): (Vec<SimMsg>, Vec<SimMsg>) = inbox.iter().partition(|&msg| match msg {
        // EngineMessage::ObjDest(..) => true,
        SimMsg::StateChange(SimStateChng::ObjDest(..)) => true,
        _ => false,
    });

    *inbox = rest;

    // turn messages into tuples:
    let mut ret: Vec<(UId, f32, f32)> = vec![];
    for i in 0..target.len() {
        // if let EngineMessage::ObjDest(id, pos) = target[i] {
        if let SimMsg::StateChange(SimStateChng::ObjDest(id, pos)) = target[i] {
            ret.push((
                //id.get().clone(),
                id,
                pos.x.to_num::<f32>(),
                pos.y.to_num::<f32>(),
            ));
        }
    }

    return ret;
}

// pub fn inbox_drain_map_layout(inbox: &mut Vec<EngineMessage>) -> Vec<(f32, f32, bool, i32)> {
pub fn inbox_drain_map_layout(inbox: &mut Vec<SimMsg>) -> Vec<(f32, f32, bool, i32)> {
    // let (target, rest): (Vec<EngineMessage>, Vec<EngineMessage>) =
    let (target, rest): (Vec<SimMsg>, Vec<SimMsg>) = inbox.iter().partition(|&msg| match msg {
        // EngineMessage::MapTile(..) => true,
        SimMsg::SimInfo(SimStateInfo::MapTile(..)) => true,
        _ => false,
    });

    *inbox = rest;

    // turn messages into tuples:
    let mut ret: Vec<(f32, f32, bool, i32)> = vec![];
    for i in 0..target.len() {
        // if let EngineMessage::MapTile(pos, tile) = target[i] {
        if let SimMsg::SimInfo(SimStateInfo::MapTile(pos, tile)) = target[i] {
            ret.push((
                pos.x.to_num::<f32>(),
                pos.y.to_num::<f32>(),
                tile.blocks_path(),
                tile.z_level(),
            ));
        }
    }

    return ret;
}

// pub fn inbox_drain_pathfinding_tmp(inbox: &mut Vec<EngineMessage>) -> Vec<(UId, Vec<Vector2>)> {
pub fn inbox_drain_pathfinding_tmp(inbox: &mut Vec<SimMsg>) -> Vec<(UId, Vec<Vector2>)> {
    // let (target, rest): (Vec<EngineMessage>, Vec<EngineMessage>) =
    let (target, rest): (Vec<SimMsg>, Vec<SimMsg>) = inbox.iter().partition(|&msg| match msg {
        // EngineMessage::ObjPathTmp(..) => true,
        SimMsg::StateChange(SimStateChng::ObjPathTmp(..)) => true,
        _ => false,
    });

    *inbox = rest;

    let mut ret: Vec<(UId, Vec<Vector2>)> = vec![];

    for i in 0..target.len() {
        // if let EngineMessage::ObjPathTmp(id, positions) = target[i] {
        if let SimMsg::StateChange(SimStateChng::ObjPathTmp(id, positions)) = target[i] {
            ret.push((
                //id.get().clone(),
                id,
                positions
                    .iter()
                    .map(|pos| pos_to_vector2(*pos))
                    .dedup()
                    .collect(),
            ));
        }
    }

    return ret;
}

pub fn inbox_drain_targeting(inbox: &mut Vec<SimMsg>) -> Vec<(UId, Vector2)> {
    // No target will return vector of (-1, -1). Since map coords are >=0 this is ok.
    let (target, rest): (Vec<SimMsg>, Vec<SimMsg>) = inbox.iter().partition(|&msg| match msg {
        SimMsg::StateChange(SimStateChng::ObjTargetNone(..)) => true,
        SimMsg::StateChange(SimStateChng::ObjTargetPos(..)) => true,
        _ => false,
    });

    *inbox = rest;

    let mut ret: Vec<(UId, Vector2)> = vec![];

    for i in 0..target.len() {
        match target[i] {
            SimMsg::StateChange(SimStateChng::ObjTargetNone(id)) => {
                ret.push((id, Vector2::new(-1.0, -1.0)));
            }
            SimMsg::StateChange(SimStateChng::ObjTargetPos(id, position)) => {
                ret.push((id, pos_to_vector2(position)));
            }
            _ => {}
        }
    }
    return ret;
}
