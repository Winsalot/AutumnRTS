use crate::common::*;
use crate::sim_fix_math::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct HealthComp {
    hp_max: FixF,
    hp: FixF,
    armour: FixF,
    regen_hp: FixF,
    regen_cd_dur: TickNum,
    regen_cd: TickNum,
}

impl HealthComp {
    pub fn new_debug() -> Self {
        HealthComp {
            hp_max: FixF::from_num(10),
            hp: FixF::from_num(10),
            armour: FixF::from_num(1),
            regen_hp: FixF::from_num(1),
            regen_cd_dur: 20,
            regen_cd: 20,
        }
    }

    pub fn _new(hp: FixF, arm: FixF, regen: FixF, regen_cd: TickNum) -> Self {
        HealthComp {
            hp_max: hp,
            hp: hp,
            armour: arm,
            regen_hp: regen,
            regen_cd_dur: regen_cd,
            regen_cd: regen_cd,
        }
    }

    pub fn get_hp(&self) -> &FixF {
        &self.hp
    }

    pub fn damage(&mut self, dmg: &FixF) {
        // damage always at least 1.
        self.hp -= (dmg - self.armour).max(FixF::from_num(1));
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    pub fn regen(&mut self) {
        // SHould be run once per tick
        self.regen_cd -= 1;
        // not the most elegant implementation, but straightforward enough.
        if self.regen_cd == 0 {
            self.regen_cd = self.regen_cd_dur;
            if self.hp < self.hp_max {
                self.hp += self.regen_hp;
            }
        }
    }
}

#[cfg(test)]
mod hp_tests {
    use crate::sim_components::health_comp::HealthComp;
    use crate::sim_fix_math::*;

    #[test]
    fn regen_test() {
        // cargo test -- --nocapture regen_test

        // This test shoudl simply not overflow to pass :)
        let mut hp_comp = HealthComp::new_debug();

        for _ in 0..30 {
            hp_comp.regen();
            println!("{:?}", hp_comp);
        }
    }

    #[test]
    fn damage_and_regen() {
        // cargo test -- --nocapture damage_and_regen

        let mut hp_comp = HealthComp::new_debug();

        hp_comp.damage(&FixF::from_num(10));

        assert_eq!(*hp_comp.get_hp(), FixF::from_num(1));

        for _ in 0..20 {
            hp_comp.regen();
        }

        assert_eq!(*hp_comp.get_hp(), FixF::from_num(2));

        hp_comp.damage(&FixF::from_num(0));

        assert_eq!(*hp_comp.get_hp(), FixF::from_num(1));

        hp_comp.damage(&FixF::from_num(-1));

        assert_eq!(*hp_comp.get_hp(), FixF::from_num(0));
        assert_eq!(hp_comp.is_alive(), false);
    }
}
