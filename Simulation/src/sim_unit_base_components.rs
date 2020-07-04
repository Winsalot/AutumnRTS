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

// Square hitbox. W,H should be treadted as radius
pub struct CollisionComponent{
	h: FixF,
	w: FixF,
}


pub fn plc_unit(pos: Pos, speed: FixF) -> EntityBuilder {

	let mut unit_builder = EntityBuilder::new();

	unit_builder.add(TypeNameComp::new("placeholder"));
	unit_builder.add(PositionComp::new(pos));
	unit_builder.add(DestinationComp::new(pos));
	unit_builder.add(SpeedComponent::new(speed));
	unit_builder.add(CollisionComponent::new(FixF::from_num(0.5)));

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

impl CollisionComponent{
	pub fn new(radius: FixF) -> Self {
		CollisionComponent{
			w: radius,
			h: radius,
		}
	}

	pub fn get_h(&self) -> &FixF {
		&self.h
	}

	pub fn get_w(&self) -> &FixF {
		&self.w
	}
}

pub fn is_colliding(
	p1: &PositionComp,
	c1: &CollisionComponent,
	p2: &PositionComp,
	c2: &CollisionComponent
	) -> bool {

	let pos1 = p1.get_pos();
	let pos2 = p2.get_pos();

	let dx = (pos1.x - pos2.x).abs();
	let dy = (pos1.y - pos2.y).abs();

	if (dx<(c1.get_w() + c2.get_w())) | (dy < (c1.get_h() + c2.get_h())) {
		return true;
	}
	false
}