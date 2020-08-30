use crate::common::*;
use crate::sim_fix_math::*;

// Cooldown component common for multiple actions.
// My plan is to use it for a common colldown for stop-to-fire and stop-to-cast.
// Maybe some abilities could also increase or decrease it.
// COuld also be used to prevent units from moving on every tick.


pub struct CDComp {
	// game tick at which the cooldown expire.
	cd_end: TickNum,
}

impl CDComp {

	// Initially spawns with no cooldown.
	pub fn new() -> Self{
		CDComp{
			cd_end: 0,
		}
	}

	pub fn set_cd(&mut self, current_tick: &TickNum, cd: TickNum){
		self.cd_end = current_tick + cd;
	}

	// Check if unit is under cooldown right now.
	pub fn is_on_cd(&self, current_tick: &TickNum) -> bool {
		&self.cd_end <= current_tick
	}

	pub fn get_cd(&self, current_tick: &TickNum) -> TickNum {
		if &self.cd_end < current_tick {
			return 0;
		} else {
			return self.cd_end - current_tick;
		}
	}
}