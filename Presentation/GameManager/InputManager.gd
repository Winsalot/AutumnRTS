extends Node


# Declare member variables here. Examples:
# var a = 2
# var b = "text"


# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


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
