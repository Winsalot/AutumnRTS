extends Control


# Declare member variables here. Examples:
# var a = 2
# var b = "text"


# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.

## TODO: this should probably use _gui_input().
## However, gui input doesnt work for some reason
##func _gui_input(event):
#func _unhandled_input(event):
#	if event.is_action_pressed("quit"):
#		get_tree().quit()
#		self.get_tree().set_input_as_handled()
#	# action processing when mouse in game:
#	if event.is_action_pressed("debug_spawn_unit"):
#		RenderState.gamemanager.debug_spawn_unit_msg()
#		self.get_tree().set_input_as_handled()
#	if event.is_action_pressed("deselect_all"):
#		RenderState.deselect_all()
#		self.get_tree().set_input_as_handled()

#func in_game_input(event):
#	if event.is_action_pressed("debug_spawn_unit"):
#		print("Spawn unit!")

func _process(delta):
	update_fps(delta)
	pass

func update_fps(delta):
	var fps = Performance.get_monitor(Performance.TIME_FPS)
	var text = "FPS: " +  String(fps)
	get_node("TopPanel/HBoxContainer/FPS label").set_text(text)



# Some signals for mouse position:
# I am not a fan of this imp,ementation, but I want to get things done
#func _on_BottomPanel_mouse_entered():
#	RenderState.mouse_area = "gui"
#	print("mouse entered")
#
#func _on_BottomPanel_mouse_exited():
#	RenderState.mouse_area = "game"
#	print("mouse exited")
#
#func _on_TopPanel_mouse_entered():
#	RenderState.mouse_area = "gui"
#	print("mouse entered")
#
#func _on_TopPanel_mouse_exited():
#	RenderState.mouse_area = "game"
#	print("mouse exited")
