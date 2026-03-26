use std::fs;
use bevy::prelude::Resource;
use crate::{get_nibble_from_byte, MEM_SIZE, PC_REGISTER, REGISTER_COUNT, ROM_SIZE};

#[derive(Resource)]
pub struct Emulator {
    physical_memory: [u8; MEM_SIZE],
    registers: [u8; REGISTER_COUNT],
    rom_disk: [u8; ROM_SIZE],
}

impl Emulator{
    pub fn hardware_setup() -> Emulator {
        Emulator {
            physical_memory: [0; MEM_SIZE],
            registers: [0; REGISTER_COUNT],
            rom_disk: [0; ROM_SIZE],
        }
    }
    pub fn cpu_cycle(&mut self){
        let pc_memory_value = self.physical_memory[self.registers[PC_REGISTER] as usize];
        let opcode_nibble = get_nibble_from_byte(pc_memory_value as u32, 0);
        println!("{:#b}", opcode_nibble);
    }
    pub fn install_rom_disk(&mut self, path: String) {
        let bytes: Vec<u8> = fs::read(path).unwrap();
        for i in 0..bytes.len() {
            let byte = bytes[i];
            self.rom_disk[i] = byte;
        }
    }
}