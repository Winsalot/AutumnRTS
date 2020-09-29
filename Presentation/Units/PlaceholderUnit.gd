extends Spatial


var is_selected = false;
var mouse_entered = false


# Called when the node enters the scene tree for the first time.
func _ready():
	print("Unit3D has been spawned!")
	$SelectionCircle.visible = false
	pass # Replace with function body.


func deselect():
	is_selected = false
	$SelectionCircle.visible = false

func select():
	is_selected = true
	RenderState.select_add(self)
	$SelectionCircle.visible = true

func _unhandled_input(event):
	if event.is_action_pressed("mouse_select_single") && \
	mouse_entered:
		self.select() # This should be called from somehwere else
		self.get_tree().set_input_as_handled()

func _on_Area_mouse_entered():
	mouse_entered = true;
	pass

func _on_Area_mouse_exited():
	mouse_entered = false;
	pass
