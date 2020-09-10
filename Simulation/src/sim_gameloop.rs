use crate::sim_systems::abilities::sys_abilities_smart;
use crate::sim_systems::fire_weapons::sys_fire_weapons;
use crate::sim_systems::projectiles::sys_projectiles;
use crate::sim_systems::unit_behaviour_ai::sys_unit_behaviour_ai;
// use crate::sim_systems::targeting::auto_assign_targets;
use crate::messenger::*;
// use crate::sim_systems::abilities::sys_abilities;
use crate::sim_systems::input_systems::*;
use crate::sim_systems::movement::*;
use crate::sim_systems::pathfinding::*;
//use crate::sim_sys_pathfinding::sys_pathfinding_astar;

use crate::sim_ecs::*;
use crate::sim_map::Map;

use std::thread::JoinHandle;

// Starts game loop and returns all the control handles

pub fn start_loop(n_players: u32, fps: u32) -> (JoinHandle<()>, RendMessenger) {
    let (sim_messenger, rend_messenger) = create_messenger();

    let sim_handle = std::thread::spawn(move || {
        // Initialise kay parts of the simulation:
        let messenger = sim_messenger;
        let map = Map::make_test_map();
        let mut sim = SimState::new(map, messenger, n_players, fps);

        update_fps_info(&mut sim);
        first_tick(&mut sim);
        sim.end_tick();

        // Run game loop & systems
        'running: loop {
            update_fps_info(&mut sim);

            if run_single_tick(&mut sim) {
                break 'running;
            }

            sim.end_tick();
        }
    });

    return (sim_handle, rend_messenger);
}

pub fn first_tick(sim: &mut SimState) {
    sys_init_send_map(sim);
    send_messages(sim);
}

pub fn run_single_tick(sim: &mut SimState) -> bool {
    receive_messages(sim);

    if input_break_check(sim) {
        return true;
    }

    sys_input_to_order(sim);

    sys_unit_behaviour_ai(sim);

    sys_fire_weapons(sim);
    sys_projectiles(sim);

    input_spawn_unit(sim);
    input_spawn_smart_unit(sim);

    sys_input_dest(sim);
    // sys_abilities(sim);

    sys_abilities_smart(sim);

    sys_pathfinding_astar(sim);
    sys_pathfinding_smart(sim);

    sys_set_next_pos(sim);
    sys_set_next_pos_smart(sim);

    sys_collision_pred(sim);

    sys_set_pos(sim);
    sys_set_pos_smart(sim);

    clear_inbox(sim);
    send_messages(sim);

    false
}
