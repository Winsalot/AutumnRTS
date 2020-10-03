extends Node


# Declare member variables here. Examples:
# var a = 2
# var b = "text"


# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.

func mouse_pos_projection():
	var position2D = get_viewport().get_mouse_position()
	var dropPlane  = Plane( 0, 1, 0, 0 )
	var position3D = dropPlane.intersects_ray(\
	RenderState.camera.project_ray_origin(position2D),\
	RenderState.camera.project_ray_normal(position2D))
	return position3D

func _unhandled_input(event):
	if event.is_action_pressed("quit"):
		get_tree().quit()
		self.get_tree().set_input_as_handled()
	# action processing when mouse in game:
	if event.is_action_pressed("debug_spawn_unit"):
		RenderState.gamemanager.debug_spawn_unit_msg()
		self.get_tree().set_input_as_handled()
	if event.is_action_pressed("debug_spawn_smart_unit"):
		RenderState.gamemanager.debug_spawn_smart()
		self.get_tree().set_input_as_handled()
	if event.is_action_pressed("deselect_all"):
		RenderState.deselect_all()
		self.get_tree().set_input_as_handled()
	if event.is_action_pressed("move_order_selected"):
		var xy = mouse_pos_projection()
		self.input_order_move(xy)

func input_order_move(target_pos):
	var selected_ids = []
	for unit in RenderState.selected_units:
		selected_ids.push_back(unit.get_uid())
	RenderState.rustbridge.\
	input_order_move(RenderState.player_id,\
	selected_ids, \
	Vector2(target_pos.x, target_pos.z))
