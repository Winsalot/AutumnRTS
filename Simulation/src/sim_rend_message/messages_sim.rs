use crate::sim_map::MapTile;
use crate::common::*;
use crate::sim_fix_math::*;

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
    ObjTargetNone(UId),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SimStateInfo {
    Fps(u64, u64),
    GameTick(TickNum),
    MapTile(Pos, MapTile),
}