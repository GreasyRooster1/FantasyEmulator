mod instructions;
mod tests;
mod cpu;

use bevy::prelude::*;

const MEM_SIZE: usize = 65_535; //limited by 8bit bytes
const REGISTER_COUNT: usize = 16; // limited by asm arguments
const RA_REGISTER: usize = 0xF;
const PC_REGISTER: usize = 0xE;


fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}
