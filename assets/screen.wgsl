#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct ScreenData {
  data: array<vec4<f32>, 2048>,
  palette: array<vec4<f32>, 16>
};

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> material_color: vec4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var<uniform> data: ScreenData;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let uv = mesh.uv;
    let pixel_pos = floor(uv*128.0)/128.0;
    let index = floor(pixel_pos.u+pixel_pos.v*128.0);
    let full = data.data[floor(index/4.0)]
    let comp = index % 4;
    let val = 0;
    if(comp == 0){
        val = full.x
    }
    if(comp == 1){
        val = full.y
    }
    if(comp == 2){
        val = full.z
    }
    if(comp == 3){
        val = full.w
    }
    let col = data.palette[val];
    return col;
}