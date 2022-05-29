struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] color: vec4<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] color: vec4<f32>;
};

[[stage(vertex)]]
fn vertex_main(vertex: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.clip_position = vec4<f32>(vertex.position, 1.0);
    output.color = vertex.color;
    return output;
}

[[stage(fragment)]]
fn fragment_main(vertex: VertexOutput) -> [[location(0)]] vec4<f32> {
    return vertex.color;
}
