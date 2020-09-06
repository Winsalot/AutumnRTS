//Weird part of code. Could be placed in common.rs. neither component, nor syustem.

use crate::common::*;
use crate::sim_fix_math::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AbilityEffect {
    Mundane,
    BuildSimpleStructure,
    GenericAbility { damage: i32 },
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Ability {
    ability: AbilityEffect,
    range: FixF,
    cd: TickNum,
    cd_end: TickNum,
}

impl Ability {
    pub fn mundane_abil() -> Self {
        Ability {
            ability: AbilityEffect::Mundane,
            range: FixF::from_num(0),
            cd: 0,
            cd_end: 0,
        }
    }

    pub fn build_structure(range: FixF, cd: TickNum) -> Self {
        Ability {
            ability: AbilityEffect::BuildSimpleStructure,
            range: range,
            cd: cd,
            cd_end: 0,
        }
    }

    pub fn get_range(&self) -> &FixF {
        &self.range
    }

    pub fn get_cd_end(&self) -> &TickNum {
    	&self.cd_end
    }

    pub fn start_cd(&mut self, curr_tick: &TickNum){
    	self.cd_end += curr_tick + self.cd;
    }

    pub fn get_effect(&self) -> &AbilityEffect{
    	&self.ability
    }
}
