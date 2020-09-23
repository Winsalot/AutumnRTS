extends Control


# Declare member variables here. Examples:
# var a = 2
# var b = "text"


# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.

# TODO: this should probably use _gui_input(). However, gui input doesnt work for some reason
func _unhandled_input(event):
	if event.is_action_pressed("quit"):
		get_tree().quit()



func _process(delta):
	update_fps(delta)	
	pass

func update_fps(delta):
	var own_fps = String(1.0/(delta+(1.0/2000.0)))
	var text = "FPS: " + String(own_fps)
	get_node("TopPanel/HBoxContainer/FPS label").set_text(text)
