struct Transform {
    [[location(2)]] data0: vec4<f32>;
    [[location(3)]] data1: vec4<f32>;
    [[location(4)]] data2: vec4<f32>;
    [[location(5)]] data3: vec4<f32>;
};

struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] tex_coords: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] tex_coords: vec2<f32>;
};

[[stage(vertex)]]
fn vertex_main(vertex: VertexInput, transform: Transform) -> VertexOutput {
    let transform_matrix = mat4x4<f32>(
        transform.data0,
        transform.data1,
        transform.data2,
        transform.data3,
    );

    var output: VertexOutput;
    output.clip_position = transform_matrix * vec4<f32>(vertex.position, 1.0);
    output.tex_coords = vertex.tex_coords;
    return output;
}

[[group(0), binding(0)]]
var texture: texture_2d<f32>;
[[group(0), binding(1)]]
var texture_sampler: sampler;

[[stage(fragment)]]
fn fragment_main(vertex: VertexOutput) -> [[location(0)]] vec4<f32> {
    return textureSample(texture, texture_sampler, vertex.tex_coords);
}
