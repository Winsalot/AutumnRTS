use crate::common::SimMsg::StateChange;
use crate::common::SimStateChng::*;
use crate::sim_components::order_queue_comp::OrderQueueComp;
use crate::sim_weapon_list::WeaponType;

use crate::sim_components::active_ability_comp::*;
use crate::sim_components::projectile_comp::*;
use crate::sim_components::sim_unit_base_components::*;
use crate::sim_components::structure_comp::*;
use crate::sim_components::targeting_comp::*;
use crate::sim_components::unitstate_comp::*;
use crate::sim_components::weapon_comp::*;
use crate::sim_fix_math::Pos;
use hecs::*;

use crate::common::*;

use crate::sim_ecs::*;
use crate::sim_fix_math::FixF;

// These are temporary semi-hard-coded functions that spawn entities.

pub fn plc_unit(sim: &mut SimState, owner: PId, pos: Pos, speed: FixF, coll_r: FixF) {
    let mut unit_builder = EntityBuilder::new();
    let player = sim.res.players.get(owner);

    if let Some(player) = player {
        unit_builder.add(TypeNameComp::new("placeholder"));
        unit_builder.add(PositionComp::new(pos));
        unit_builder.add(NextPosComp::new(pos));
        unit_builder.add(DestinationComp::new(pos));
        unit_builder.add(SpeedComponent::new(speed, 1));
        unit_builder.add(CollComp::new(coll_r));
        unit_builder.add(IdComp::new(&mut sim.res.id_counter, player));
        unit_builder.add(PathComp::new());
        unit_builder.add(TargetComp::new(FixF::from_num(3)));
        unit_builder.add(ActiveAbilityComp::builder());

        let new_entity = sim.ecs.spawn(unit_builder.build());

        let msg = StateChange(ObjSpawn(sim.res.id_counter - 1, *player, pos, coll_r));
        sim.res.send_batch.push(msg);

        sim.res.id_map.insert(sim.res.id_counter - 1, new_entity);
    }
}

pub fn plc_smart_unit(sim: &mut SimState, owner: PId, pos: Pos, speed: FixF, coll_r: FixF) {
    let mut unit_builder = EntityBuilder::new();
    let player = sim.res.players.get(owner);

    if let Some(player) = player {
        unit_builder.add(TypeNameComp::new("placeholder"));
        unit_builder.add(PositionComp::new(pos));
        unit_builder.add(NextPosComp::new(pos));
        unit_builder.add(SpeedComponent::new(speed, 1));
        unit_builder.add(CollComp::new(coll_r));
        unit_builder.add(IdComp::new(&mut sim.res.id_counter, player));
        unit_builder.add(PathComp::new());
        unit_builder.add(TargetComp::new(FixF::from_num(3)));
        unit_builder.add(ActiveAbilityComp::builder());
        unit_builder.add(UnitStateComp::new());
        unit_builder.add(OrderQueueComp::new());
        unit_builder.add(WeaponComp::new_debug());

        let new_entity = sim.ecs.spawn(unit_builder.build());

        let msg = StateChange(ObjSpawn(sim.res.id_counter - 1, *player, pos, coll_r));
        sim.res.send_batch.push(msg);

        sim.res.id_map.insert(sim.res.id_counter - 1, new_entity);
    }
}

pub fn plc_building(sim: &mut SimState, owner: PId, pos: Pos) {
    let mut unit_builder = EntityBuilder::new();

    unit_builder.add(TypeNameComp::new("placeholder_building"));
    unit_builder.add(IdComp::new(
        &mut sim.res.id_counter,
        sim.res.players.get(owner).unwrap(),
    ));
    unit_builder.add(StructureComp::new(pos));

    let new_entity = sim.ecs.spawn(unit_builder.build());

    // let msg = EngineMessage::StructurePosTmp(sim.res.id_counter - 1, pos.round());
    let msg = StateChange(StructurePosTmp(sim.res.id_counter - 1, pos.round()));
    sim.res.send_batch.push(msg);

    sim.res.id_map.insert(sim.res.id_counter - 1, new_entity);
    sim.map.map_mem.add(vec![pos]);
}

pub fn plc_projectile(
    sim: &mut SimState,
    shooter: &UId,
    shooter_pos: &Pos,
    _wep_type: &WeaponType,
    trg: &ObjTarget,
    speed: FixF,
) -> Entity {
    // Has id, projectile, speed, pos, target, components

    // For real though. Maybe It don't even need ID component?
    // It will be interacting with world. But world won't be interacting with it.

    let mut unit_builder = EntityBuilder::new();

    // SHould spawn at shooter's position:
    unit_builder.add(PositionComp::new(*shooter_pos));
    unit_builder.add(SpeedComponent::new(speed, 0));
    unit_builder.add(TargetComp::from_trg(trg));
    unit_builder.add(ProjectileComp::new(shooter, FixF::from_num(1)));

    sim.ecs.spawn(unit_builder.build())
}
