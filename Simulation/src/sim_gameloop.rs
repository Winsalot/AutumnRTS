use crate::sim_systems::unit_behaviour_ai::sys_unit_behaviour_ai;
use crate::sim_systems::targeting::auto_assign_targets;
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

        // // Run init systems:
        // update_fps_info(&mut sim);
        // sys_init_send_map(&mut sim);
        // send_messages(&mut sim);
        // sim.end_tick();

        update_fps_info(&mut sim);
        first_tick(&mut sim);
        sim.end_tick();

        // Run game loop & systems
        'running: loop {

            update_fps_info(&mut sim);

            if run_single_tick(&mut sim){
                break 'running;
            }

            sim.end_tick();

            // update_fps_info(&mut sim);
            // receive_messages(&mut sim);

            // if input_break_check(&mut sim) {
            //     break 'running;
            // }

            // sys_input_to_order(&mut sim);
            
            // sys_unit_behaviour_ai(&mut sim);
            
            // input_spawn_unit(&mut sim);
            // //input_spawn_structure(&mut sim);

            // sys_input_dest(&mut sim);
            // sys_abilities(&mut sim);

            // sys_pathfinding_astar(&mut sim);
            // sys_pathfinding_smart(&mut sim);

            // sys_collision_pred(&mut sim);

            // sys_set_pos(&mut sim);
            // sys_set_pos_smart(&mut sim);

            // sys_set_next_pos(&mut sim);
            // sys_set_next_pos_smart(&mut sim);

            // auto_assign_targets(&mut sim);
            // clear_inbox(&mut sim);
            // send_messages(&mut sim);
            // sim.end_tick();
        }
    });

    return (sim_handle, rend_messenger);
}

fn first_tick(sim: &mut SimState){
            // Run init systems:
        // update_fps_info(sim);
        sys_init_send_map(sim);
        send_messages(sim);
        // sim.end_tick();
}

fn run_single_tick(sim: &mut SimState) -> bool {

            // update_fps_info(sim);
            receive_messages(sim);

            if input_break_check(sim) {
                return true;
            }

            sys_input_to_order(sim);
            
            sys_unit_behaviour_ai(sim);
            
            input_spawn_unit(sim);
            input_spawn_smart_unit(sim);
            //input_spawn_structure(&mut sim);

            sys_input_dest(sim);
            sys_abilities(sim);

            sys_pathfinding_astar(sim);
            sys_pathfinding_smart(sim);

            sys_collision_pred(sim);

            sys_set_pos(sim);
            sys_set_pos_smart(sim);

            sys_set_next_pos(sim);
            sys_set_next_pos_smart(sim);

            auto_assign_targets(sim);
            clear_inbox(sim);
            send_messages(sim);
            // sim.end_tick();

            false
}

#[cfg(test)]
mod interation_tests {

    #[test]





fn smart_movement_test(){
        // run with:
        // cargo test -- --nocapture smart_movement_test

        // Basically idea is to create a single smart character, give movement order and print whole gamestate every frame.

        use crate::messenger::*;
        use crate::sim_ecs::*;
        use crate::sim_gameloop::*;
        use crate::common::*;   
        use crate::sim_fix_math::*;        
        use crate::sim_map::Map;
        use crate::sim_components::order_queue_comp::OrderQueueComp;
        use crate::sim_components::unitstate_comp::UnitStateComp;
        use crate::sim_components::sim_unit_base_components::PathComp;
        use crate::sim_components::sim_unit_base_components::PositionComp;
        use crate::sim_components::targeting_comp::TargetComp;


        fn print_components(sim: &mut SimState, e: &UId) {
            type ToQuery<'a> = (
                &'a UnitStateComp,
                &'a OrderQueueComp,
                &'a TargetComp,
                &'a PathComp,
                &'a PositionComp,
            );

            let mut query = sim.ecs.query_one::<ToQuery>(*sim.res.id_map.get(e).unwrap()).unwrap();
            let (state, queue, trg, path, pos) = query.get().unwrap();
            println!("{:?} \n", state);
            println!("{:?} \n", queue);
            println!("{:?} \n", trg);
            println!("{:?} \n", path);
            println!("{:?} \n", pos);

        }


        let (sim_messenger, rend_messenger) = create_messenger();

        let map = Map::make_test_map();
        let mut sim = SimState::new(map, sim_messenger, 1, 10);

        //run first 2 ticks:
        first_tick(&mut sim);
        rend_messenger.rec();
        run_single_tick(&mut sim);

        let msg0 = RenderMessage::SpawnSmart(0, Pos::from_num(1,1));
        //let msg1 = RenderMessage::Spawn(0, Pos::from_num(2,2));
        //rend_messenger.send(vec![msg0, msg1]);
        rend_messenger.send(vec![msg0]);

        run_single_tick(&mut sim);
        //run_single_tick(&mut sim);

        println!("MESSAGES: {:?}", rend_messenger.rec());
        print_components(&mut sim, &0);


        let mut units: [Option<UId>; UNIT_GROUP_CAP]= [None; UNIT_GROUP_CAP];
        units[0] = Some(0);
        let msg = RenderMessage::InputOrder(
            0,
            units, 
            UnitOrder::MoveTo(Pos::from_num(4,1)),
            );

        rend_messenger.send(vec![msg]);

        run_single_tick(&mut sim);

        println!("MESSAGES: {:?}", rend_messenger.rec());
        print_components(&mut sim, &0);

        run_single_tick(&mut sim);

        print_components(&mut sim, &0);

        run_single_tick(&mut sim);
        // run_single_tick(&mut sim);
        // run_single_tick(&mut sim);

        print_components(&mut sim, &0);
        println!("MESSAGES: {:?}", rend_messenger.rec());

    }

}