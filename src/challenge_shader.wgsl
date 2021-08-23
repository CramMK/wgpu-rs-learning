// Vertex shader

struct VertexOutput {
    [[builtin(position)]] clip_coordinates: vec4<f32>;
    [[location(0)]] position: vec2<f32>;
};

[[stage(vertex)]]
fn main(
    [[builtin(vertex_index)]] in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    out.clip_coordinates = vec4<f32>(x, y, 0.0, 1.0);
    out.position = vec2<f32>(x, y);
    return out;
}

// Fragment shader

[[stage(fragment)]]
fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
   let r = in.position.x;
   let g = in.position.y;
   return vec4<f32>(r, g, 0.1, 1.0);
}
