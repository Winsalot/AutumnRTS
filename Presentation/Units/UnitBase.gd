extends KinematicBody


var uid = -1 # default value

var real_pos = Vector2(0,0)
export(float) var unit_size = 1

onready var gravity = -ProjectSettings.get_setting("physics/3d/default_gravity")
var velocity: Vector3

func _ready():
	$CollisionShape.shape.radius = self.unit_size / 2
#	$CollisionShape.shape.height = self.unit_size / 2
	$CollisionShape.translation = Vector3(0.0, self.unit_size / 2, 0.0)
	
	pass # Replace with function body.

func _physics_process(delta):
		velocity.y += delta * gravity
		velocity = move_and_slide(velocity, Vector3.UP)

#these functions should be overloaded I guess:
func get_uid():
	return self.uid

func set_uid(id):
	self.uid = id

func get_rpos():
	return self.real_pos

func set_rpos(xy):
	self.real_pos = xy
