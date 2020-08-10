// Simulation rendering decoupling and use of MPSC requries to use types that implement Copy trait. Vec doesn't implement it, therefore I will use fixed size array for abilities. The result is that unit will have a capped number of active abilities.

/*// This shit sucks either way.
I would prefer to have ability component that holds array of abilities. Seems like a cleaner approach to having a separate component for every ability.
But the problem I am worried about is sim-rend messenger. This part is always trouble. How would a message look like for this kind of data. I think either way I would be throwing around indices. Which might not be that bad though. Idk.

So I guess idea is that rend sends index of ability to use. 

And engine informs renderer of ability by sending index and enum value. But that means Ability enum should be decoupled from the component itself.
*/

use crate::sim_abilities::*;

const N_ABILITY_CAP: usize = 3;


pub struct ActiveAbilityComp {
	abilities: [Option<Ability>; N_ABILITY_CAP],
}