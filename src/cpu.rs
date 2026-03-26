use crate::{MEM_SIZE, PC_REGISTER, REGISTER_COUNT, ROM_SIZE, get_nibble_from_byte};
use bevy::prelude::Resource;
use std::fs;

#[derive(Resource)]
pub struct Emulator {
    pub physical_memory: [u8; MEM_SIZE],
    pub registers: [u8; REGISTER_COUNT],
    pub rom_disk: [u8; ROM_SIZE],
}

impl Emulator {
    pub fn hardware_setup() -> Emulator {
        Emulator {
            physical_memory: [0; MEM_SIZE],
            registers: [0; REGISTER_COUNT],
            rom_disk: [0; ROM_SIZE],
        }
    }

    pub fn boot(&mut self, disk_path: String) {
        self.install_rom_disk(disk_path);
        self.load_rom_to_memory(0x0000, self.rom_disk.len() as u16);
    }

    pub fn cpu_cycle(&mut self) {
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

    pub fn load_rom_to_memory(&mut self, start_address: u16, amount: u16) {
        let start = start_address as usize;
        let end = start + amount as usize;
        let mut rom_idx = 0;
        for i in start..end {
            let byte = self.rom_disk[rom_idx];
            self.physical_memory[i] = byte;
            rom_idx += 1;
        }
    }
}

