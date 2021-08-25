// Vertex shader
struct VertexInput {
	[[location(0)]] position: vec3<f32>;
	[[location(1)]] texture_coords: vec2<f32>;
};

struct VertexOutput {
	[[builtin(position)]] clip_coordinate: vec4<f32>;
	[[location(0)]] texture_coords: vec2<f32>;
};

[[stage(vertex)]]
fn main(model: VertexInput) -> VertexOutput {
	var out: VertexOutput;
	out.clip_coordinate = vec4<f32>(model.position, 1.0);
	out.texture_coords = model.texture_coords;
	return out;
}

// Fragment shader
[[group(0), binding(0)]]
var t_aqua: texture_2d<f32>;
[[group(0), binding(1)]]
var s_aqua: sampler;

[[stage(fragment)]]
fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
	//return vec4<f32>(in.color, 1.0);
	return textureSample(t_aqua, s_aqua, in.texture_coords);
}
