use crate::common::*;
use crate::sim_fix_math::*;

// weapon states common for all weapons:
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum WeaponState {
    Ready,
    Aiming { cd: TickNum },
    Firing { cd: TickNum },
    Cooldown { cd: TickNum },
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct WeaponParams {
    range: FixF,       // weapon range
    aim_dur: TickNum,  // how frequently can weapon fire
    fire_dur: TickNum, // How long the firing itself takes
    cd_dur: TickNum,   // how long cooldown takes
}

impl WeaponParams {
    pub fn new(range: FixF, aim_dur: TickNum, fire_dur: TickNum, cd_dur: TickNum) -> Self {
        WeaponParams {
            range: range,
            aim_dur: aim_dur,
            fire_dur: fire_dur,
            cd_dur: cd_dur,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum WeaponType {
    None, // No weapon.
    Gun,  // Simple kinetic projectile weapon
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Weapon {
    state: WeaponState,
    params: WeaponParams,
    wep_type: WeaponType,
}

impl Weapon {
    // Standard weapon for debug
    pub fn debug_rifle(range: FixF) -> Self {
        let params = WeaponParams::new(range, 1, 3, 5);
        Weapon {
            state: WeaponState::Ready,
            params: params,
            wep_type: WeaponType::Gun,
        }
    }

    pub fn debug_rifle_fast(range: FixF) -> Self {
        let params = WeaponParams::new(range, 0, 1, 0);
        Weapon {
            state: WeaponState::Ready,
            params: params,
            wep_type: WeaponType::Gun,
        }
    }

    pub fn unarmed() -> Self {
        let params = WeaponParams::new(FixF::from_num(0), 0, 0, 0);
        Weapon {
            state: WeaponState::Ready,
            params: params,
            wep_type: WeaponType::None,
        }
    }

    // #[allow(non_shorthand_field_patterns)]
    pub fn get_range(&self) -> FixF {
        self.params.range
    }

    pub fn get_state(&self) -> &WeaponState {
        &self.state
    }

    pub fn get_type(&self) -> &WeaponType {
        &self.wep_type
    }

    // THIS IS horrible
    pub fn update_idle(&mut self) {
        // In general should revert to ready if was aiming or ready.
        // If was firing then goes on cooldown.
        // if Was on cd then reduces cd by 1.

        let new_state: WeaponState;

        match self.wep_type {
            WeaponType::None => return,
            WeaponType::Gun => {
                new_state = self.update_idle_gun();
            }
        }

        self.state = new_state;
    }

    pub fn update_firing(&mut self) {
        let new_state: WeaponState;

        match self.wep_type {
            WeaponType::None => return,
            WeaponType::Gun => {
                new_state = self.update_firing_gun();
            }
        }

        self.state = new_state;
    }

    fn update_firing_gun(&self) -> WeaponState {
        // update cooldowns and advance in sequence:
        // ready -> aiming -> firing -> cooldown -> ready

        let mut new_state = self.state;

        for _ in 0..4 {
            match new_state {
                WeaponState::Ready => {
                    new_state = WeaponState::Aiming {
                        cd: self.params.aim_dur,
                    };
                    if self.params.aim_dur == 0 {
                        continue;
                    }
                }
                WeaponState::Aiming { cd } => {
                    if cd <= 1 {
                        new_state = WeaponState::Firing {
                            cd: self.params.fire_dur,
                        };
                        // no idea when this would actually be useful. But lets be consistent:
                        if self.params.fire_dur == 0 {
                            continue;
                        }
                    } else {
                        new_state = WeaponState::Aiming { cd: cd - 1 };
                    }
                }
                WeaponState::Firing { cd } => {
                    if cd <= 1 {
                        new_state = WeaponState::Cooldown {
                            cd: self.params.cd_dur,
                        };
                        // no idea when this would actually be useful. But lets be consistent:
                        if self.params.cd_dur == 0 {
                            continue;
                        }
                    } else {
                        new_state = WeaponState::Firing { cd: cd - 1 };
                    }
                }
                WeaponState::Cooldown { cd } => {
                    if cd <= 1 {
                        new_state = WeaponState::Ready;
                        continue;
                    } else {
                        new_state = WeaponState::Cooldown { cd: cd - 1 };
                    }
                }
            }
            // end loop if match pattern was exhausted.
            break;
        }

        new_state
    }

    fn update_idle_gun(&self) -> WeaponState {
        // this is pure spaghet

        let mut new_state = self.state;

        // Loop to skip over states with zero cooldown.
        // Using for loop to avoid accidental infinite loop haha
        for _ in 0..4 {
            match new_state {
                WeaponState::Ready => {
                    new_state = WeaponState::Ready;
                    break;
                }
                WeaponState::Aiming { .. } => {
                    new_state = WeaponState::Ready;
                    break;
                }
                WeaponState::Firing { .. } => {
                    new_state = WeaponState::Cooldown {
                        cd: self.params.cd_dur,
                    };
                    // If cooldown takes more than 1 frame
                    if self.params.cd_dur > 0 {
                        break;
                    }
                }
                WeaponState::Cooldown { cd } => {
                    if cd <= 1 {
                        new_state = WeaponState::Ready;
                        break;
                    } else {
                        new_state = WeaponState::Cooldown { cd: cd - 1 };
                        break;
                    }
                }
            }
        }

        new_state
    }
}

#[cfg(test)]
mod weapon_fsm_test {

    use crate::sim_weapon_list::*;

    #[test]
    fn test_gun() {
        // cargo test -- --nocapture test_gun

        let mut gun = Weapon::debug_rifle(FixF::from_num(5));

        {
            assert_eq!(*gun.get_state(), WeaponState::Ready);
        }

        gun.update_idle();

        {
            assert_eq!(*gun.get_state(), WeaponState::Ready);
        }

        gun.update_firing();

        // println!("{:?}", gun);

        {
            assert_eq!(*gun.get_state(), WeaponState::Aiming { cd: 1 });
        }

        gun.update_firing();

        {
            assert_eq!(*gun.get_state(), WeaponState::Firing { cd: 3 });
        }

        gun.update_idle();

        {
            assert_eq!(*gun.get_state(), WeaponState::Cooldown { cd: 5 });
        }

        // remake gun:
        let mut gun = Weapon::debug_rifle(FixF::from_num(5));

        for _ in 0..5 {
            gun.update_firing();
        }

        {
            assert_eq!(*gun.get_state(), WeaponState::Cooldown { cd: 5 });
        }

        gun.update_firing();

        {
            assert_eq!(*gun.get_state(), WeaponState::Cooldown { cd: 4 });
        }

        gun.update_idle();

        {
            assert_eq!(*gun.get_state(), WeaponState::Cooldown { cd: 3 });
        }

        gun.update_idle();
        gun.update_idle();
        gun.update_idle();

        {
            assert_eq!(*gun.get_state(), WeaponState::Ready);
        }

        let mut gun = Weapon::debug_rifle_fast(FixF::from_num(5));

        gun.update_firing();

        {
            assert_eq!(*gun.get_state(), WeaponState::Firing { cd: 1 });
        }

        gun.update_firing();

        {
            assert_eq!(*gun.get_state(), WeaponState::Firing { cd: 1 });
        }
    }
}
