use crate::messenger::*;
use crate::sim_systems::abilities::sys_abilities;
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

        // Run init systems:
        update_fps_info(&mut sim);
        sys_init_send_map(&mut sim);
        send_messages(&mut sim);
        sim.end_tick();

        // Run game loop & systems
        'running: loop {
            update_fps_info(&mut sim);
            receive_messages(&mut sim);

            if input_break_check(&mut sim) {
                break 'running;
            }

            input_spawn_unit(&mut sim);
            //input_spawn_structure(&mut sim);
            sys_input_dest(&mut sim);
            sys_abilities(&mut sim);
            sys_pathfinding_astar(&mut sim);
            sys_collision_pred(&mut sim);
            sys_set_pos(&mut sim);
            sys_set_next_pos(&mut sim);
            clear_inbox(&mut sim);
            send_messages(&mut sim);
            sim.end_tick();
        }
    });

    return (sim_handle, rend_messenger);
}
