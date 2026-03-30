use bevy::asset::{Asset, Handle};
use bevy::image::Image;
use bevy::math::Vec4;
use bevy::prelude::TypePath;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;
use bevy::sprite_render::Material2d;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct ScreenMaterial {
    #[uniform(0)]
    color: Vec4,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Handle<Image>,
}

impl Material2d for ScreenMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/screen.wgsl".into()
    }
}