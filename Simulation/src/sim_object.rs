// Will be reworked when/if I move to ECS
use crate::messenger::EngineMessage;
use crate::sim_fix_math::{Pos, FixF};



pub type ObjectID = u64;


#[derive(Debug)]
pub struct Object {
	id: ObjectID,
	xy: Pos,
	dest: Pos,
	speed: FixF,
}

impl Object {
	pub fn new(sim_id: &mut ObjectID, xy: Pos, s: FixF) -> Object {
		//let xy = Pos::new(x,y);
		let id = sim_id.clone();
		*sim_id += 1;
		Object{
			id: id,
			xy: xy,
			dest: xy,
			speed:s}
	}

	pub fn set_dest(&mut self, xy: Pos) -> EngineMessage {
		self.dest = xy;
		EngineMessage::ObjDest(self.id, self.dest)
	}

	pub fn get_id(&self) -> &ObjectID {
		&self.id
	}

	pub fn update(&mut self) -> Vec<EngineMessage> {

		let mut ret_msg: Vec<EngineMessage> = vec![];

		if self.xy == self.dest {
			return ret_msg;
		}

		let distance = Pos::dist(&self.xy, &self.dest);
		if distance == 0 {
			// if distance is too small, move here without taking any steps
			self.xy = self.dest;
			ret_msg.push(EngineMessage::ObjMove(self.id, self.xy));
			return ret_msg;
		}

		let dx = (self.xy - self.dest) / distance;
		self.xy = self.xy - dx * self.speed.min(distance);

		ret_msg.push(EngineMessage::ObjMove(self.id, self.xy));
		return ret_msg;
	}

/*
	pub fn generate_message(&self) -> EngineMessage{
		EngineMessage::Object(self.id.clone(), self.xy.clone(), self.dest.clone())
	}
*/
	
}
