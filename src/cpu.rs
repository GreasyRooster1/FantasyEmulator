use bevy::prelude::Resource;
use crate::{MEM_SIZE, REGISTER_COUNT};

#[derive(Resource)]
struct Emulator {
    physical_memory: [u8; MEM_SIZE],
    registers: [u8; REGISTER_COUNT],
}

impl Emulator{
    pub fn hardware_setup(&mut self){
        self.physical_memory.fill(0);
        self.registers.fill(0);
    }
    pub fn cpu_cycle(&mut self){
    }
}