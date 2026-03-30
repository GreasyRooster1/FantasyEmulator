use bevy::asset::{Asset, AssetServer, Assets, Handle};
use bevy::color::LinearRgba;
use bevy::image::Image;
use bevy::math::{Vec3, Vec4};
use bevy::mesh::{Mesh, Mesh2d};
use bevy::prelude::{Camera2d, Commands, MeshMaterial2d, Rectangle, Res, ResMut, Transform, TypePath};
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;
use bevy::sprite_render::Material2d;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct ScreenMaterial {
    #[uniform(0)]
    color: LinearRgba,
}

impl Material2d for ScreenMaterial {
    fn fragment_shader() -> ShaderRef {
        "screen.wgsl".into()
    }
}

pub fn setup_screen(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ScreenMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // camera
    commands.spawn(Camera2d);

    // quad
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(ScreenMaterial {
            color: LinearRgba::BLUE,

        })),
        Transform::default().with_scale(Vec3::splat(128.)),
    ));
}