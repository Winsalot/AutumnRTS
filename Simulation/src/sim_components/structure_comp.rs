use crate::sim_fix_math::*;

// Component for buildings
// SHould contain location and occupied squares around it.
// Location should not be float.

pub struct StructureComp {
	pos: Pos, // centre of the structure
	tiles: Vec<Pos> // occupied tiles as vectors from centre.
}

impl StructureComp {
	pub fn new(pos: &Pos) -> Self {
		StructureComp {
			pos: pos.clone(),
			tiles: vec![pos.clone()]
		}
	}
}