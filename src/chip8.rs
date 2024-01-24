//! ## This emulates a CHIP-8
//! ### Links
//! - [Chip-8 wikipedia](https://en.wikipedia.org/wiki/CHIP-8)
//!
//! ### Memory layout
//! - 4K of memory -> address space from 0x0000 -> 0x1000
//! - interpreter is at       : 0x0000 -> 0x01FF = 512 bytes
//! - programs starts at      : 0x0200 -> 0x0E9F = 3232 bytes
//! - call stack at           : 0x0EA0 -> 0x0EFF = 96 bytes
//! - used for display refresh: 0x0F00 -> 0x0FFF = 256 bytes
//!
//! As our interpreter is running natively outside the 4K memory we will
//! use the lower 512 bytes to store font data.
//!
//! ### Registers, stack and timers
//! #### Registers
//! - It has 16 u8 registers from V0 -> VF
//!     - VF is also used as flag for some instructions
//! - I: address register (12 bits) involved in memory operations
//! #### Stack
//! - use to store return addresses when subroutines are called
//! #### Timers
//! - It has two timers that count downs at 60 Hz until reach 0
//!     - Delay timer;
//!     - Sound timer;
//!
//! ### Input
//! - Done with an hex keyboard that has 16 keys
//!
//! ### Graphics and sound
//! - Display is 64x32 pixels and monochrome
//! - Graphics are drawn using sprites
//!     - sprites is 8 wide and 1->15 pixels height
//!     - sprites are XOR'ed with corresponding screen pixels
//! - A beeping sound is played when sound timer is nonzero.

use std::{fs::File, io::Read};

pub struct Chip8 {
    pc: u16,
    mem: [u8; 4096],
}

impl Chip8 {
    pub fn new(rom_name: &str) -> Self {
        let mut chip = Chip8 {
            pc: 0x200, // Entry point of our code
            mem: [0; 4096],
        };
        let mut opcode = [0; 4]; // opcode is 4 bytes
        let mut pc = chip.pc as usize;

        let mut f = File::open(rom_name).unwrap();
        while let Ok(()) = f.read_exact(&mut opcode) {
            // Read 4 bytes by 4 bytes
            println!(
                "{:#06X}: {:#04x} {:#04x} {:#04x} {:#04x}",
                pc, opcode[0], opcode[1], opcode[2], opcode[3]
            );
            chip.mem[pc] = opcode[0];
            pc += 1;
            chip.mem[pc] = opcode[1];
            pc += 1;
            chip.mem[pc] = opcode[2];
            pc += 1;
            chip.mem[pc] = opcode[3];
            pc += 1;
            if pc == 0x0E9F {
                println!("Memory is full");
                break;
            }
        }

        chip
    }
}
