//use crate::sim_map::StructureMemory;
use crate::common::*;
use crate::fpscounter::*;
use crate::messenger::*;
use crate::sim_map::Map;
use crate::sim_player_alliances::*;
use hecs::*;

// Basically a struct that contains ECS system and system state.
pub struct SimState {
    pub ecs: World,
    pub messenger: SimMessenger,
    pub inbox: Vec<RenderMessage>,
    pub send_batch: Vec<EngineMessage>,
    pub fps_counter: FpsCounter,
    pub fps_limit: u32,
    pub break_loop: bool,
    current_tick: TickNum,
    pub id_counter: UId,
    pub map: Map,
    players: PlayerList,
    //map_mem: StructureMemory,
}

impl SimState {
    pub fn new(game_map: Map, sim_messenger: SimMessenger, n_players: u32, fps_limit: u32) -> Self {
        SimState {
            ecs: World::new(),
            messenger: sim_messenger,
            inbox: vec![],
            send_batch: vec![],
            fps_counter: FpsCounter::new(10),
            fps_limit: fps_limit,
            break_loop: false,
            current_tick: 0,
            id_counter: 0,
            map: game_map,
            players: PlayerList::ffa(n_players),
        }
    }

    pub fn current_tick(&self) -> TickNum {
        self.current_tick
    }

    pub fn end_tick(&mut self) {
        self.current_tick += 1;
        self.fps_counter.limit_fps(self.fps_limit);
        self.fps_counter.tick();
    }
}
