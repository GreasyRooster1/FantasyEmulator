mod cpu;
mod instructions;
mod tests;
mod screen;

use std::thread;
use bevy::prelude::*;
use bevy::sprite_render::Material2dPlugin;
use crate::cpu::Emulator;
use crate::screen::{setup_screen, update_screen, ScreenMaterial};

const MEM_SIZE: usize = 65_535; // could be as large as an i32
const ROM_SIZE: usize = 65_535; // could be as large as an i32
const REGISTER_COUNT: usize = 256; // limited by asm arguments
const PC_REGISTER: usize = 0xFF;
const RA_REGISTER: usize = 0xFE;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins,
                     Material2dPlugin::<ScreenMaterial>::default()))
        .insert_resource(Emulator::hardware_setup())
        .add_systems(Startup, (boot_cpu,setup_screen))
        .add_systems(Update, (cpu_cycle,update_screen))
        .run();
}

fn boot_cpu(mut emulator:ResMut<Emulator>){
    emulator.boot("./data/boot.rom".to_string());
}

fn cpu_cycle(mut emulator:ResMut<Emulator>){
    if !emulator.running() {
        return;
    }
    emulator.cpu_cycle();
}

pub fn get_nibble_from_byte(data: u128, i: u32) -> u8 {
    let nib_idx = i + 1;
    ((data >> (128 - nib_idx * 4)) & 0x0F) as u8
}

pub fn get_byte_from_data(data: u128, i: u32) -> u8 {
    let nib_idx = i + 1;
    dbg!(
        format!("{:#b}", data >> (128 - nib_idx * 8)),
        128 - nib_idx * 8,
        format!("{data:#b}"),
        format!("{:#b}", (data >> (128 - nib_idx * 8)))
    );
    ((data >> (128 - nib_idx * 8)) & 0xFF) as u8
}

