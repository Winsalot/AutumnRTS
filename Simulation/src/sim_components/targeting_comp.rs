use crate::common::*;
use crate::sim_fix_math::*;

///Simply put unit's target for hostile action. Target can be nothing (idle), 
/// Position (firing into the ground), another unit (firing at it).
/// Automatic targeting type means unit chooses and attacks target on its own (eg. firing at nearby enemy). 
/// "Order"" type target means player has given that unit order to  target particular unit. In this case it could target ground, allies or lower priority enemies (eg. attacking structures over units). 
/// Giving order of "None" target could be interpreted as hold fire & position command.
/// Target range is range at which unit "notices" hostiles and targets them. Can be much bigger than weapon range.

/// Overall I feel like I am going in wrong direction wih this. Should probably make this more specific. Like what if healer unit can heal and attack. Should heal ability rely on different targeting component?

enum TargetingType{
	Order,
	Automatic
}

pub struct TargetComp {
	/// Current target
	target: ObjTarget,
	/// How was current target chosen?
	trg_mode: TargetingType,
	/// Range to automatically search for targets
	trg_range: FixF,
}

impl TargetComp {
	pub fn new(range: FixF) -> Self {
		TargetComp{
			target: ObjTarget::None,
			trg_mode: TargetingType::Automatic,
			trg_range: range
		}
	}
}