// Simulation rendering decoupling and use of MPSC requries to use types that implement Copy trait. Vec doesn't implement it, therefore I will use fixed size array for abilities. The result is that unit will have a capped number of active abilities.

/*// This shit sucks either way.
I would prefer to have ability component that holds array of abilities. Seems like a cleaner approach to having a separate component for every ability.
But the problem I am worried about is sim-rend messenger. This part is always trouble. How would a message look like for this kind of data. I think either way I would be throwing around indices. Which might not be that bad though. Idk.

So I guess idea is that rend sends index of ability to use.

And engine informs renderer of ability by sending index and enum value. But that means Ability enum should be decoupled from the component itself.
*/

use crate::common::AbilityID;
use crate::common::TickNum;
use crate::common::N_ABILITY_CAP;
use crate::sim_abilities_list::*;
use crate::sim_fix_math::*;

//const N_ABILITY_CAP: usize = 3;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ActiveAbilityComp {
    abilities: [Ability; N_ABILITY_CAP as usize],
}

impl ActiveAbilityComp {
    pub fn builder() -> Self {
        let mut abil = [Ability::mundane_abil(); N_ABILITY_CAP as usize];
        abil[0] = Ability::build_structure(FixF::from_num(2), 0);
        ActiveAbilityComp { abilities: abil }
    }

    pub fn get_ability(&self, id: AbilityID) -> &Ability {
        &self.abilities[id as usize]
    }

    pub fn start_cooldown(&mut self, abil_id: &AbilityID, curr_tick: &TickNum) {
        let abil = &mut self.abilities[*abil_id as usize];
        abil.start_cd(curr_tick);
    }
}
