use std::thread::{JoinHandle};
use gdnative::*;
use crate::sim_fix_math::{Pos, FixF};
//use snowflake::ProcessUniqueId;

mod sim_object;
mod fpscounter;
mod sim_fix_math;
mod sim_gameloop;
mod messenger;
mod simulation;
mod rustbridge;
mod rustbridge_messages;
mod sim_unit_base_components;
mod sim_systems;
mod sim_ecs;

use rustbridge::*;





// Function that registers all exposed classes to Godot
fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<RustBridge>();
    //handle.add_class::<RustBridge_GameLoop>();
}

// macros that create the entry-points of the dynamic library.
godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
