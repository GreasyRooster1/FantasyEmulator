use bevy::prelude::*;

const MEM_SIZE: usize = 65_535;

#[derive(Resource)]
struct Emulator{
    physical_memory: [u8; MEM_SIZE],
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}
