use crate::common::*;
use crate::sim_fix_math::*;

pub struct HealthComp {
    /// Health
    hp: FixF,
    // Armor for flat damage reduction:
    armour: FixF,
    // Regeneration:
    regen_hp: FixF,
    // intervals (cooldown) at which to apply regeneration:
    regen_cd: TickNum,
}

impl HealthComp {
    pub fn new_simple() -> Self {
        HealthComp {
            hp: FixF::from_num(50),
            armour: FixF::from_num(2),
            regen_hp: FixF::from_num(1),
            regen_cd: 20,
        }
    }

    pub fn new(hp: FixF, arm: FixF, regen: FixF, regen_cd: TickNum) -> Self {
        HealthComp {
            hp: hp,
            armour: arm,
            regen_hp: regen,
            regen_cd: regen_cd,
        }
    }

    pub fn get_hp(&self) -> FixF {
        self.hp.clone()
    }

    pub fn damage(&mut self, dmg: &FixF) {
        // damage always at least 1.
        self.hp += (self.armour - dmg).max(FixF::from_num(1));
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    pub fn regen(&mut self, current_tick: &TickNum) {
        // not the most elegant implementation, but straightforward enough.
        if (current_tick % self.regen_cd) == 0 {
            self.hp += self.regen_hp;
        }
    }
}
