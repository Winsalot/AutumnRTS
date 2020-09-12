use crate::sim_components::health_comp::HealthComp;
use crate::sim_components::projectile_comp::*;
use crate::sim_components::sim_unit_base_components::*;
use crate::sim_components::targeting_comp::*;
use crate::sim_ecs::SimState;
use crate::sim_fix_math::*;
use hecs::Entity;
// use crate::common::*;
use crate::sim_systems::targeting::*;

pub fn sys_projectiles(sim: &mut SimState) {
    update_projectile_positions(sim);
    projectile_impact(sim);
}

// Updates projectile positions (no collisions)
fn update_projectile_positions(sim: &mut SimState) {
    type ToQuery<'a> = (&'a PositionComp, &'a SpeedComponent, &'a TargetComp);

    let mut to_move_projectiles: Vec<(Entity, Pos)> = vec![];

    for (e_id, (pos_comp, speed_comp, trg_comp)) in
        sim.ecs.query::<ToQuery>().with::<ProjectileComp>().iter()
    {
        if let Some(trg_pos) = target_to_pos(sim, trg_comp.get_trg()) {
            let distance = pos_comp.get_pos().dist(&trg_pos);

            // There is error in distance calculations when using fixed point math.
            // So lets be safe:
            match distance > *speed_comp.get_speed() {
                true => {
                    let dx = (*pos_comp.get_pos() - trg_pos) / distance;
                    let next_pos = *pos_comp.get_pos() - dx * (*speed_comp.get_speed());
                    to_move_projectiles.push((e_id, next_pos));
                }
                false => {
                    to_move_projectiles.push((e_id, trg_pos));
                }
            }
        }
    }

    for (e_id, next_pos) in to_move_projectiles.iter() {
        let mut pos_comp = sim.ecs.get_mut::<PositionComp>(*e_id).unwrap();
        pos_comp.set_pos(*next_pos);
    }
}

// Projectile impact with target. Deals damage and despawns projectile
// TODO: this whole implementation is trash and should be rewritten someday
fn projectile_impact(sim: &mut SimState) {
    // Check if reached target.
    // Interact with target.
    // Despawn projectile.

    type ToQuery<'a> = (&'a PositionComp, &'a TargetComp, &'a ProjectileComp);

    let mut to_despawn: Vec<Entity> = vec![];
    let mut to_damage: Vec<(Entity, FixF)> = vec![];

    for (e_id, (pos_comp, trg_comp, proj_comp)) in sim.ecs.query::<ToQuery>().iter() {
        if let Some(trg_pos) = target_to_pos(sim, trg_comp.get_trg()) {
            if trg_pos == *pos_comp.get_pos() {
                let impact_pos = pos_comp.get_pos();

                to_despawn.push(e_id);

                // yes this inefficient as fuck:
                // And yes, this is grid search
                type ToQueryTrg<'a> = (&'a PositionComp, &'a CollComp);

                for (e_id1, (pos_comp1, coll_comp1)) in
                    sim.ecs.query::<ToQueryTrg>().with::<HealthComp>().iter()
                {
                    // If impact happens within collision radius of target
                    if impact_pos.dist(pos_comp1.get_pos()) < *coll_comp1.get_r() {
                        to_damage.push((e_id1, *proj_comp.get_dmg()));
                    }
                }
            }
        } else {
            // no target, so despawn:
            to_despawn.push(e_id);
        }
    }

    for entity in to_despawn.iter() {
        #[allow(unused_must_use)]
        {
            sim.ecs.despawn(*entity);
        }
    }

    for (entity, damage) in to_damage.iter() {
        damage_entity(sim, *entity, *damage);
    }
}

fn damage_entity(sim: &mut SimState, e_id: Entity, dmg: FixF) {
    let mut hp_comp = sim.ecs.get_mut::<HealthComp>(e_id).unwrap();
    hp_comp.damage(&dmg);
}

#[cfg(test)]
mod shooting_tests {
    use crate::sim_rend_message::*;
    use crate::sim_components::health_comp::HealthComp;
    use crate::sim_components::sim_unit_base_components::IdComp;
    use crate::sim_components::sim_unit_base_components::NextPosComp;
    use crate::sim_components::weapon_comp::WeaponComp;

    use crate::sim_ecs::*;
    use crate::sim_gameloop::first_tick;
    use crate::sim_gameloop::run_single_tick;

    use crate::common::*;
    use crate::sim_components::order_queue_comp::OrderQueueComp;
    use crate::sim_components::sim_unit_base_components::PathComp;
    use crate::sim_components::sim_unit_base_components::PositionComp;
    use crate::sim_components::targeting_comp::TargetComp;
    use crate::sim_components::unitstate_comp::UnitStateComp;
    use crate::sim_fix_math::*;
    use crate::sim_map::Map;

    fn print_components(sim: &mut SimState, e: &UId) {
        type ToQuery<'a> = (
            &'a IdComp,
            &'a UnitStateComp,
            &'a OrderQueueComp,
            &'a TargetComp,
            &'a PathComp,
            &'a PositionComp,
            &'a NextPosComp,
            &'a WeaponComp,
            &'a HealthComp,
        );

