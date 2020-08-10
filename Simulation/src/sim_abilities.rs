//Chill my dude. This file contains list of abilities and subsystems that "cast" them. subsystems Themselves are called from within systems in main game loop.

// Fuck me. I forgot that I need to pass aditional arguments to subsystems. One more Enum? This is getting crazy tbh.
use crate::common::*;
use crate::sim_components::sim_unit_base_components::PositionComp;
use crate::sim_systems::plc_building;
use crate::sim_fix_math::*;
use crate::sim_ecs::*;

use hecs::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Ability {
	Mundane,
	BuildSimpleStructure,
	GenericAbility{pw_cost: i32, cooldown_end_at: TickNum, range: FixF, damage: i32},
}

pub fn use_ability(
	sim: &mut SimState,
	entity: UId,
	target: ObjTarget,
	ability: &mut Ability){

	match ability {
		Ability::BuildSimpleStructure => build_simple_structure(sim, entity, target),
		Ability::GenericAbility{pw_cost: _pw,
			cooldown_end_at: mut cd, 
			range: _r, 
			damage: dmg} => generic_ability(sim, &mut cd, &dmg),
		Ability::Mundane => (),
	}
}

fn build_simple_structure(
	sim: &mut SimState,
	id: UId,
	target: ObjTarget,
	) {

	if let ObjTarget::Position(pos) = target {
		// Now find the rounded position (tile) of caster
		// And rounded position (tile) of target
		// If tiles are adjacent then spawn_structure.

		if !sim.map.within(pos){
			return;
		}

		let pos0 = pos.round();

		let builder_pos = sim.ecs.get::<PositionComp>(Entity::from_bits(id)).unwrap();

		let pos1 = builder_pos.get_pos().round();
		drop(builder_pos); // fuck you borrow checker.

		// is adjacent?
		if pos0.dist(&pos1) < FixF::from_num(2.0){
			// now actually spawn a structure.
			if sim.map.tile_from_pos(pos0).blocks_path(){
                return;
            }

            if sim.map.map_mem.get_blocked().contains(&pos0.round()){
                return;
            }
            let mut new_structure = plc_building(pos0, &mut sim.id_counter);
            let e = sim.ecs.spawn(new_structure.build());

            sim.map.add_structure(vec![pos0]);

            let msg = EngineMessage::StructurePosTmp(e.to_bits(), pos0.round());
            sim.send_batch.push(msg);
		}
	}
}

fn generic_ability(
	_sim: &mut SimState, 
	cooldown_end_at: &mut TickNum, 
	damage: &i32) {


	println!("Casting ability! Deals {:?} damage!", damage); 
	*cooldown_end_at += 30; // 30 ticks cooldown.
}

