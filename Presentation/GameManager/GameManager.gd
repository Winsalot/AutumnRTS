extends Node


var maptile

# Called when the node enters the scene tree for the first time.
func _ready():
	RenderState.set("gamemanager", self) # set value in singleton
	maptile = preload("res://Presentation/Map/MapTile.tscn")
	
	RenderState.rustbridge.start_loop(2,10)



func _process(delta):
	RenderState.rustbridge.receive_sim_messages() # should be first thing every frame
	spawn_map()
	
	RenderState.rustbridge.clear_inbox()
	RenderState.rustbridge.deliver_input() 

# Shitty implementation for now:
func spawn_map():
	var map_info = RenderState.rustbridge.get_msg_map()
	print(map_info)
	for tile_spawn in map_info:
		var tile = maptile.instance()
		var xzy = Vector3(tile_spawn[0],float(tile_spawn[3])/2,tile_spawn[1]) 
		tile.set_translation(xzy)
		tile.set_name("Tile:" + String(xzy))
		if tile_spawn[2]:
			tile.set_scale(Vector3(0.8, 1, 0.8))
#		tile.block_path = tile_spawn[2]
#		tile.z_level = tile_spawn[3]
		self.add_child(tile)
		print("tile added")
		#print("Tile ", tile.get_name(), " placed at " + String(tile.get_position()) + ", block: "+ String(tile.block_path))
