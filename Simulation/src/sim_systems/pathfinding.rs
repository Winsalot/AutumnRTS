// use crate::common::SimMsg::StateChange;
// use crate::common::SimStateChng::ObjPathTmp;
use crate::sim_components::order_queue_comp::OrderQueueComp;
use crate::sim_components::sim_unit_base_components::IdComp;
use crate::sim_components::targeting_comp::TargetComp;
use crate::sim_components::unitstate_comp::UnitStateComp;
use crate::sim_rend_message::messages_sim::SimMsg;
use crate::sim_rend_message::messages_sim::SimMsg::StateChange;
use crate::sim_rend_message::messages_sim::SimStateChng::ObjPathTmp;
use crate::sim_systems::targeting::target_to_pos;

use crate::sim_components::sim_unit_base_components::DestinationComp;
use crate::sim_components::sim_unit_base_components::PathComp;
use crate::sim_components::sim_unit_base_components::PositionComp;
use num_traits::identities::Zero;
use pathfinding::prelude::astar;

use crate::sim_ecs::SimState;
use crate::sim_fix_math::*;
use crate::sim_map::*;
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
        .filter(|x| !map.map_mem.get_blocked().contains(x))
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
        .filter(|x| !map.map_mem.get_blocked().contains(&(**x + pos)))
        .filter(|x| {
            let adj1 = pos + Pos::from_num(x.x, FixF::zero());
            let adj2 = pos + Pos::from_num(FixF::zero(), x.y);
            (!map.tile_from_pos(adj1).blocks_path()) & (!map.tile_from_pos(adj2).blocks_path())
        })
        .filter(|x| {
            let adj1 = pos + Pos::from_num(x.x, FixF::zero());
            let adj2 = pos + Pos::from_num(FixF::zero(), x.y);
            (!map.map_mem.get_blocked().contains(&adj1))
                & (!map.map_mem.get_blocked().contains(&adj2))
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
    // fn path_to_message_tmp(id: &IdComp, path: &VecDeque<Pos>) -> Vec<EngineMessage> {
    fn path_to_message_tmp(id: &IdComp, path: &VecDeque<Pos>) -> Vec<SimMsg> {
        let mut path1 = path.clone();
        let end = path1.back();
        match end {
            //None => return EngineMessage::None,
            None => return vec![],
            Some(goal) => {
                let mut ret = [*goal; 20];
                for i in 0..path1.len().min(20) {
                    ret[i] = path1.pop_front().unwrap();
                }
                // return vec![EngineMessage::ObjPathTmp(*id.get(), ret)];
                return vec![StateChange(ObjPathTmp(*id.get(), ret))];
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

    'query_loop: for (_, (id, pos, dest, path_comp)) in
        sim.ecs.query::<ToQuery>().without::<UnitStateComp>().iter()
    {
        //println!("Calculating path for {:?}", id);
        if dest.last_set() != sim.current_tick() {
            continue 'query_loop;
        }

        if pos.get_pos() == dest.get_dest() {
            continue 'query_loop;
        }

        let path = PathfindingHelper::find_path(&sim.map, *pos.get_pos(), *dest.get_dest());

        //println!("Path for {:?} found: {:?}",id, path);

        // TODO: remove this later sometime. Player doesnt need to see pathfinding details.
        {
            let mut msg = PathfindingHelper::path_to_message_tmp(id, &path);
            sim.res.send_batch.append(&mut msg);
        }

        path_comp.set(path);
    }
}

pub fn sys_pathfinding_smart(sim: &mut SimState) {
    // Finds path only once
    // Checks unit state for executiuon.

    type ToQuery<'a> = (
        &'a IdComp,
        &'a PositionComp,
        // &'a DestinationComp,
        &'a TargetComp,
        &'a mut OrderQueueComp,
        &'a mut UnitStateComp,
        &'a mut PathComp,
    );

    'query_loop: for (_, (id, pos, target, _order_queue, state, path_comp)) in
        &mut sim.ecs.query::<ToQuery>()
    {
        // Check if pathfinding can be run on this tick
        if !state.pathfind() {
            continue 'query_loop;
        }

        // let dest = target.get_trg_pos().unwrap();
        let dest = target_to_pos(sim, target.get_trg()).unwrap();

        // If current path ends at destination then don't recalculate.
        // This check is redundant, but whatever.
        if let Some(last_node) = path_comp.get_path().back() {
            if last_node == &dest {
                continue 'query_loop;
            }
        }

        let path = PathfindingHelper::find_path(&sim.map, *pos.get_pos(), dest);

        //println!("Path for {:?} found: {:?}",id, path);

        // TODO: remove this later sometime. Player doesnt need to see pathfinding details.
        {
            let mut msg = PathfindingHelper::path_to_message_tmp(id, &path);
            sim.res.send_batch.append(&mut msg);
        }

        //state.pathfind_finished();

        path_comp.set(path);
    }
}
