use pathfinding::prelude::astar;
use num_traits::identities::Zero;

use crate::messenger::*;
use crate::sim_ecs::SimState;
use crate::sim_fix_math::*;
use crate::sim_map::*;
use crate::sim_unit_base_components::*;
use itertools::Itertools;
use std::collections::VecDeque;

// Note: assumes that destination is pointing to walkable tile.
// Otherwise will fail.

#[derive(Debug)]
struct PathfindingHelper {
    start: Pos,
    goal: Pos,
}

impl PathfindingHelper {
    fn new(start: Pos, goal: Pos) -> Self {
        PathfindingHelper {
            start: start,
            goal: goal,
        }
    }

    fn get_start(&self) -> Pos {
        self.start
    }

    fn get_goal(&self) -> Pos {
        self.goal
    }

    fn check_goal(&self, pos: Pos) -> bool {
        pos == self.goal
    }

    fn heuristic_euclidean(pos1: Pos, pos2: Pos) -> FixF {
        pos1.dist(&pos2) * 10
    }

    // Would use FixF for cost, but it doesn;t implement num_traits::identities::Zero trait. So use u32
    fn adjacent(map: &Map, pos: Pos) -> Vec<(Pos, FixF)> {
        // check if initial cell is within map:
        if !map.within(pos) {
            return vec![];
        }

        // start with adjaent cells that are within the map
        let mut adj: Vec<(Pos, FixF)> = vec![
            pos + Pos::from_num(0, 1),
            pos + Pos::from_num(1, 0),
            pos + Pos::from_num(0, -1),
            pos + Pos::from_num(-1, 0),
        ]
        .iter()
        .filter(|x| map.within(**x))
        .filter(|x| !map.tile_from_pos(**x).blocks_path())
        .map(|x| *x)
        .map(|x| (x, FixF::from_num(10)))
        .collect();

        // Individually check availability of diagonal cells.
        // Only add if they don't block path and have non-blocking adjacent cells.
        // Since map is rectangle, adjacent cells will always be within the map.
        let mut diagonals: Vec<(Pos, FixF)> = vec![
            Pos::from_num(1, 1),
            Pos::from_num(1, -1),
            Pos::from_num(-1, -1),
            Pos::from_num(-1, 1),
        ]
        .iter()
        .filter(|x| map.within(**x + pos))
        .filter(|x| !map.tile_from_pos(**x + pos).blocks_path())
        .filter(|x| {
            let adj1 = pos + Pos::from_num(x.x, FixF::zero());
            let adj2 = pos + Pos::from_num(FixF::zero(), x.y);
            (!map.tile_from_pos(adj1).blocks_path()) &
             (!map.tile_from_pos(adj2).blocks_path()) 
        })
        .map(|x| *x + pos)
        .map(|x| (x, FixF::from_num(14)))
        .collect();

        adj.append(&mut diagonals);

        adj
    }

    // returns vector of points to visit. Includes current position and final position
    fn find_path(map: &Map, start: Pos, goal: Pos) -> VecDeque<Pos> {
        let path_helper = PathfindingHelper::new(start.round(), goal.round());

        let result = astar(
            &path_helper.get_start(),
            |p| PathfindingHelper::adjacent(map, *p),
            |p| PathfindingHelper::heuristic_euclidean(*p, path_helper.get_goal()),
            |p| path_helper.check_goal(*p),
        );

        match result {
            None => return VecDeque::new(),
            Some((path, ..)) => {
                let mut ret: VecDeque<Pos> = VecDeque::from(path);
                //ret.push_front(start);
                ret = ret
                    .iter()
                    .dedup()
                    .map(|x| *x + start.fractional_part())
                    .collect();
                ret.pop_back();    
                ret.pop_front();    
                ret.push_back(goal);
                //ret = ret.iter().dedup().map(|x| *x).collect();

                return ret;
            }
        }
    }

    // Exact pathfinding hsould not be rendered. Remove later.
    fn path_to_message_tmp(id: &IdComp, path: &VecDeque<Pos>) -> EngineMessage {
        let mut path1 = path.clone();
        let end = path1.back();
        match end {
            None => return EngineMessage::None,
            Some(goal) => {
                let mut ret = [*goal; 20];
                for i in 0..path1.len().min(20) {
                    ret[i] = path1.pop_front().unwrap();
                }
                return EngineMessage::ObjPathTmp(*id.get(), ret);
            }
        }
    }
}

/// Should compute the path towards the destination
// Also should generate path message
pub fn sys_pathfinding_astar(sim: &mut SimState) {
    type ToQuery<'a> = (
        &'a IdComp,
        &'a PositionComp,
        &'a DestinationComp,
        &'a mut PathComp,
    );

    //let ecs = &mut sim.ecs;
    //let map = &sim.map;

    //println!("Running pathfinding system");

    'query_loop: for (_, (id, pos, dest, path_comp)) in &mut sim.ecs.query::<ToQuery>() {
        //println!("Calculating path for {:?}", id);
        if dest.last_set() != sim.current_tick() {
            continue 'query_loop;
        }

        if pos.get_pos() == dest.get_dest() {
            continue 'query_loop;
        }

        let path = PathfindingHelper::find_path(&sim.map, *pos.get_pos(), *dest.get_dest());

        //println!("Path for {:?} found: {:?}",id, path);

        // TODO: remove this later sometime:
        {
            let msg = PathfindingHelper::path_to_message_tmp(id, &path);
            sim.send_batch.push(msg);
        }

        path_comp.set(path);
    }
}

#[cfg(test)]
mod pathfinding_tests {

    #[test]
    fn adjacent_test() {
        // run with:
        // cargo test -- --nocapture
        use crate::messenger::*;
        use crate::sim_ecs::*;
        use crate::sim_fix_math::*;
        use crate::sim_map::Map;
        use crate::sim_sys_pathfinding::PathfindingHelper;

        let (sim_messenger, _rend_messenger) = create_messenger();

        let messenger = sim_messenger;
        let map = Map::make_test_map();
        let sim = SimState::new(map, messenger,1, 10);

        let pos1 = Pos::from_num(2, 2);
        let pos2 = Pos::from_num(3, 3);
        let pos3 = Pos::from_num(4, 4);
        let pos4 = Pos::from_num(5, 2);

        println!(
            "{:?} adjacent: {:?}",
            pos1,
            PathfindingHelper::adjacent(&sim.map, pos1)
        );
        println!(
            "{:?} adjacent: {:?}",
            pos2,
            PathfindingHelper::adjacent(&sim.map, pos2)
        );
        println!(
            "{:?} adjacent: {:?}",
            pos3,
            PathfindingHelper::adjacent(&sim.map, pos3)
        );
        println!(
            "{:?} adjacent: {:?}",
            pos4,
            PathfindingHelper::adjacent(&sim.map, pos4)
        );
    }

    #[test]
    fn find_path() {
        use crate::messenger::*;
        use crate::sim_ecs::*;
        use crate::sim_fix_math::*;
        use crate::sim_map::Map;
        use crate::sim_sys_pathfinding::PathfindingHelper;

        let (sim_messenger, _rend_messenger) = create_messenger();

        let messenger = sim_messenger;
        let map = Map::make_test_map();
        let sim = SimState::new(map, messenger,1, 10);

        let start = Pos::from_num(5, 3);
        let goal = Pos::from_num(3, 3);

        let path = PathfindingHelper::find_path(&sim.map, start, goal);

        println!("The path is: \n");
        println!("{:?}", path);
    }
}
