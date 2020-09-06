use crate::common::*;
use crate::sim_fix_math::*;
use crate::sim_weapon_list::Weapon;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct WeaponComp {
    weapons: [Weapon; N_WEAPON_CAP as usize],
}

impl WeaponComp {
    pub fn new(weapons: Vec<Weapon>) -> Self {
        let mut unit_weapons = [Weapon::None; N_WEAPON_CAP as usize];

        for i in 0..(weapons.len().min(N_WEAPON_CAP as usize)) {
            unit_weapons[i] = weapons[i];
        }

        WeaponComp {
            weapons: unit_weapons,
        }
    }

    // Gives unit a standard issue weapon.
    pub fn new_debug() -> Self {
        WeaponComp::new(vec![Weapon::new_gun(FixF::from_num(10), 3, 1)])
    }
}