        let mut query = sim
            .ecs
            .query_one::<ToQuery>(*sim.res.id_map.get(e).unwrap())
            .unwrap();
        let (id, state, queue, trg, path, pos, nextpos, weapon, hp) = query.get().unwrap();
        println!("\n Tick: {:?} \n", sim.current_tick());
        println!("{:?} \n", id);
        println!("{:?} \n", state);
        println!("{:?} \n", queue);
        println!("{:?} \n", trg);
        println!("{:?} \n", path);
        println!("{:?} \n", pos);
        println!("{:?} \n", nextpos);
        println!("{:?} \n", weapon);
        println!("{:?} \n", hp);

        let ids = sim.ecs.iter().map(|(id, _)| id).collect::<Vec<_>>();

        println!("Existing entities: {:?} \n", ids);
    }

    fn make_game_state() -> (SimState, RendMessenger) {
        let (sim_messenger, rend_messenger) = create_messenger();

        let map = Map::make_test_map();
        let mut sim = SimState::new(map, sim_messenger, 2, 10);

        //run first 2 ticks:
        first_tick(&mut sim);
        rend_messenger.rec();
        run_single_tick(&mut sim);
        sim.end_tick_debug();

        (sim, rend_messenger)
    }

    #[test]
    fn shoot_ground() {
        // cargo test -- --nocapture shoot_ground

        // This is heavily interconnected with other systems.
        // Also this whole test is retarded. I just checked prnt outputs.

        let (mut sim, rend_messenger) = make_game_state();

        let msg0 = RenderMessage::SpawnSmart(0, Pos::from_num(1, 1));
        let msg1 = RenderMessage::SpawnSmart(1, Pos::from_num(1.0, 2.0));
        rend_messenger.send(vec![msg0, msg1]);
        run_single_tick(&mut sim);
        sim.end_tick_debug();

        // selection of units:
        let mut units: [Option<UId>; UNIT_GROUP_CAP] = [None; UNIT_GROUP_CAP];
        units[0] = Some(0);
        units[1] = Some(1);

        let msg = RenderMessage::InputOrder(
            0,
            units,
            UnitOrder::ForceAttack(ObjTarget::Position(Pos::from_num(3, 1))),
        );

        rend_messenger.send(vec![msg]);

        run_single_tick(&mut sim);
        sim.end_tick_debug();
        print_components(&mut sim, &0);

        run_single_tick(&mut sim);
        sim.end_tick_debug();
        print_components(&mut sim, &0);

        run_single_tick(&mut sim);
        sim.end_tick_debug();
        print_components(&mut sim, &0);

        run_single_tick(&mut sim);
        sim.end_tick_debug();
        print_components(&mut sim, &0);

        run_single_tick(&mut sim);
        sim.end_tick_debug();
        print_components(&mut sim, &0);

        run_single_tick(&mut sim);
        sim.end_tick_debug();
        print_components(&mut sim, &0);
    }

    #[test]
    fn shoot_friend() {
        // cargo test -- --nocapture shoot_friend

        let (mut sim, rend_messenger) = make_game_state();

        let msg0 = RenderMessage::SpawnSmart(0, Pos::from_num(1, 1));
        let msg1 = RenderMessage::SpawnSmart(1, Pos::from_num(1.0, 2.0));
        rend_messenger.send(vec![msg0, msg1]);
        run_single_tick(&mut sim);
        sim.end_tick_debug();

        // selection of units:
        let mut units: [Option<UId>; UNIT_GROUP_CAP] = [None; UNIT_GROUP_CAP];
        units[0] = Some(0);
        units[1] = Some(1);

        let msg = RenderMessage::InputOrder(0, units, UnitOrder::ForceAttack(ObjTarget::Entity(1)));

        rend_messenger.send(vec![msg]);

        run_single_tick(&mut sim);
        sim.end_tick_debug();
        print_components(&mut sim, &1);

        run_single_tick(&mut sim);
        sim.end_tick_debug();
        print_components(&mut sim, &1);

        run_single_tick(&mut sim);
        sim.end_tick_debug();
        print_components(&mut sim, &1);

        {
            let hp = sim
                .ecs
                .get::<HealthComp>(*sim.res.id_map.get(&1).unwrap())
                .unwrap();
            assert!(*hp.get_hp() < FixF::from_num(10));
        }
    }

    #[test]
    fn murder_friend() {
        // cargo test -- --nocapture murder_friend

        // Shoot until friend dies.

        let (mut sim, rend_messenger) = make_game_state();

        let msg0 = RenderMessage::SpawnSmart(0, Pos::from_num(1, 1));
        let msg1 = RenderMessage::SpawnSmart(1, Pos::from_num(1.0, 2.0));
        rend_messenger.send(vec![msg0, msg1]);
        run_single_tick(&mut sim);
        sim.end_tick_debug();

        // selection of units:
        let mut units: [Option<UId>; UNIT_GROUP_CAP] = [None; UNIT_GROUP_CAP];
        units[0] = Some(0);
        units[1] = Some(1);

        let msg = RenderMessage::InputOrder(0, units, UnitOrder::ForceAttack(ObjTarget::Entity(1)));

        rend_messenger.send(vec![msg]);

        for _ in 0..40 {
            run_single_tick(&mut sim);
            sim.end_tick_debug();
        }

        {
            assert_eq!(sim.res.id_map.get(&1), None);
        }

        {
            let ids = sim.ecs.iter().map(|(id, _)| id).collect::<Vec<_>>();
            assert_eq!(1, ids.len());
        }
    }
}
