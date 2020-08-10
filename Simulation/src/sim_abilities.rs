//Chill my dude. This file contains list of abilities and subsystems that "cast" them. subsystems Themselves are called from within systems in main game loop.

// Fuck me. I forgot that I need to pass aditional arguments to subsystems. One more Enum? This is getting crazy tbh.

use crate::sim_fix_math::*;
use crate::sim_ecs::*;


pub enum Ability {
	GenericAbility{pw_cost: i32, cooldown_end_at: u64, range: FixF, damage: i32},
	BuildSimpleStructure,

}

pub fn use_ability(sim: &mut SimState, ability: &mut Ability){

	match ability {
		Ability::BuildSimpleStructure => build_simple_structure(),
		Ability::GenericAbility{pw_cost: _pw,
			cooldown_end_at: mut cd, 
			range: _r, 
			damage: dmg} => generic_ability(sim, &mut cd, &dmg),
		//_ => ()
	}
}

fn build_simple_structure(){

}

fn generic_ability(
	_sim: &mut SimState, 
	cooldown_end_at: &mut u64, 
	damage: &i32) {


	println!("Casting ability! Deals {:?} damage!", damage); 
	*cooldown_end_at += 30; // 30 ticks cooldown.
}