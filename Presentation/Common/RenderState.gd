extends Node

#Autoload script that holds all UI related info. 
#Including UI related node references

var rustbridge
var gamemanager
var camera
var camera_pivot

var mouse_area

onready var player_id = 0

# Called when the node enters the scene tree for the first time.
func _ready():
	if mouse_area == null:
		mouse_area = "game"
	pass # Replace with function body.

func mouse_in_game():
	self.mouse_area == "game"
