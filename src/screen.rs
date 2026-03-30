use bevy::asset::{Asset, AssetServer, Assets, Handle};
use bevy::color::LinearRgba;
use bevy::image::Image;
use bevy::math::{Vec3, Vec4};
use bevy::mesh::{Mesh, Mesh2d};
use bevy::prelude::{Camera2d, Commands, Component, MeshMaterial2d, Query, Rectangle, Res, ResMut, Transform, TypePath, With};
use bevy::render::render_resource::{AsBindGroup, Buffer, ShaderType};
use bevy::shader::ShaderRef;
use bevy::sprite_render::Material2d;


#[derive(Clone, Debug, ShaderType)]
pub struct ScreenData{
    data:[u32;2048],
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct ScreenMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[uniform(1)]
    screen_data: ScreenData,
}

#[derive(Component)]
pub struct Screen;

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
            screen_data: ScreenData{data:[0; 2048]},
        })),
        Transform::default().with_scale(Vec3::splat(500.)),
        Screen
    ));
}

pub fn update_screen(
    mut materials: ResMut<Assets<ScreenMaterial>>,
    query: Query<&mut MeshMaterial2d<ScreenMaterial>, With<Screen>>,
) {
    for handle in query.iter() {
        if let Some(material) = materials.get_mut(handle) {
            material.screen_data.data=[1; 2048]
        }
    }
}