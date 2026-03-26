use bevy::prelude::Resource;
use crate::{MEM_SIZE, REGISTER_COUNT};

#[derive(Resource)]
struct Emulator {
    physical_memory: [u8; MEM_SIZE],
    registers: [u8; REGISTER_COUNT],
}

impl Emulator{
    pub fn cpu_cycle(&mut self){
    }
}