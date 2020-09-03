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
	// I really have doubt about this. What happens when I implement group orders?
	//order_queue: [UnitOrder; ORDER_SCHEDULE_MAX],
	state: UnitState,
	//substate: UnitSubState,
	cooldown_end: TickNum,
}

impl UnitStateComp {
	pub fn new() -> Self {
		UnitStateComp{
			//order_queue: [UnitOrder::None; ORDER_SCHEDULE_MAX],
			state: UnitState::Idle,
			//substate: UnitSubState::Idle,
			cooldown_end: 0,
		}
	}

	pub fn get_state(&self) -> &UnitState {
		&self.state
	}

	pub fn set_state(&mut self, state: UnitState) {
		self.state = state;
		// Setting a new state should update to initial substate.
		// match self.state {
		// 	UnitState::Idle => {self.substate = UnitSubState::Idle;},
		// 	UnitState::Move => {self.substate = UnitSubState::PathfindAndMove;},
		// }
	}

	pub fn pathfind(&self) -> bool {
		// match self.substate {
		match self.state {
			// UnitSubState::PathfindAndMove => true,
			UnitState::PathfindAndMove => true,
			_ => false,
		}
	}

// Shoudl be checked in behaviour AI system
	// // changes state to a new one in order
	// pub fn pathfind_finished(&mut self) {
	// 	if self.substate == UnitSubState::PathfindAndMove {
	// 		self.substate = UnitSubState::Move;
	// 	}
	// }

	pub fn can_move(&self, current_tick: &TickNum) -> bool {
		let mut ret = false;
		if &self.cooldown_end <= current_tick {
			// match self.substate {
			// 	UnitSubState::PathfindAndMove => ret = true,
			// 	UnitSubState::Move => ret = true,
			// 	_ => {},
			// };
			match self.state {
				UnitState::PathfindAndMove => ret = true,
				UnitState::Move => ret = true,
				_ => {},
			};
		}
		ret
	}

	pub fn just_moved(
		&mut self, 
		current_tick: &TickNum,
		cooldown: &TickNum) {
		self.cooldown_end = current_tick + cooldown;
	}
}