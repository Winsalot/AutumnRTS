//use crate::sim_map::StructureMemory;
use crate::common::*;
use crate::fpscounter::*;
use crate::sim_rend_message::*;
use crate::sim_map::Map;
use crate::sim_player_alliances::*;
//use hecs::*;
use hecs::{Entity, World};
use std::collections::HashMap;

// Basically a struct that contains ECS system and system state.
pub struct SimState {
    pub ecs: World,
    pub messenger: SimMessenger,
    pub break_loop: bool,
    pub map: Map,
    pub res: SimResources,
}

impl SimState {
    pub fn new(game_map: Map, sim_messenger: SimMessenger, n_players: u32, fps_limit: u32) -> Self {
        SimState {
            ecs: World::new(),
            messenger: sim_messenger,
            break_loop: false,
            map: game_map,
            res: SimResources::new(n_players, fps_limit),
        }
    }

    pub fn current_tick(&self) -> TickNum {
        self.res.current_tick
    }

    pub fn end_tick(&mut self) {
        self.res.current_tick += 1;
        self.res.fps_counter.limit_fps(self.res.fps_limit);
        self.res.fps_counter.tick();
    }

    pub fn end_tick_debug(&mut self) {
        // instant tick, no sleep.
        self.res.current_tick += 1;
        self.res.fps_counter.tick();
    }
}

pub struct SimResources {
    pub id_map: HashMap<UId, Entity>, // Dead entities should be removed
    pub id_counter: UId,
    pub players: PlayerList,
    pub current_tick: TickNum,
    pub inbox: Vec<RenderMessage>,
    pub send_batch: Vec<SimMsg>,
    pub fps_counter: FpsCounter,
    pub fps_limit: u32,
}

impl SimResources {
    pub fn new(n_players: u32, fps_limit: u32) -> Self {
        SimResources {
            id_map: HashMap::new(),
            id_counter: 0,
            current_tick: 0,
            players: PlayerList::ffa(n_players as PId),
            inbox: vec![],
            send_batch: vec![],
            fps_counter: FpsCounter::new(10),
            fps_limit: fps_limit,
        }
    }
}
