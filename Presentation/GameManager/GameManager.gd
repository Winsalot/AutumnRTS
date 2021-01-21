extends Node


var maptile
var maptile_block
var plc_unit3d

# Called when the node enters the scene tree for the first time.
func _ready():
	RenderState.set("gamemanager", self) # set value in singleton
#	maptile = preload("res://Presentation/Map/MapTile.tscn")
	maptile = preload("res://Presentation/Map/Tile_plc_non_block.tscn")
	maptile_block = preload("res://Presentation/Map/Tile_plc_block.tscn")
#	plc_unit3d = preload("res://Presentation/Units/Placeholder_Marine.tscn")
	plc_unit3d = preload("res://Presentation/Units/Electro_marine3.tscn")
		
	start_loop()



func _process(_delta):
	# should be first thing every frame
	RenderState.rustbridge.receive_sim_messages() 
	spawn_map()
	spawn_units()
	new_units()
	
	update_real_pos()
	
	RenderState.rustbridge.clear_inbox()
	RenderState.rustbridge.deliver_input() 

func start_loop():
	# TODO: this should initialise RenderState.sim_params values
	RenderState.rustbridge.start_loop(2,2)


# Shitty implementation for now:
func spawn_map():
	var map_info = RenderState.rustbridge.get_msg_map()
	for tile_spawn in map_info:
		var tile 
		if tile_spawn[2]:
			tile = maptile_block.instance()
		else:
			tile = maptile.instance()
		var xzy = Vector3(tile_spawn[0],float(tile_spawn[3])/2,tile_spawn[1]) 
		tile.set_translation(xzy)
		tile.set_name("Tile:" + String(xzy))
#		tile.block_path = tile_spawn[2]
#		tile.z_level = tile_spawn[3]
		self.add_child(tile)
#		print("tile added")
		#print("Tile ", tile.get_name(), " placed at " + String(tile.get_position()) + ", block: "+ String(tile.block_path))

# Sends the message to simulation to spawn unit
func debug_spawn_unit_msg():
	var position2D = get_viewport().get_mouse_position()
	var dropPlane  = Plane( 0, 1, 0, 0 )
	var position3D = dropPlane.intersects_ray(RenderState.camera.project_ray_origin(position2D),RenderState.camera.project_ray_normal(position2D))
	RenderState.rustbridge.tmp_spawn_obj(RenderState.player_id, Vector2(position3D.x,position3D.z))


# Sends the message to simulation to spawn unit
func debug_spawn_smart():
	var position2D = get_viewport().get_mouse_position()
	var dropPlane  = Plane( 0, 1, 0, 0 )
	var position3D = dropPlane.intersects_ray(RenderState.camera.project_ray_origin(position2D),RenderState.camera.project_ray_normal(position2D))
	RenderState.rustbridge.debug_spawn_smart(RenderState.player_id, Vector2(position3D.x,position3D.z))

func unit_name(id):
	#var params = self.get_node("/root/PresentationParams") #autoload node
	return "U_" + String(id)

# Spawns dumb units
func spawn_units():
	var spawn_info = RenderState.rustbridge.get_msg_spawn()
	#if spawn_info.size() > 0:
	#	print(spawn_info)
	for unit_spawn in spawn_info:
		var unit = plc_unit3d.instance()
		var xy = Vector2(unit_spawn[3], unit_spawn[4]) #* params.scale
		unit.set_translation(Vector3(xy.x, 0.0, xy.y))
		unit.set_name(unit_name(unit_spawn[0]))
#		unit.player = unit_spawn[1]
#		unit.team = unit_spawn[2]
#		unit.unique_id = unit_spawn[0]
#		unit.coll_radius = unit_spawn[5]
		self.add_child(unit)
	pass

# spawns smart units
func new_units():
	var spawn_info = RenderState.rustbridge.get_msg_new_unit()
	for unit_spawn in spawn_info:
		var unit = plc_unit3d.instance()
		var xy = unit_spawn.get("pos")
		unit.set_translation(Vector3(xy.x, 3, xy.y))
		unit.set_name(unit_name(unit_spawn.get("uid")))
		self.add_child(unit)
		unit.set_uid(unit_spawn.get("uid"))
#		unit.set_rpos(Vector2(xy.x, xy.y))
		unit.real_pos = Vector2(xy.x, xy.y) # temporary sulution, because calling set_rpos triggers move animation haha

# Updates real position information
# TODO: will error if unit with that name doesnt exist
func update_real_pos():
	var move_info = RenderState.rustbridge.get_msg_move()
	for unit_move in move_info:
		var unit = self.get_node(unit_name(unit_move[0]))
		unit.set_rpos(Vector2(unit_move[1], unit_move[2]))

