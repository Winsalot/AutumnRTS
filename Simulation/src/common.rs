/*
Here goes things that are used by at least 2 of:
Systems
Components
Messenger & RustBridge

Right now everything store in a single file. It will be reworked into its own module once it fills up <3
*/


use crate::sim_map::MapTile;
use crate::sim_fix_math::*;


/// Unit's id. u64 because that's what hec's Entity converts to/from.
pub type UId = u64;


pub type TickNum = u32; 

pub const N_ABILITY_CAP: usize = 3;


// Target. Either posiion or entity.
pub enum ObjTarget {
	Position(Pos),
	Entity(UId),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum EngineMessage {
    //ObjSpawn(IdComp, Pos), // Deprecated
    ObjPosColl(UId, Pos, FixF), // Message carrying position and collision radius info
    ObjMove(UId, Pos),
    ObjNextPos(UId, Pos),
    ObjDest(UId, Pos),
    ObjPathTmp(UId, [Pos; 20]), // To visualise pathfinding. Sends next 10 steps.
    MapTile(Pos, MapTile),
    StructurePosTmp(UId, Pos),
    Fps(u64, u64),
    //FpsFull(u64, u64),
    None, // this message sucks
    //Break,
    //Object(ObjectID, Pos, Pos), // TODO remove this variant
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RenderMessage {
    Destination(UId, Pos),
    Spawn(Pos),
    SpawnStructureTmp(Pos),
    UnitSpawnStructureTmp(UId,Pos),
    //None,
    Break,
}