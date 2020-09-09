use crate::common::*;
use crate::sim_fix_math::*;
use crate::sim_weapon_list::Weapon;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct WeaponComp {
    weapons: [Weapon; N_WEAPON_CAP as usize],
}

impl WeaponComp {
    pub fn new(weapons: Vec<Weapon>) -> Self {
        let mut unit_weapons = [Weapon::unarmed(); N_WEAPON_CAP as usize];

        for i in 0..(weapons.len().min(N_WEAPON_CAP as usize)) {
            unit_weapons[i] = weapons[i];
        }

        WeaponComp {
            weapons: unit_weapons,
        }
    }

    // Gives unit a standard issue weapon.
    pub fn new_debug() -> Self {
        WeaponComp::new(vec![Weapon::debug_rifle(FixF::from_num(5))])
    }

    pub fn get_max_range(&self) -> FixF {
        let mut max_range = FixF::from_num(0);

        for i in 0..(N_WEAPON_CAP as usize) {
            max_range = max_range.max(self.weapons[i].get_range());
        }

        max_range
    }

    pub fn get_weapons_in_range(&self, range: &FixF) -> [bool; N_WEAPON_CAP as usize] {
        let mut in_range = [false; N_WEAPON_CAP as usize];

        for i in 0..(N_WEAPON_CAP as usize) {
            if self.weapons[i].get_range() >= *range {
                in_range[i] = true;
            }
        }

        in_range
    }
}
