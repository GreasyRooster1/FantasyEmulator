use crate::{MEM_SIZE, REGISTER_COUNT, ROM_SIZE, get_nibble_from_byte, PC_REGISTER};
use bevy::prelude::Resource;
use std::fs;
use std::fs::File;
use std::io::Write;
use crate::instructions::*;

const DBG_MEM_REPORT_FILE: &str = "./mem_dump.log";
const DBG_ROM_REPORT_FILE: &str = "./rom_dump.log";

pub const SCREEN_BUF_START: usize = 0x7000;
pub const SCREEN_BUF_LENGTH: usize = 0x2000; //8192

#[derive(Resource, Debug)]
pub struct Emulator {
    pub physical_memory: [u8; MEM_SIZE],
    pub registers: [i32; REGISTER_COUNT],
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
        self.registers[PC_REGISTER] += instruction_len as i32;
        instruction.execute(self,args);
    }

    pub fn match_instruction_opcode(&self, opcode: u8) -> Box<dyn Instruction>{
        match opcode {
            0b0000_0000 => Box::new(NOP),

            0b0001_0000 => Box::new(ADD),
            0b0001_0001 => Box::new(ADDI),
            0b0001_0010 => Box::new(SUB),
            0b0001_0011 => Box::new(SUBI),
            0b0001_0100 => Box::new(MUL),
            0b0001_0101 => Box::new(MULI),
            0b0001_0110 => Box::new(DIV),
            0b0001_0111 => Box::new(DIVI),
            0b0001_1000 => Box::new(MOD),
            0b0001_1001 => Box::new(MODI),

            0b0010_0000 => Box::new(AND),
            0b0010_0001 => Box::new(ANDI),
            0b0010_0010 => Box::new(OR),
            0b0010_0011 => Box::new(ORI),
            0b0010_0100 => Box::new(XOR),
            0b0010_0101 => Box::new(XORI),
            0b0010_0110 => Box::new(NOT),
            0b0010_0111 => Box::new(RSH),
            0b0010_1000 => Box::new(RSHI),
            0b0010_1001 => Box::new(LSH),
            0b0010_1010 => Box::new(LSHI),

            0b0011_0000 => Box::new(STOW),
            0b0011_0001 => Box::new(STOH),
            0b0011_0010 => Box::new(STOB),
            0b0011_0011 => Box::new(NOP), //placeholder
            0b0011_0100 => Box::new(LODW),
            0b0011_0101 => Box::new(LODH),
            0b0011_0110 => Box::new(LODB),
            0b0011_0111 => Box::new(LODI),

            0b0100_0000 => Box::new(JMP),
            0b0100_0001 => Box::new(BREQ),
            0b0100_0010 => Box::new(BRNEQ),
            0b0100_0011 => Box::new(BRGT),
            0b0100_0100 => Box::new(BRGTE),
            0b0100_0101 => Box::new(BRLT),
            0b0100_0110 => Box::new(BRLTE),
            0b0100_0111 => Box::new(BREZ),
            0b0100_1000 => Box::new(BRNEZ),
            0b0100_1001 => Box::new(CALL),
            0b0100_1010 => Box::new(RET),

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

