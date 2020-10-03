extends Node


var uid = -1 # default value

func _ready():
	pass # Replace with function body.

#these functions should be overloaded I guess:
func get_uid():
	return self.uid

func set_uid(id):
	self.uid = id
