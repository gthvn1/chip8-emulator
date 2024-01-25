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

/// Chip8 has 4Ko of RAM
const MEMSIZE: usize = 4096;
/// Fonts are loaded at offset 0x0
const FONTS_OFFSET: usize = 0x0;
/// Fonts are 8x5 (5 bytes) and from 0x0 to 0xE
const FONTS_SIZE: usize = 80;
/// Stack offset
const STACK_OFFSET: usize = 0x0EA0;
/// Stack size is 96 bytes
const STACK_SIZE: usize = 96;
/// Display offset
const DISPLAY_OFFSET: usize = 0xF00;
/// Display size is 256 bytes
const DISPLAY_SIZE: usize = 256;
/// 16 Data registers named V0 to VF
const VREGS_SIZE: usize = 16;

#[allow(dead_code)]
pub struct Opcode {
    /// Address
    nnn: u16,
    /// 8-bit constant
    nn: u8,
    /// 4-bit constant
    n: u8,
    /// X register identifier (it is a 4-bit register)
    x: u8,
    /// Y register identifier (it is a 4-bit register)
    y: u8,
}

pub struct Chip8 {
    /// 4K memory
    mem: [u8; MEMSIZE],
    /// program counter
    pc: u16,
    /// Data registers from V0 to VF
    vregs: [u8; VREGS_SIZE],
    /// 16-bit register for memory address
    i: u16,
}

impl Chip8 {
    /// Loads in memory the `rom` passed as a parameter.
    /// The `rom` must be a file that contains a valid ROM.
    /// There is no check done when loading it.
    pub fn new(rom: &str) -> Self {
        let mut chip = Chip8 {
            mem: [0; MEMSIZE],
            pc: 0x200, // Entry point of our code
            vregs: [0; VREGS_SIZE],
            i: 0,
        };

        let mut opcode = [0; 4]; // opcode is 4 bytes
        let mut pc = chip.pc as usize;

        let mut f = File::open(rom).unwrap();
        while let Ok(()) = f.read_exact(&mut opcode) {
            if pc >= (0x0EA0 - 4) {
                println!("Memory is full");
                break;
            }
            chip.mem[pc] = opcode[0];
            chip.mem[pc + 1] = opcode[1];
            chip.mem[pc + 2] = opcode[2];
            chip.mem[pc + 3] = opcode[3];
            pc += 4;
        }

        // Load the fonts
        chip.mem[FONTS_OFFSET..(FONTS_OFFSET + FONTS_SIZE)].copy_from_slice(&[
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ]);

        chip
    }

    /// Return the memory where fonts are loaded
    pub fn fonts(&self) -> &[u8] {
        &self.mem[FONTS_OFFSET..(FONTS_OFFSET + FONTS_SIZE)]
    }

    /// Return the memory where stack is located
    pub fn stack(&self) -> &[u8] {
        &self.mem[STACK_OFFSET..(STACK_OFFSET + STACK_SIZE)]
    }

    /// Return the memory where display is located
    pub fn display(&self) -> &[u8] {
        &self.mem[DISPLAY_OFFSET..(DISPLAY_OFFSET + DISPLAY_SIZE)]
    }

    /// Emulate the instruction at program counter.
    pub fn emulate(&mut self) {
        let _ = self.vregs; // TODO: use it for real
        let _ = self.i; // TODO: use it for real
        todo!()
    }

    /// Dumps the content of all memory on stdin.
    pub fn dump_memory(&self) {
        for (i, byte) in self.mem.iter().enumerate() {
            if i % 0x10 == 0 {
                print!("\n{i:#06X}: ");
            }
            print!("{byte:#04x} ");
        }
        println!();
    }
}
