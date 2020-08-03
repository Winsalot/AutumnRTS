extends Node
class_name GameManager

# DESCRIPTION:
# This node is responsible from taking messages from Rustbridge 
# and representing the game state. 
# IT DOES NOT: does not send user input back to rustbridge

# parent node, for function calls ans stuff:
var rustbridge;

var plc_unit; # placeholder unit. Will use dictionary with multiple units
var plc_tile;
var params;




func _ready():
	plc_unit = preload("res://Presentation_Godot/Scenes/Placeholder_Unit.tscn")
	plc_tile = preload("res://Presentation_Godot/Scenes/TileInfo.tscn")
	self.set_process_input(false)
	# rustbridge: RustBridge = self.get_parent()	
	params = self.get_node("/root/PresentationParams")
	rustbridge = self.get_parent()
	rustbridge.start_loop(1, 2) 
	pass 


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	rustbridge.receive_sim_messages() # should be first thing every frame
	
	spawn_units()
	move_units()
	get_engine_fps()
	set_unit_dest()
	spawn_map()
	
	rustbridge.clear_inbox()
	rustbridge.deliver_input() # Should be last action in presentation tick
	pass

func unit_name(id):
	#var params = self.get_node("/root/PresentationParams") #autoload node
	return params.unit_name_prefix + String(id)

func spawn_units():
	var spawn_info = rustbridge.get_msg_spawn()
	for unit_spawn in spawn_info:
		var unit = plc_unit.instance()
		var xy = Vector2(unit_spawn[1], unit_spawn[2]) * params.scale
		unit.set_position(xy)
		unit.set_name(unit_name(unit_spawn[0]))
		unit.unique_id = unit_spawn[0]
		self.add_child(unit)
	pass

func spawn_map():
	var map_info = rustbridge.get_msg_map()
	for tile_spawn in map_info:
		#print(tile_spawn)
		var tile = plc_tile.instance()
		var xy = Vector2(tile_spawn[0], tile_spawn[1]) * params.scale 
		tile.set_position(xy)
		tile.set_name("Tile:" + String(xy))
		tile.block_path = tile_spawn[2]
		tile.z_level = tile_spawn[3]
		self.add_child(tile)
		#print("Tile ", tile.get_name(), " placed at " + String(tile.get_position()) + ", block: "+ String(tile.block_path))


func move_units():
	var move_info = rustbridge.get_msg_move()
	for unit_move in move_info:
		var unit = self.get_node(unit_name(unit_move[0]))
		var xy = Vector2(unit_move[1], unit_move[2]) * params.scale
		unit.set_position(xy)
	pass

func get_engine_fps():
	var sim_fps =  rustbridge.get_msg_fps()
	if !sim_fps.empty():
		self.get_node("/root/PresentationParams").sim_fps = sim_fps.back()

func set_unit_dest():
	var dest_info = rustbridge.get_msg_dest()
	for unit_dest in dest_info:
		var unit = self.get_node(unit_name(unit_dest[0]))
		unit.dest = Vector2(unit_dest[1], unit_dest[2])*params.scale
