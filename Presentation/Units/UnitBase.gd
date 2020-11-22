extends KinematicBody


var uid = -1 # default value

var real_pos = Vector2(0,0)
export(float) var unit_size = 1

onready var gravity = -ProjectSettings.get_setting("physics/3d/default_gravity")
var velocity: Vector3

var is_selected = false
var mouse_entered = false

func _ready():
	$CollisionShape.shape.radius = self.unit_size / 4
#	$CollisionShape.shape.height = self.unit_size / 2
	$CollisionShape.translation = Vector3(0.0, self.unit_size / 4, 0.0)
	
	pass # Replace with function body.

func _physics_process(delta):
	if !is_on_floor():
		velocity.y += delta * gravity
		velocity = move_and_slide(velocity, Vector3.UP)

func _unhandled_input(event):
	if event.is_action_pressed("mouse_select_single") && \
	mouse_entered:
		self.select() 
		self.get_tree().set_input_as_handled()

#these functions should be overloaded I guess:
func get_uid():
	return self.uid

func set_uid(id):
	self.uid = id

func get_rpos():
	return self.real_pos

func set_rpos(xy):
	self.real_pos = xy

func deselect():
	is_selected = false
	#RenderState.deselect(self) # TBA

func select():
	is_selected = true
	RenderState.select_add(self)


func _on_Clickbox_Area_mouse_entered():
	mouse_entered = true


func _on_Clickbox_Area_mouse_exited():
	mouse_entered = false
