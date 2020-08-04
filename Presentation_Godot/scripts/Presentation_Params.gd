extends Node

# This is a singleton which holds presentation parameters. 
# Right now it is just scale factor between Presentation and simulation

var scale = Vector2(64.0, 64.0)
var unit_name_prefix = "unit_id"
var sim_fps = 0
var detailed_info = false



# Called when the node enters the scene tree for the first time.
func _ready():
	#print(self.get_path())
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
