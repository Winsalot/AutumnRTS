use crate::sim_fix_math::{Pos, FixF};
use crate::fpscounter::FpsCounter;
//use snowflake::ProcessUniqueId;
use crate::messenger::{EngineMessage, RenderMessage};
use crate::sim_object::*;
//use fixed::prelude::*;

pub struct Simulation{
	pub obj_id_counter: ObjectID,
	objects: Vec<Object>,
	pub fps_counter: FpsCounter,
}

impl Simulation {
	pub fn new(fps_n_avg: usize) -> Self{
		Simulation{
			obj_id_counter: 0,
			objects: Vec::new(),
			fps_counter: FpsCounter::new(fps_n_avg)
		}
	}

	fn spawn(&mut self, pos: &Pos) -> EngineMessage{

		let new_object = Object::new(
			&mut self.obj_id_counter,
			*pos, 
			FixF::from_num(1)
			);

		let ret_msg = EngineMessage::ObjSpawn(*new_object.get_id(), *pos);

		self.objects.push(new_object);

		return ret_msg;
	}

	fn obj_query(&mut self, id: &ObjectID) -> Option<&mut Object>{
		for i in 0..self.objects.len(){
			if self.objects[i].get_id() == id {
				return Some(&mut self.objects[i]);
			}
		}
		None
	}

	pub fn process_input(&mut self, 
		rend_msg: &Vec<RenderMessage>, 
		break_loop: &mut bool) ->
		Vec<EngineMessage>
	{

		let mut ret_msg: Vec<EngineMessage> = vec![];

		for msg in rend_msg {
			match msg {
	        	RenderMessage::Destination(id, pos) => {
	        		if let Some(obj) = self.obj_query(id){
	        			ret_msg.push(obj.set_dest(*pos));
	        		}
	        	},
	        	RenderMessage::Spawn(pos) => {
	        		ret_msg.push(self.spawn(pos));
	        	}
	        	RenderMessage::Break => {*break_loop = true;},
	        	_ => {},
	        }
	    }
	    return ret_msg;
	}

	pub fn add_object(&mut self, obj: Object){
		self.objects.push(obj);
	}

	pub fn update_full(&mut self,
		rend_msg: Vec<RenderMessage>) -> (bool, Vec<EngineMessage>)
	{

		//let mut sim_msg = vec![EngineMessage::None]; 
		let mut ret_msg: Vec<EngineMessage> = vec![];
		let mut break_loop = false;

		ret_msg.append(
			&mut self.process_input(&rend_msg, &mut break_loop)
			);

		for i in 0..self.objects.len(){
			ret_msg.append(
				&mut self.objects[i].update()
				);
		}

/*
		for i in 0..self.objects.len(){
			let msg = self.objects[i].generate_message();
			sim_msg.push(msg);
		}
*/

		let fps_info = self.fps_counter.get_fps_simple();
		ret_msg.push(EngineMessage::Fps(fps_info));

		(break_loop, ret_msg)

	}
}






/*
#[derive(Debug)]
pub struct Object {
	id: ObjectID,
	xy: Pos,
	dest: Pos,
	speed: FixF,
}

impl Object {
	pub fn new(xy: Pos, s: FixF) -> Object {
		//let xy = Pos::new(x,y);
		Object{id: ProcessUniqueId::new(),
			xy: xy,
			dest: xy,
			speed:s}
	}

	pub fn set_dest(&mut self, xy: Pos){
		self.dest = xy;
	}

	pub fn update(&mut self){
		if self.xy == self.dest {
			return;
		}
		//let v = ((self.x - self.x1) as f32, (self.y - self.y1) as f32);
		//let v_abs = (v.0.powi(2) + v.1.powi(2)).sqrt();
		let distance = Pos::dist(&self.xy, &self.dest);
		if distance == 0 {
			// if distance is too small, move here without taking any steps
			self.xy = self.dest;
			return;
		}
		let dx = (self.xy - self.dest) / distance;
		self.xy = self.xy - dx * self.speed.min(distance);
	}

	pub fn generate_message(&self) -> EngineMessage{
		EngineMessage::Object(self.id.clone(), self.xy.clone(), self.dest.clone())
	}

	
}
*/