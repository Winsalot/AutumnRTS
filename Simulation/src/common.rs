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

pub type AbilityID = u8;

pub const N_ABILITY_CAP: AbilityID = 3;

pub const ORDER_SCHEDULE_MAX: usize = 30;

// This a tricky one. Means that orders for groups above this number will start to act funny.
// However, this will be adressed once it becomes a problem
pub const UNIT_GROUP_CAP: usize = 32; // >32 is possible but complicates things (no traits like Debug or PartialEq)

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
    pub fn to_str(&self) -> String {
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

/// Decoupled from RenderMessage because in the future Renderer will send orders for group of units.
/// But UnitOrder is always specific for a single unit.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UnitOrder {
    None,
    MoveTo(Pos),
    Ability(AbilityID, ObjTarget),
}

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
    UnitUnavailable,  // Invalid unit. Maybe already dead.
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UnitState {
    Idle,
    Move,
    PathfindAndMove,
    UseAbility(AbilityID),
    UseAbilityFailed(SimWarnMsg),
}
