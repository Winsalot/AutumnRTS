use crate::common::*;
//use crate::sim_fix_math::*;


pub struct UnitStateComp {
	// I really have doubt about this. What happens when I implement group orders?
	//order_queue: [UnitOrder; ORDER_SCHEDULE_MAX],
	curr_state: UnitState,
	cooldown_end: TickNum,
}

impl UnitStateComp {
	pub fn new() -> Self {
		UnitStateComp{
			//order_queue: [UnitOrder::None; ORDER_SCHEDULE_MAX],
			curr_state: UnitState::Idle,
			cooldown_end: 0,
		}
	}

	pub fn get_state(&self) -> &UnitState {
		&self.curr_state
	}

	pub fn set_state(&mut self, state: UnitState) {
		self.curr_state = state;
	}
}