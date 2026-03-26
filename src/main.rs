mod cpu;
mod instructions;
mod tests;

use bevy::prelude::*;

const MEM_SIZE: usize = 65_535; //limited by 8bit bytes
const ROM_SIZE: usize = 65_535; //limited by 8bit bytes
const REGISTER_COUNT: usize = 16; // limited by asm arguments
const RA_REGISTER: usize = 0xF;
const PC_REGISTER: usize = 0xE;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}

pub fn get_nibble_from_byte(data: u32, i: u32) -> u8 {
    let nib_idx = i + 1;
    dbg!(
        format!("{:#b}", data >> (32 - nib_idx * 4)),
        32 - nib_idx * 4,
        format!("{data:#b}")
    );
    ((data >> (32 - nib_idx * 4)) & 0x0F) as u8
}

