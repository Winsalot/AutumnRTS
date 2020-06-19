use gdnative::*;
use crate::sim_fix_math::{Pos, FixF};
use crate::messenger::*;
use crate::sim_object::*;

// this small module offers functions to process Simulation messages 
// and prepaes them for godot


pub fn pos_to_vector2(pos: Pos) -> Vector2 {
	let (x, y): (f32, f32) = (
		pos.x.to_num::<f32>(),
		pos.y.to_num::<f32>(),
		);
	Vector2::new(x, y)
}

pub fn vector2_to_pos(vec: Vector2) -> Pos {
	Pos::from_num(vec.x, vec.y)
}

// return Variant- ready format
pub fn inbox_drain_spawn(inbox: &mut Vec<EngineMessage>) -> 
	Vec<(ObjectID, f32, f32)> 
	{
	let (target, rest): (Vec<EngineMessage>, Vec<EngineMessage>) = inbox
		.clone()
		.iter()
		.partition(|&msg| match msg {
			EngineMessage::ObjSpawn(..) => true,
			_ => false,
		});

	*inbox = rest;

	// turn messages into tuples:
	let mut ret: Vec<(ObjectID, f32, f32)> = vec![];
	for i in 0..target.len(){
		if let EngineMessage::ObjSpawn(id, pos) = target[i] {
			ret.push(
				(id, pos.x.to_num::<f32>(), pos.y.to_num::<f32>())
				);
		}
	}

	return ret;
}

pub fn inbox_drain_move(inbox: &mut Vec<EngineMessage>) -> 
	Vec<(ObjectID, f32, f32)> 
	{
	let (target, rest): (Vec<EngineMessage>, Vec<EngineMessage>) = inbox
		.clone()
		.iter()
		.partition(|&msg| match msg {
			EngineMessage::ObjMove(..) => true,
			_ => false,
		});

	*inbox = rest;

	// turn messages into tuples:
	let mut ret: Vec<(ObjectID, f32, f32)> = vec![];
	for i in 0..target.len(){
		if let EngineMessage::ObjMove(id, pos) = target[i] {
			ret.push(
				(id, pos.x.to_num::<f32>(), pos.y.to_num::<f32>())
				);
		}
	}

	return ret;
}


pub fn inbox_drain_fps(inbox: &mut Vec<EngineMessage>) -> 
	Vec<u64> 
	{
	let (target, rest): (Vec<EngineMessage>, Vec<EngineMessage>) = inbox
		.clone()
		.iter()
		.partition(|&msg| match msg {
			EngineMessage::Fps(..) => true,
			_ => false,
		});

	*inbox = rest;

	// turn messages into tuples:
	let mut ret: Vec<u64> = vec![];
	for i in 0..target.len(){
		if let EngineMessage::Fps(fps) = target[i] {
			ret.push(fps);
		}
	}

	return ret;
}

pub fn inbox_drain_dest(inbox: &mut Vec<EngineMessage>) -> 
	Vec<(ObjectID, f32, f32)> 
	{
	let (target, rest): (Vec<EngineMessage>, Vec<EngineMessage>) = inbox
		.clone()
		.iter()
		.partition(|&msg| match msg {
			EngineMessage::ObjDest(..) => true,
			_ => false,
		});

	*inbox = rest;

	// turn messages into tuples:
	let mut ret: Vec<(ObjectID, f32, f32)> = vec![];
	for i in 0..target.len(){
		if let EngineMessage::ObjDest(id, pos) = target[i] {
			ret.push(
				(id, pos.x.to_num::<f32>(), pos.y.to_num::<f32>())
				);
		}
	}

	return ret;
}