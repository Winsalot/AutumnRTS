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
	plc_unit3d = preload("res://Presentation/Units/Placeholder_Marine.tscn")
	
	RenderState.rustbridge.start_loop(2,2)



func _process(_delta):
	# should be first thing every frame
	RenderState.rustbridge.receive_sim_messages() 
	spawn_map()
	spawn_units()
	
	RenderState.rustbridge.clear_inbox()
	RenderState.rustbridge.deliver_input() 

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


func spawn_units():
	var spawn_info = RenderState.rustbridge.get_msg_spawn()
	#if spawn_info.size() > 0:
	#	print(spawn_info)
	for unit_spawn in spawn_info:
		var unit = plc_unit3d.instance()
		var xy = Vector2(unit_spawn[3], unit_spawn[4]) #* params.scale
		unit.set_translation(Vector3(xy.x, 0, xy.y))
		unit.set_name(unit_name(unit_spawn[0]))
#		unit.player = unit_spawn[1]
#		unit.team = unit_spawn[2]
#		unit.unique_id = unit_spawn[0]
#		unit.coll_radius = unit_spawn[5]
		self.add_child(unit)
	pass

