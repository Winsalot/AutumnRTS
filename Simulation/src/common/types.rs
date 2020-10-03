use crate::sim_fix_math::*;

/// Unit's id. u64 because that's what hec's Entity converts to/from.
pub type UId = u64;

pub type TickNum = u32;

pub type PId = u8; // player Id

pub type AbilityID = u8;

pub const N_ABILITY_CAP: AbilityID = 3;

pub const N_WEAPON_CAP: AbilityID = 2; // Not sure about type, but good for now.

pub const ORDER_SCHEDULE_MAX: usize = 30;

// This a tricky one. Means that orders for groups above this number will start to act funny.
// However, this will be adressed once it becomes a problem
pub const UNIT_GROUP_CAP: usize = 32; // >32 is possible but complicates things (no traits like Debug or PartialEq)

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

// Target. Either posiion or entity.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ObjTarget {
    Position(Pos),
    Entity(UId),
    None,
}

/// Decoupled from RenderMessage because in the future Renderer will send orders for group of units.
/// But UnitOrder is always specific for a single unit.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UnitOrder {
    None,
    MoveTo(Pos),
    Ability(AbilityID, ObjTarget),
    ForceAttack(ObjTarget),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UnitState {
    Idle,
    Move,
    PathfindAndMove,
    UseAbility(AbilityID),
    UseAbilityFailed,
    FireWeapons([bool; N_WEAPON_CAP as usize]),
}
