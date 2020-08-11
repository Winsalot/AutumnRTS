//Chill my dude. This file contains list of abilities and subsystems that "cast" them. subsystems Themselves are called from within systems in main game loop.

use crate::common::*;
use crate::sim_fix_math::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Ability {
    Mundane,
    BuildSimpleStructure,
    GenericAbility {
        pw_cost: i32,
        cooldown_end_at: TickNum,
        range: FixF,
        damage: i32,
    },
}
