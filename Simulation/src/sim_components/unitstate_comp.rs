use crate::common::*;
//use crate::sim_fix_math::*;


pub struct UnitStateComp {
	// I really have doubt about this. What happens when I implement group orders?
	order_queue: [UnitOrder; ORDER_SCHEDULE_MAX],
	curr_state: UnitState,
	cooldown_end: TickNum,
}

impl UnitStateComp {
	pub fn new() -> Self {
		UnitStateComp{
			order_queue: [UnitOrder::None; ORDER_SCHEDULE_MAX],
			curr_state: UnitState::Idle,
			cooldown_end: 0,
		}
	}

	pub fn set_single_order(&mut self, order: UnitOrder){
		// Erase old orders:
		self.order_queue = [UnitOrder::None; ORDER_SCHEDULE_MAX];
		// Set new single order:
		self.order_queue[0] = order;
	}

	// pub fn start_tick(&mut self) {
	// 	if self.cooldown <= 0 {
	// 		return;
	// 	}
	// 	self.cooldown -= 1;
	// }

	pub fn get_current_order(&self) -> &UnitOrder{
		&self.order_queue[0]
	}

	pub fn current_order_completed(&mut self) {
		for i in 0..(ORDER_SCHEDULE_MAX - 1){
			self.order_queue[i] = self.order_queue[i+1];
		}
		self.order_queue[ORDER_SCHEDULE_MAX - 1] = UnitOrder::None;
	}

	pub fn get_state(&self) -> &UnitState {
		&self.curr_state
	}

	pub fn set_state(&mut self, state: UnitState) {
		self.curr_state = state;
	}
}