use pathfinding::prelude::astar;

use crate::sim_unit_base_components::*;
use crate::sim_map::*;
use crate::sim_ecs::SimState;
use crate::sim_fix_math::*;

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
		pos1.dist(&pos2)
	}

	fn adjacent(map: &Map, pos: Pos) -> Vec<Pos> {

		// check if initial cell is within map:
		if !map.within(pos){
			return vec![];
		}

		// start with adjaent cells that are within the map
		let mut adj: Vec<Pos> = vec![
			pos + Pos::from_num(0, 1),
			pos + Pos::from_num(1, 0),
			pos + Pos::from_num(0, -1),
			pos + Pos::from_num(-1, 0),
			]
			.iter()
			.filter(|x| map.within(**x))
			.filter(|x| !map.tile_from_pos(**x).blocks_path())
			.map(|x| *x)
			.collect();

		// Individually check availability of diagonal cells.
		// Only add if they don't block path and have non-blocking adjacent cells.
		// Since map is rectangle, adjacent cells will always be within the map.
		let mut diagonals: Vec<Pos> = vec![
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
			.collect();



		adj.append(&mut diagonals);

		adj
	}
}





/*

// TODO: rewrite this one.
fn run_astar(map: &mut Map) {
    map.reset_path();

    let result = astar(
        map.get_start(),
        |p| map.adjacent(p),
        |p| p.dist_e(map.get_goal()),
        |p| p == map.get_goal(),
    );
    //println!("{:?}", result);
    //println!("{:?}",result.expect("no path found").1);
    if let Some((tiles_path, _)) = result {
        map.mark_path(tiles_path);
    }
}

*/

#[cfg(test)]
mod pathfinding_tests {
	// run with:
	// cargo test --release -- --nocapture

	#[test]
	fn adjacent_test(){
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
}