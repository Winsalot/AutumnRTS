use crate::common::*;
use crate::sim_fix_math::*;

// Using enum like this is uncharted waters for me. But it might work out in the end :)
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Weapon {
    None,
    Gun {
        range: FixF,          // weapon range
        fire_cd: TickNum,     // how frequently can weapon fire
        fire_cd_end: TickNum, // Next time when unit will be able to fire
        charge_dur: TickNum,  // How many ticks it takes to fire single shot.
    },
}

impl Weapon {
    pub fn new_gun(range: FixF, fire_cd: TickNum, charge_dur: TickNum) -> Self {
        Weapon::Gun {
            range: range,
            fire_cd: fire_cd,
            fire_cd_end: 0,
            charge_dur: charge_dur,
        }
    }
}
