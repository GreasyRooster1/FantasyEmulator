mod instructions;

use bevy::prelude::*;

const MEM_SIZE: usize = 65_535;

#[derive(Resource)]
struct Emulator {
    physical_memory: [u8; MEM_SIZE],
    registers: Registers,
}

struct Registers {
    r0: u8,

    r1: u8,
    r2: u8,
    r3: u8,
    r4: u8,
    r5: u8,
    r6: u8,
    r7: u8,
    r8: u8,
    r9: u8,
    r10: u8,
    r11: u8,
    r12: u8,
    r13: u8,

    program_counter: u8,
    return_address: u8,
}

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}
