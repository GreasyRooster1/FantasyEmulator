use crate::{MEM_SIZE, PC_REGISTER, REGISTER_COUNT, ROM_SIZE, get_nibble_from_byte};
use bevy::prelude::Resource;
use std::fs;
use std::fs::File;
use std::io::Write;
use crate::instructions::*;

const DBG_MEM_REPORT_FILE: &str = "./mem_dump.log";
const DBG_ROM_REPORT_FILE: &str = "./rom_dump.log";

const SCREEN_BUF_START: usize = 0x0;
const SCREEN_BUF_LENGTH: usize = 8192;

#[derive(Resource, Debug)]
pub struct Emulator {
    pub physical_memory: [u8; MEM_SIZE],
    pub registers: [u8; REGISTER_COUNT],
    pub rom_disk: [u8; ROM_SIZE],

    is_running: bool,
}

impl Emulator {
    pub fn hardware_setup() -> Emulator {
        Emulator {
            physical_memory: [0; MEM_SIZE],
            registers: [0; REGISTER_COUNT],
            rom_disk: [0; ROM_SIZE],
            is_running: false,
        }
    }

    pub fn boot(&mut self, disk_path: String) {
        self.install_rom_disk(disk_path);
        self.load_rom_to_memory(0x0000, self.rom_disk.len() as u16);
        self.is_running = true;
    }

    pub fn cpu_cycle(&mut self) {
        let pc_memory_value = self.physical_memory[self.registers[PC_REGISTER] as usize];
        let opcode_nibble = pc_memory_value>>4;
        let instruction = self.match_instruction_opcode(opcode_nibble);

        let instruction_len = instruction.bytes_len();
        let mut instruction_data: Vec<u8> = vec![];
        for i in 0..instruction.bytes_len() {
            let pc = self.registers[PC_REGISTER] as usize;
            let val = self.physical_memory[(pc+i as usize) % MEM_SIZE];
            instruction_data.push(val);
        }

        let args = InstructionArgs::from_bytes(instruction_data);
        self.registers[PC_REGISTER] += instruction_len;
        instruction.execute(self,args);
    }

    pub fn match_instruction_opcode(&self, opcode: u8) -> Box<dyn Instruction>{
        match opcode {
            0b0000 => Box::new(NOP),
            0b0001 => Box::new(ADD),
            0b0010 => Box::new(SUB),
            0b0011 => Box::new(MUL),
            0b0100 => Box::new(DIV),
            0b0101 => Box::new(NOP),  //placeholder
            0b0110 => Box::new(REM),
            0b0111 => Box::new(NOT),
            0b1000 => Box::new(AND),
            0b1001 => Box::new(OR),
            0b1010 => Box::new(XOR),
            0b1011 => Box::new(PEEK),
            0b1100 => Box::new(POKE),
            0b1101 => Box::new(LODI),
            0b1110 => Box::new(BRANCH),
            0b1111 => Box::new(HALT),

            _ => {println!("Unknown opcode {:#b}", opcode); Box::new(NOP)},
        }
    }

    pub fn halt(&mut self){
        self.is_running = false;
    }
    pub fn running(&self) -> bool{
        self.is_running
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

    pub fn debug_memory_report(&self){
        println!("starting debug memory report...");
        let mut file = File::create(DBG_MEM_REPORT_FILE).unwrap();
        file.write_all(&self.physical_memory).unwrap();
        println!("finished debug memory report at {DBG_MEM_REPORT_FILE}");
    }
    pub fn debug_rom_report(&self){
        println!("starting debug rom report...");
        let mut file = File::create(DBG_ROM_REPORT_FILE).unwrap();
        file.write_all(&self.rom_disk).unwrap();
        println!("finished debug rom report at {DBG_ROM_REPORT_FILE}");
    }
}

