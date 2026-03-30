#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct ScreenData {
  data: array<vec4<f32>, 2048>
};

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> material_color: vec4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var<uniform> data: ScreenData;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let uv = mesh.uv;
    let pixel_pos = floor(uv*128.0)/128.0;
    return vec4(pixel_pos,1.0,1.0);
}