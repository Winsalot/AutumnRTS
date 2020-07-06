use crate::sim_fix_math::*;


pub struct MapTile {
	block_path: bool,
	z_level: i32
}
pub struct Map {
	tilemap: Vec<MapTile>
}

impl MapTile {
	fn ground_tile(z_level: i32) -> Self {
		MapTile{
			block_path: false,
			z_level: z_level,
		}
	}

	fn wall_tile(z_level: i32) -> Self {
		MapTile{
			block_path: true,
			z_level: z_level,
		}
	}
}

impl Map {
	pub fn empty_map(width: u32, height: u32) -> Self {

		let mut map: Vec<MapTile> = vec![];

		for _ in 0..(width*height) {
			map.push(MapTile::ground_tile(0));
		}

		Map{
			tilemap: map,
		}
	}

	pub fn get_tile(&mut self, x: u32, y: u32) -> &mut MapTile {
		&mut self.tilemap[(x+(x*y)) as usize]
	}

	fn set_tile(&mut self, x: u32, y: u32, new_tile: MapTile) {
		self.tilemap[(x+(x*y)) as usize] = new_tile;
	}

	pub fn make_test_map() -> Self {
		//makes 8x8 map with walls and multiple z levels
		let mut map = Map::empty_map(8,8);

		for y in 0..8 {
			'lower: for x in 0..8 {
				// Add exterior walls:
				if (y == 0) | (y == 8) | (x == 0) | (x == 8) {
					map.set_tile(x, y, MapTile::wall_tile(1));
					continue 'lower;
				}

				match (x, y) {
					(1,1) => map.set_tile(x, y, MapTile::ground_tile(1)),
					(1,2) => map.set_tile(x, y, MapTile::ground_tile(1)),
					(1,3) => map.set_tile(x, y, MapTile::ground_tile(1)),
					(2,1) => map.set_tile(x, y, MapTile::ground_tile(1)),
					(2,2) => map.set_tile(x, y, MapTile::ground_tile(1)),
					(2,3) => map.set_tile(x, y, MapTile::ground_tile(1)),
					(3,1) => map.set_tile(x, y, MapTile::ground_tile(1)),
					(3,2) => map.set_tile(x, y, MapTile::ground_tile(1)),
					(3,3) => map.set_tile(x, y, MapTile::ground_tile(1)),

					(1,4) => map.set_tile(x, y, MapTile::wall_tile(1)),
					(2,4) => map.set_tile(x, y, MapTile::wall_tile(1)),
					(3,4) => map.set_tile(x, y, MapTile::wall_tile(1)),
					(4,4) => map.set_tile(x, y, MapTile::wall_tile(1)),
					(4,3) => map.set_tile(x, y, MapTile::wall_tile(1)),
					
					_ => {},
				}

			}
		}

		map
	}

}