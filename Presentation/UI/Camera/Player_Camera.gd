extends Spatial

var MOVE_MARGIN = 20 # pixels
var MOVE_SPEED = 4 # idk what units lul


onready var camera = $Camera


func _ready():
	RenderState.set("camera", self)
	pass # Replace with function body.

func calc_move(delta):
	var v_size = self.get_viewport().size
	var m_xy = self.get_viewport().get_mouse_position()
	var move_vec = Vector2()
	
	if  m_xy.x <= MOVE_MARGIN:
		move_vec.x -= 1
	if  m_xy.y <= MOVE_MARGIN:
		move_vec.y -= 1
	if  m_xy.x >= (v_size.x - MOVE_MARGIN):
		move_vec.x += 1
	if  m_xy.y >= (v_size.y - MOVE_MARGIN):
		move_vec.y += 1
	
	var ret = Vector3(move_vec.x * MOVE_SPEED, 0, move_vec.y * MOVE_SPEED)
	return ret*delta

func _process(delta):
	self.set_translation(self.get_translation() + calc_move(delta))
