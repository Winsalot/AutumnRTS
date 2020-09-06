use gdnative::*;
use rustbridge::*;

mod common;
mod fpscounter;
mod messenger;
mod rustbridge;
mod rustbridge_messages;
mod sim_abilities_list;
mod sim_components;
mod sim_ecs;
mod sim_fix_math;
mod sim_gameloop;
mod sim_map;
mod sim_player_alliances;
mod sim_systems;
mod sim_weapon_list;

// Function that registers all exposed classes to Godot
fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<RustBridge>();
}

// macros that create the entry-points of the dynamic library.
godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
