use crate::common::*;
//use crate::sim_fix_math::*;

// // this is private enum of various substates.
// #[derive(Debug, PartialEq, Clone, Copy)]
// enum UnitSubState {
// 	Idle,
// 	PathfindAndMove,
// 	Move,
// }

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct UnitStateComp {
    state: UnitState,
    cooldown_end: TickNum,
}

impl UnitStateComp {
    pub fn new() -> Self {
        UnitStateComp {
            state: UnitState::Idle,
            cooldown_end: 0,
        }
    }

    pub fn get_state(&self) -> &UnitState {
        &self.state
    }

    pub fn set_state(&mut self, state: UnitState) {
        self.state = state;
    }

    pub fn pathfind(&self) -> bool {
        match self.state {
            UnitState::PathfindAndMove => true,
            _ => false,
        }
    }

    pub fn can_move(&self, current_tick: &TickNum) -> bool {
        //let mut ret = false;
        if &self.cooldown_end <= current_tick {
            match self.state {
                UnitState::PathfindAndMove => return true,
                UnitState::Move => return true,
                _ => return false,
            }
        }
        false
    }

    pub fn just_moved(&mut self, current_tick: &TickNum, cooldown: &TickNum) {
        self.cooldown_end = current_tick + cooldown;
    }
}
