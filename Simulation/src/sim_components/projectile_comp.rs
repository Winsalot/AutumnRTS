use crate::common::UId;
use crate::sim_fix_math::FixF;


// A component shared by all projectiles.
// Stores the UId of shooter
// Also stores what effects it does on impact


#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ProjectileComp {
	shooter: UId,
	// effects: Vec<StatusEffects>, // someday it will look like this
	dmg: FixF,
}

impl ProjectileComp {
	pub fn new(shooter: &UId, dmg: FixF) -> Self {
		ProjectileComp{
			shooter: *shooter,
			dmg: dmg,
		}
	}
}