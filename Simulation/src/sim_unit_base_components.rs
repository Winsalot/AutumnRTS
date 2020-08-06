use hecs::*;
use std::collections::VecDeque;

use crate::sim_fix_math::*;

/// Unit type name
pub struct TypeNameComp {
    name: String,
}

/// Unit id
/// Reason is that hecs ecs reuses id's, and
///this could cause some bugs in the future.
///Id component should be perfectly unique in game context
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct IdComp {
    id: u64,
}

/// Location component
#[derive(Debug, PartialEq, Clone)]
pub struct PositionComp {
    pos: Pos,
}

/// Next position component
pub struct NextPosComp {
    pos: Pos,
}

// Unit's destination component
pub struct DestinationComp {
    dest: Pos,
    updated_on: u64,
}

/// Unit's speed component
pub struct SpeedComponent {
    speed: FixF,
}

// Square hitbox. W,H should be treadted as radius
pub struct CollComp {
    r: FixF,
}

// pathfinding pomponent. Holds positions that unit should walk to.
#[derive(Debug, PartialEq, Clone)]
pub struct PathComp {
    positions: VecDeque<Pos>,
}

pub fn plc_unit(pos: Pos, speed: FixF, id_counter: &mut u64) -> EntityBuilder {
    let mut unit_builder = EntityBuilder::new();

    unit_builder.add(TypeNameComp::new("placeholder"));
    unit_builder.add(PositionComp::new(pos));
    unit_builder.add(NextPosComp::new(pos));
    unit_builder.add(DestinationComp::new(pos));
    unit_builder.add(SpeedComponent::new(speed));
    unit_builder.add(CollComp::new(FixF::from_num(0.5)));
    unit_builder.add(IdComp::new(id_counter));
    unit_builder.add(PathComp::new());

    unit_builder
}

impl TypeNameComp {
    pub fn new(name: &str) -> Self {
        TypeNameComp {
            name: String::from(name),
        }
    }
}

impl PositionComp {
    pub fn new(pos: Pos) -> Self {
        PositionComp { pos: pos }
    }

    pub fn set_pos(&mut self, pos: Pos) {
        self.pos = pos;
    }

    pub fn get_pos(&self) -> &Pos {
        &self.pos
    }
}

impl NextPosComp {
    pub fn new(pos: Pos) -> Self {
        NextPosComp { pos: pos }
    }

    pub fn set_pos(&mut self, pos: Pos) {
        self.pos = pos;
    }

    pub fn get_pos(&self) -> &Pos {
        &self.pos
    }
}

impl DestinationComp {
    pub fn new(pos: Pos) -> Self {
        DestinationComp {
            dest: pos,
            updated_on: 0,
        }
    }

    pub fn set_dest(&mut self, pos: Pos, tick: u64) {
        self.dest = pos;
        self.updated_on = tick;
    }

    pub fn get_dest(&self) -> &Pos {
        &self.dest
    }

    pub fn last_set(&self) -> u64 {
        self.updated_on
    }
}

impl SpeedComponent {
    pub fn new(s: FixF) -> Self {
        SpeedComponent { speed: s }
    }

    pub fn get_speed(&self) -> &FixF {
        &self.speed
    }
}

impl CollComp {
    pub fn new(radius: FixF) -> Self {
        CollComp { r: radius }
    }

    pub fn get_r(&self) -> &FixF {
        &self.r
    }
}

impl IdComp {
    pub fn new(id_counter: &mut u64) -> Self {
        let id = std::mem::replace(id_counter, *id_counter + 1);

        IdComp { id: id }
    }

    pub fn get(&self) -> &u64 {
        &self.id
    }

    pub fn from(id: u64) -> Self {
        IdComp { id: id }
    }
}

impl PathComp {
    pub fn new() -> Self {
        PathComp {
            positions: VecDeque::new(),
        }
    }

    pub fn get(&self) -> &VecDeque<Pos> {
        &self.positions
    }

    pub fn get_mut(&mut self) -> &mut VecDeque<Pos> {
        &mut self.positions
    }

    pub fn set(&mut self, path: VecDeque<Pos>) {
        self.positions = path;
    }

    pub fn from_vec(&mut self, path: Vec<Pos>) {
        self.positions = VecDeque::from(path);
    }

    pub fn is_empty(&self) -> bool {
        self.positions.len() == 0
    }

    pub fn get_next_pos(&mut self, current_pos: &Pos) -> Option<&Pos> {
        match self.positions.front() {
            None => return None,
            Some(front) => {
                if front == current_pos {
                    self.positions.pop_front();
                }
                return self.positions.front();
            }
        }
    }
}
