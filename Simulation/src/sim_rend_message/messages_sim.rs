use crate::common::*;
use crate::sim_fix_math::*;
use crate::sim_map::MapTile;

/// New engine messages
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SimMsg {
    Warn(PlayerId, SimWarnMsg), // PlayerId because player shouldn't hear bot's warnings.
    StateChange(SimStateChng),
    SimInfo(SimStateInfo),
}

/// Simulation warning messages variants
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SimWarnMsg {
    _AbilTrgInvalid,  // Target invalid
    _AbilUnavailable, // on cooldown
    AbilOnCD,
    UnitUnavailable, // Invalid unit. Maybe already dead.
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SimStateChng {
    ObjSpawn(UId, PlayerId, Pos, FixF), // obj spawn info
    // ObjPosColl(UId, Pos, FixF),
    ObjMove(UId, Pos),
    ObjNextPos(UId, Pos),
    ObjDest(UId, Pos),
    ObjPathTmp(UId, [Pos; 20]),
    StructurePosTmp(UId, Pos),
    ObjTargetPos(UId, Pos),
    ObjTarget(UId, ObjTarget),
    ObjTargetNone(UId),
    UnitNew {
        uid: UId,
        owner: PlayerId,
        pos: Pos,
        speed: FixF,
        coll_r: FixF,
    },
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SimStateInfo {
    Fps(f64, f64),
    GameTick(TickNum),
    MapTile(Pos, MapTile),
}
