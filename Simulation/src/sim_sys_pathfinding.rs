use pathfinding::prelude::astar;

use crate::sim_unit_base_components::*;
use crate::sim_map::*;
use crate::sim_ecs::SimState;
use crate::sim_fix_math::*;
use std::collections::VecDeque;

// Note: assumes that destination is pointing to walkable tile.
// Otherwise will fail.

#[derive(Debug)]
struct PathfindingHelper {
	start: Pos,
	goal: Pos,
}

impl PathfindingHelper {
	fn new(start: Pos, goal: Pos) -> Self{
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
		pos1.dist(&pos2)*10
	}


// Would use FixF for cost, but it doesn;t implement num_traits::identities::Zero trait. So use u32
	fn adjacent(map: &Map, pos: Pos) -> Vec<(Pos,FixF)> {

		// check if initial cell is within map:
		if !map.within(pos){
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
			.filter(|x| {
				let adj1 = pos + Pos::from_num(x.x, zero());
				let adj2 = pos + Pos::from_num(zero(), x.y);
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
	fn find_path(map: &Map, start: Pos, goal: Pos) -> Option<VecDeque<Pos>> {

		let path_helper = PathfindingHelper::new(start.round(), goal.round());

		let result = astar(
	        &path_helper.get_start(),
	        |p| PathfindingHelper::adjacent(map, *p),
	        |p| PathfindingHelper::heuristic_euclidean(*p, path_helper.get_goal()),
	        |p| path_helper.check_goal(*p),
	    );

	    match result{
	    	None => return None,
	    	Some((path, ..)) => {
	    		let mut ret: VecDeque<Pos> = VecDeque::from(path);
	    		ret.push_back(goal);
	    		ret.push_front(start);

	    		return Some(ret);
	    	}
	    }
	}
}





#[cfg(test)]
mod pathfinding_tests {

	#[test]
	fn adjacent_test(){
	// run with:
	// cargo test -- --nocapture
		use crate::sim_ecs::*;
		use crate::messenger::*;
		use crate::sim_map::Map;
		use crate::sim_sys_pathfinding::PathfindingHelper;
		use crate::sim_fix_math::*;



		let (sim_messenger, _rend_messenger) = create_messenger();

		let messenger = sim_messenger;
		let map = Map::make_test_map();
		let sim = SimState::new(map, messenger, 10);

		let pos1 = Pos::from_num(2,2);
		let pos2 = Pos::from_num(3,3);
		let pos3 = Pos::from_num(4,4);
		let pos4 = Pos::from_num(5,2);

		println!("{:?} adjacent: {:?}",pos1, PathfindingHelper::adjacent(&sim.map, pos1));
		println!("{:?} adjacent: {:?}",pos2, PathfindingHelper::adjacent(&sim.map, pos2));
		println!("{:?} adjacent: {:?}",pos3, PathfindingHelper::adjacent(&sim.map, pos3));
		println!("{:?} adjacent: {:?}",pos4, PathfindingHelper::adjacent(&sim.map, pos4));
		
	}

	#[test]
	fn find_path(){
		use crate::sim_ecs::*;
		use crate::messenger::*;
		use crate::sim_map::Map;
		use crate::sim_sys_pathfinding::PathfindingHelper;
		use crate::sim_fix_math::*;



		let (sim_messenger, _rend_messenger) = create_messenger();

		let messenger = sim_messenger;
		let map = Map::make_test_map();
		let sim = SimState::new(map, messenger, 10);

		let start = Pos::from_num(2.3, 5.1);
		let goal = Pos::from_num(1.7, 2.7);

		let path = PathfindingHelper::find_path(&sim.map, start, goal);

		println!("The path is: \n");
		println!("{:?}", path);
	}
}