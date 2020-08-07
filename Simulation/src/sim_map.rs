use crate::messenger::EngineMessage;
use crate::sim_fix_math::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MapTile {
    block_path: bool,
    z_level: i32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Map {
    size: (usize, usize),
    tilemap: Vec<MapTile>,
}

impl MapTile {
    fn ground_tile(z_level: i32) -> Self {
        MapTile {
            block_path: false,
            z_level: z_level,
        }
    }

    fn wall_tile(z_level: i32) -> Self {
        MapTile {
            block_path: true,
            z_level: z_level,
        }
    }

    pub fn blocks_path(&self) -> bool {
        self.block_path
    }

    pub fn z_level(&self) -> i32 {
        self.z_level
    }
}

impl Map {
    pub fn empty_map(width: u32, height: u32) -> Self {
        let mut map: Vec<MapTile> = vec![];

        for _ in 0..(width * height) {
            map.push(MapTile::ground_tile(0));
        }

        Map {
            size: (width as usize, height as usize),
            tilemap: map,
        }
    }

    pub fn get_tile(&self, x: u32, y: u32) -> &MapTile {
        &self.tilemap[(x + (self.size.0 as u32 * y)) as usize]
    }

    pub fn get_tile_mut(&mut self, x: u32, y: u32) -> &mut MapTile {
        &mut self.tilemap[(x + (self.size.0 as u32 * y)) as usize]
    }

    fn set_tile(&mut self, x: u32, y: u32, new_tile: MapTile) {
        self.tilemap[(x + (self.size.0 as u32 * y)) as usize] = new_tile;
    }

    pub fn size(&self) -> (usize, usize) {
        self.size.clone()
    }

    pub fn tile_from_pos(&self, pos: Pos) -> &MapTile {
        let pos1 = pos.round();
        self.get_tile(pos.x.to_num::<u32>(), pos.y.to_num::<u32>())
    }

    // Checks wether position is within the map:
    pub fn within(&self, pos: Pos) -> bool {
        (pos.x >= 0) & (pos.y >= 0) & (pos.x < (self.size().0)) & (pos.y < (self.size().1))
    }
    /// Adjusts pos to be within map
    pub fn constrain_pos(&self, pos: &mut Pos) {
        *pos = Pos::from_num(pos.x.max(FixF::from_num(0)), pos.y.max(FixF::from_num(0)));
        *pos = Pos::from_num(
            pos.x.min(FixF::from_num(self.size().0 - 1)),
            pos.y.min(FixF::from_num(self.size().1 - 1)),
        );
    }

    pub fn to_message(&self) -> Vec<EngineMessage> {
        let mut msg: Vec<EngineMessage> = vec![];
        for x in 0..self.size.0 {
            for y in 0..self.size.1 {
                let tile = self.get_tile(x as u32, y as u32).clone();
                let tile_pos = Pos::new(FixF::from_num(x), FixF::from_num(y));
                let tile_msg = EngineMessage::MapTile(tile_pos, tile);
                msg.push(tile_msg);
            }
        }
        msg
    }

    pub fn make_test_map() -> Self {
        //makes 8x8 map with walls and multiple z levels
        let size = 12;
        let mut map = Map::empty_map(size, size);

        //map.set_tile(3, 3, MapTile::wall_tile(1));

        //println!("{:?}", map);

        for y in 0..size {
            'lower: for x in 0..size {
                // Add exterior walls:
                if (y == 0) | (y == size -1) | (x == 0) | (x == size -1) {
                    map.set_tile(x, y, MapTile::wall_tile(1));
                    continue 'lower;
                }

                match (x, y) {
                    (1, 1) => map.set_tile(x, y, MapTile::ground_tile(1)),
                    (1, 2) => map.set_tile(x, y, MapTile::ground_tile(1)),
                    (1, 3) => map.set_tile(x, y, MapTile::ground_tile(1)),
                    (2, 1) => map.set_tile(x, y, MapTile::ground_tile(1)),
                    (2, 2) => map.set_tile(x, y, MapTile::ground_tile(1)),
                    (2, 3) => map.set_tile(x, y, MapTile::ground_tile(1)),
                    (3, 1) => map.set_tile(x, y, MapTile::ground_tile(1)),
                    (3, 2) => map.set_tile(x, y, MapTile::ground_tile(1)),
                    (3, 3) => map.set_tile(x, y, MapTile::ground_tile(1)),

                    (1, 4) => map.set_tile(x, y, MapTile::wall_tile(1)),
                    (2, 4) => map.set_tile(x, y, MapTile::wall_tile(1)),
                    (3, 4) => map.set_tile(x, y, MapTile::wall_tile(1)),
                    (4, 4) => map.set_tile(x, y, MapTile::wall_tile(1)),
                    (4, 3) => map.set_tile(x, y, MapTile::wall_tile(1)),

                    (7, 3) => map.set_tile(x, y, MapTile::wall_tile(0)),
                    (7, 4) => map.set_tile(x, y, MapTile::wall_tile(0)),
                    (6, 4) => map.set_tile(x, y, MapTile::wall_tile(0)),
                    (8, 4) => map.set_tile(x, y, MapTile::wall_tile(0)),
                    (7, 5) => map.set_tile(x, y, MapTile::wall_tile(0)),

                    (9, 9) => map.set_tile(x, y, MapTile::wall_tile(0)),
                    (9, 10) => map.set_tile(x, y, MapTile::wall_tile(0)),

                    (10, 9) => map.set_tile(x, y, MapTile::ground_tile(-1)),
                    (10, 10) => map.set_tile(x, y, MapTile::ground_tile(-1)),

                    _ => {}
                }
            }
        }

        map
    }
}
