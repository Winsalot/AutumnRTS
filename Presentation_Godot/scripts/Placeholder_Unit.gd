extends Node2D
class_name Unit


export(Vector2) var real_pos; # Real position in simulation
export(Vector2) var dest; # Real position in simulation
export(float) var coll_radius;
export(bool) var is_selected
export(int) var unique_id;
export(Vector2) var pixel_scale;
export(Vector2) var next_pos;
var path
var real_path

var gui;
var params;


# Called when the node enters the scene tree for the first time.
func _ready():
	#real_pos = self.get_position(); # wtf? That's not right
	next_pos = real_pos;
	dest = real_pos;
	is_selected = false;
	gui = get_node("/root/RustBridge/GUI")
	params = self.get_node("/root/PresentationParams")
	pixel_scale = params.scale;
	path = PoolVector2Array()
	real_path = PoolVector2Array()
	
	self.position = real_pos * pixel_scale
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	#print(self.get_position() - self.dest)
	#$NextPos.set_position(next_pos - self.position)
	$NextPos.position = (next_pos - real_pos) * pixel_scale
	if params.detailed_info:
		$Details.set_text("real_pos: " + String(real_pos) + \
		"\n" + "next_pos: " + String(next_pos) + \
		"\n" + "collision_radius: " +String(coll_radius))
	else:
		$Details.set_text("")
	
	# update pathfinding:
	if !path.empty():
		if real_pos == real_path[real_path.size() - 1]:
			self.set_path(PoolVector2Array())
	
	path = PoolVector2Array()
	for xy in real_path:
		path.push_back((xy - real_pos) * pixel_scale)
	self.update()
	
	pass


func _draw():
	# Your draw commands here
	if !is_selected:
		draw_circle_custom(coll_radius * pixel_scale.x, true, Color(0.1, 0.1, 0.3, 0.9))
	else:
		draw_circle_custom(coll_radius * pixel_scale.x, true, Color(0.1, 0.1, 0.3, 0.9))
		draw_circle_custom(coll_radius * pixel_scale.x + 5, false, Color(0.960, 0.945, 0.078, 0.95))
	#print("Draw call executed")
	if path.size() >= 2:
		draw_polyline(path, Color(0.1, 0.1, 0.9, 0.9), 5.0)
	pass

func _on_Area2D_input_event(_viewport, event, _shape_idx):
	if event.is_action_pressed("mouse_select_single"):
		#print("Mouse clicked inside object. Node name: ", self.get_name())
		gui.select_unit(self)
		is_selected = true
		#$Sprite.set_self_modulate(Color(0.7, 1.0, 0.7,1))
		#self.update()
		self.get_tree().set_input_as_handled()
		#print("imput marked as handled")
	pass # Replace with function body.


func draw_circle_custom(radius, fill = true, col = Color(0.1, 0.1, 0.3, 0.9)):
	
	if radius <= 0.0:
		return
		
	var maxerror = 0.25
	var maxpoints = 128 # I think this is renderer limit
	
	var numpoints = ceil(PI / acos(1.0 - maxerror / radius))
	numpoints = clamp(numpoints, 3, maxpoints)
	
	var points = PoolVector2Array([])
	
	for i in numpoints:
		var phi = i * PI * 2.0 / numpoints
		var v = Vector2(sin(phi), cos(phi))
		points.push_back(v * radius)
	if fill:
		draw_colored_polygon(points, col)
	else:
		draw_polyline(points, col, 3)

func set_real_pos(xy):
	real_pos = xy
	self.position = real_pos * pixel_scale

func set_next_pos(xy):
	next_pos = xy
	#print("Next pos: "+String($NextPos.position) + "Own pos: " + String(self.position))

func set_dest(xy):
	dest = xy


func set_path(positions):
	real_path = PoolVector2Array(positions)



