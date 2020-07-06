extends Node2D
class_name Unit


export(Vector2) var real_pos; # Real position in simulation
export(Vector2) var dest; # Real position in simulation
export(bool) var is_selected
export(int) var unique_id;
export(Vector2) var dimensions;

var gui;


# Called when the node enters the scene tree for the first time.
func _ready():
	real_pos = self.get_position();
	is_selected = false;
	gui = get_node("/root/RustBridge/GUI")
	dest = self.get_position()
	dimensions = self.get_node("/root/PresentationParams").scale;
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
	#print(self.get_position() - self.dest)
	#pass


func _on_Area2D_input_event(viewport, event, shape_idx):
	if event.is_action_pressed("mouse_select_single"):
		#print("Mouse clicked inside object. Node name: ", self.get_name())
		gui.select_unit(self)
		$Sprite.set_self_modulate(Color(0.7, 1.0, 0.7,1))
		self.get_tree().set_input_as_handled()
		print("imput marked as handled")
	pass # Replace with function body.
