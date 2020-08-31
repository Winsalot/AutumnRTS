use crate::common::*;
//use crate::sim_fix_math::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UnitState{
	CoolDown(TickNum),
	Idle,
}

/// Decoupled from RenderMessage because in the future Renderer will send orders for group of units.
/// But UnitOrder is always specific for a single unit.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UnitOrder {
    None,
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
}