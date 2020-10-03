use crate::common::*;
use crate::sim_fix_math::*;

// #[derive(Debug, PartialEq, Clone, Copy)]
#[derive(Debug, Clone, Copy)]
pub enum RenderMessage {
    Destination(UId, PId, Pos),
    Spawn(PId, Pos),
    SpawnSmart(PId, Pos),
    //SpawnStructureTmp(Pos, PId),
    UseAbility(UId, PId, AbilityID, ObjTarget),
    InputOrder(PId, [Option<UId>; UNIT_GROUP_CAP], UnitOrder), //
    Break,
}
