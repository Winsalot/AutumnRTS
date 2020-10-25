extends Spatial


var uid = -1 # default value

var real_pos = Vector2(0,0)

func _ready():
	pass # Replace with function body.

#these functions should be overloaded I guess:
func get_uid():
	return self.uid

func set_uid(id):
	self.uid = id

func get_rpos():
	return self.real_pos

func set_rpos(xy):
	self.real_pos = xy
