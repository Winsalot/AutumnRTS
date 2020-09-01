use crate::common::*;
//use crate::sim_fix_math::*;

pub struct OrderQueueComp {
	orders: [UnitOrder; ORDER_SCHEDULE_MAX],
}


impl OrderQueueComp {
	pub fn new() -> Self {
		OrderQueueComp{
			orders: [UnitOrder::None; ORDER_SCHEDULE_MAX],
		}
	}

	
	pub fn set_single_order(&mut self, order: UnitOrder){
		// Erase old orders:
		self.orders = [UnitOrder::None; ORDER_SCHEDULE_MAX];
		// Set new single order:
		self.orders[0] = order;
	}

	pub fn get_current_order(&self) -> &UnitOrder{
		&self.orders[0]
	}

	pub fn current_order_completed(&mut self) {
		for i in 0..(ORDER_SCHEDULE_MAX - 1){
			self.orders[i] = self.orders[i+1];
		}
		self.orders[ORDER_SCHEDULE_MAX - 1] = UnitOrder::None;
	}
}