/*
Here goes things that are used by at least 2 of:
Systems
Components
Messenger & RustBridge

Right now everything store in a single file. It will be reworked into its own module once it fills up <3
*/

use crate::sim_fix_math::*;
use crate::sim_map::MapTile;

/// Unit's id. u64 because that's what hec's Entity converts to/from.
pub type UId = u64;

pub type TickNum = u32;

pub type PId = u8; // player Id

pub const N_ABILITY_CAP: u32 = 3;

// Target. Either posiion or entity.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ObjTarget {
    Position(Pos),
    Entity(UId),
    None,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TeamAlliance {
    Neutral,
    Alliance(PId),
    Spectator,
}

impl TeamAlliance {
	pub fn to_str(&self) -> String{
		match self {
			TeamAlliance::Neutral => String::from("neutral"),
			TeamAlliance::Alliance(team) => team.to_string(),
			TeamAlliance::Spectator => String::from("spec"),
		}
	}
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PlayerId {
    id: PId,
    team: TeamAlliance,
}

impl PlayerId {
    pub fn new(id: PId, team: TeamAlliance) -> Self {
        PlayerId { id: id, team: team }
    }

    pub fn get_id(&self) -> PId {
        self.id
    }

    pub fn get_team(&self) -> TeamAlliance {
        self.team
    }
}

// #[derive(Debug, PartialEq, Clone, Copy)]
// pub enum EngineMessage {
//     ObjPosColl(UId, Pos, FixF), // Message carrying position and collision radius info
//     ObjMove(UId, Pos),
//     ObjNextPos(UId, Pos),
//     ObjDest(UId, Pos),
//     ObjPathTmp(UId, [Pos; 20]), // To visualise pathfinding. Sends next n steps.
//     StructurePosTmp(UId, Pos),
//     MapTile(Pos, MapTile),
//     Fps(u64, u64),
//     //None, // this message sucks, but whatever
// }

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RenderMessage {
    Destination(UId, PId, Pos),
    Spawn(PId, Pos),
    //SpawnStructureTmp(Pos, PId),
    UseAbility(UId, PId, u32, ObjTarget),
    Break,
}

/// New engine messages
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum  SimMsg {
    Warn(PlayerId, SimWarnMsg), // PlayerId because player shouldn't hear bot's warnings.
    StateChange(SimStateChng),
    SimInfo(SimStateInfo)
}

/// Simulation warning messages variants
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SimWarnMsg{
    AbilTrgInvalid, // Target invalid
    AbilUnavailable, // on cooldown
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