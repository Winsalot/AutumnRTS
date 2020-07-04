use hecs::*;
use crate::sim_fix_math::{Pos, FixF};


/// Unit type name
pub struct TypeNameComp {
	name: String,
}

/// Location component
pub struct PositionComp {
	pos: Pos,
}

// Unit's destination component
pub struct DestinationComp {
	dest: Pos,
}

/// Unit's speed component
pub struct SpeedComponent {
	speed: FixF,
}


pub fn plc_unit(pos: Pos, speed: FixF) -> EntityBuilder {

	let mut unit_builder = EntityBuilder::new();

	unit_builder.add(TypeNameComp::new("placeholder"));
	unit_builder.add(PositionComp::new(pos));
	unit_builder.add(DestinationComp::new(pos));
	unit_builder.add(SpeedComponent::new(speed));

	unit_builder
}

impl TypeNameComp {
	pub fn new(name: &str) -> Self {
		TypeNameComp{name: String::from(name)}
	}
}

impl PositionComp {
	pub fn new(pos: Pos) -> Self {
		PositionComp{pos: pos}
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
		DestinationComp{dest: pos}
	}

	pub fn set_dest(&mut self, pos: Pos) {
		self.dest = pos;
	} 

	pub fn get_dest(&self) -> &Pos{
		&self.dest
	}
}

impl SpeedComponent {
	pub fn new(s: FixF) -> Self {
		SpeedComponent{speed: s}
	}

	pub fn get_speed(&self) -> &FixF {
		&self.speed
	}
}