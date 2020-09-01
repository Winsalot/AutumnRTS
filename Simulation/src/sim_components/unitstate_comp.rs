use crate::common::*;
//use crate::sim_fix_math::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UnitState{
	CoolDown(TickNum),
	Idle,
}


pub struct UnitStateComp {
	// I really have doubt about this. What happens when I implement group orders?
	order_queue: [UnitOrder; ORDER_SCHEDULE_MAX],
	curr_state: UnitState,
}

impl UnitStateComp {
	pub fn new() -> Self {
		UnitStateComp{
			order_queue: [UnitOrder::None; ORDER_SCHEDULE_MAX],
			curr_state: UnitState::Idle,
		}
	}

	pub fn set_single_order(&mut self, order: UnitOrder){
		// Erase old orders:
		self.order_queue = [UnitOrder::None; ORDER_SCHEDULE_MAX];
		// Set new single order:
		self.order_queue[0] = order;
	}
}