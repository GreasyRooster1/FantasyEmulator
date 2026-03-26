mod instructions;
mod tests;

use bevy::prelude::*;

const MEM_SIZE: usize = 65_535; //limited by 8bit bytes
const REGISTER_COUNT: usize = 16; // limited by asm arguments

#[derive(Resource)]
struct Emulator {
    physical_memory: [u8; MEM_SIZE],
    registers: [u8; MEM_SIZE],
}


fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}
