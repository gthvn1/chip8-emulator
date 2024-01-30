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
mod opcode;

use opcode::Opcode;
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
/// Opcode is 2 bytes
const OPCODE_SIZE: usize = 2;

pub enum Chip8Error {
    NotImplemented,
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

        // We can read byte per byte
        let mut byte: [u8; 1] = [0];
        let mut pc = chip.pc as usize;

        let mut f = File::open(rom).unwrap();
        while let Ok(()) = f.read_exact(&mut byte) {
            if pc >= 0x0EA0 {
                println!("Memory is full");
                break;
            }
            chip.mem[pc] = byte[0];
            pc += 1;
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

        // Write FF in display so we will be able to check that clean Display
        // is working
        chip.mem[DISPLAY_OFFSET..(DISPLAY_OFFSET + DISPLAY_SIZE)]
            .copy_from_slice(&[0xFF; DISPLAY_SIZE]);

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
    pub fn framebuffer(&self) -> &[u8] {
        &self.mem[DISPLAY_OFFSET..(DISPLAY_OFFSET + DISPLAY_SIZE)]
    }

    /// Emulate the instruction at program counter.
    /// Currently we are returning false for opcode that are not yet emulated
    /// but it is for testing.
    pub fn emulate_one_insn(&mut self) -> Result<(), Chip8Error> {
        let _ = self.vregs; // TODO: use it for real
        let _ = self.i; // TODO: use it for real

        // Save the old PC before updating it
        let pc = self.pc as usize;
        self.pc += OPCODE_SIZE as u16;

        let opcode = Opcode::new(u16::from_be_bytes(
            self.mem[pc..pc + OPCODE_SIZE].try_into().unwrap(),
        ));

        println!("[debug] emulate {opcode}");

        match opcode.upper4() {
            0x0 => {
                if opcode.value() == 0x00E0 {
                    // clear screen
                    self.mem[DISPLAY_OFFSET..(DISPLAY_OFFSET + DISPLAY_SIZE)]
                        .copy_from_slice(&[0; DISPLAY_SIZE]);
                } else if opcode.value() == 0x00EE {
                    return Err(Chip8Error::NotImplemented);
                } else {
                    return Err(Chip8Error::NotImplemented);
                }
            }
            0x1 => self.pc = opcode.nn() as u16,

            0x2 => return Err(Chip8Error::NotImplemented),
            0x3 => return Err(Chip8Error::NotImplemented),
            0x4 => return Err(Chip8Error::NotImplemented),
            0x5 => return Err(Chip8Error::NotImplemented),
            0x6 => {
                let idx = opcode.x() as usize;
                self.vregs[idx] = opcode.nn();
            }
            0x7 => {
                let idx = opcode.x() as usize;
                self.vregs[idx] += opcode.nn();
            }
            0x8 => return Err(Chip8Error::NotImplemented),
            0x9 => return Err(Chip8Error::NotImplemented),
            0xA => self.i = opcode.nnn(),
            0xB => return Err(Chip8Error::NotImplemented),
            0xC => return Err(Chip8Error::NotImplemented),
            0xD => {
                // Draw a sprite 8xN at coordinate (VX, VY)
                let x = self.vregs[opcode.x() as usize] as usize;
                let y = self.vregs[opcode.y() as usize] as usize;
                let n = opcode.n() as usize;
                let sprite = &self.mem[self.i as usize..(self.i as usize + n)];
                let mut fb = self.framebuffer().to_vec();

                assert!(x + 8 < 64); // We have 8 bytes for a line (64 pixels)
                assert!(y + n < 32); // We have at most 32 lines

                // We have 8 pixels per line
                for (idx, pixels) in sprite.iter().enumerate() {
                    let x = x / 8; // Transform pixel to bytes
                    let offset = x + x * (y + idx);
                    println!("(x:{x}, y:{y}), idx:{idx}, {pixels:?} -> @ {offset}");
                    fb[offset] = *pixels;
                }
                if self.framebuffer().to_vec() == fb {
                    self.vregs[0xF] = 0;
                } else {
                    self.mem[DISPLAY_OFFSET..(DISPLAY_OFFSET + DISPLAY_SIZE)].copy_from_slice(&fb);
                    self.vregs[0xF] = 1;
                }
            }
            0xE => return Err(Chip8Error::NotImplemented),
            0xF => return Err(Chip8Error::NotImplemented),
            _ => unreachable!(),
        };

        Ok(())
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
