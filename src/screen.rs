use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use bevy::asset::{Asset, AssetServer, Assets, Handle};
use bevy::color::{LinearRgba, Srgba};
use bevy::image::Image;
use bevy::math::{Vec3, Vec4};
use bevy::mesh::{Mesh, Mesh2d};
use bevy::prelude::{Camera2d, Commands, Component, MeshMaterial2d, Query, Rectangle, Res, ResMut, Transform, TypePath, With};
use bevy::render::render_resource::{AsBindGroup, Buffer, ShaderType};
use bevy::shader::ShaderRef;
use bevy::sprite_render::Material2d;


#[derive(Clone, Debug, ShaderType)]
pub struct ScreenData{
    data:[Vec4;2048],
    palette:[Vec4; 16],
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
            screen_data: ScreenData{
                data:[Vec4::splat(0.0); 2048],
                palette:load_palette().unwrap()
            }
        })),
        Transform::default().with_scale(Vec3::splat(500.)),
        Screen
    ));
}

pub fn load_palette() -> io::Result<[Vec4;16]>{
    let path = "example.txt";
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut palette:Vec<Vec4> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let color = Srgba::hex(format!("#{line}FF")).unwrap();
        palette.push(Vec4::new(color.red,color.green,color.blue,1.0));
    }
    let slice = palette.as_slice().try_into().unwrap();
    Ok(slice)
}

pub fn update_screen(
    mut materials: ResMut<Assets<ScreenMaterial>>,
    query: Query<&mut MeshMaterial2d<ScreenMaterial>, With<Screen>>,
) {
    for handle in query.iter() {
        if let Some(material) = materials.get_mut(handle) {
            material.screen_data.data=[Vec4::splat(1.0); 2048]
        }
    }
}