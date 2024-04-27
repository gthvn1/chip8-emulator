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

use log;
use opcode::Opcode;
use std::{fs::File, io::Read};

use crate::framebuffer::Framebuffer;

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
/// Display Resolution
const RESOLUTION: (usize, usize) = (64, 32);

#[derive(Debug)]
pub enum Chip8Error {
    NotImplemented,
    MemoryFull,
    UnknownOpcode,
}

pub struct Chip8 {
    /// 4K memory
    mem: [u8; MEMSIZE],
    /// program counter
    pc: usize,
    /// Data registers from V0 to VF
    vregs: [u8; VREGS_SIZE],
    /// 16-bit register for memory address
    i: u16,
    fb: Framebuffer,
}

impl Default for Chip8 {
    fn default() -> Self {
        Self::new()
    }
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            mem: [0; MEMSIZE],
            pc: 0x200, // Entry point of our code
            vregs: [0; VREGS_SIZE],
            i: 0,
            fb: Framebuffer::new(RESOLUTION.0, RESOLUTION.1),
        }
    }

    /// Loads in memory the `rom` passed as a parameter.
    /// The `rom` must be a file that contains a valid ROM.
    /// There is no check done when loading it.
    pub fn load(&mut self, from: &str) -> Result<(), Chip8Error> {
        // We can read byte per byte
        let mut byte: [u8; 1] = [0];

        // We don't want to change the PC so don't use self.pc to load
        // the program
        let mut pc = self.pc;
        let mut f = File::open(from).unwrap();

        while let Ok(()) = f.read_exact(&mut byte) {
            if pc >= 0x0EA0 {
                eprintln!("Memory is full");
                return Err(Chip8Error::MemoryFull);
            }
            self.mem[pc] = byte[0];
            pc += 1;
        }

        // Load the fonts at FONTS_OFFSET
        self.mem[FONTS_OFFSET..(FONTS_OFFSET + FONTS_SIZE)].copy_from_slice(&[
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

        // Write 0xFF in display so we will be able to check that clean Display
        // is working.
        self.mem[DISPLAY_OFFSET..(DISPLAY_OFFSET + DISPLAY_SIZE)]
            .copy_from_slice(&[0xFF; DISPLAY_SIZE]);

        Ok(())
    }

    /// Return the memory where fonts are loaded
    pub fn fonts(&self) -> &[u8] {
        &self.mem[FONTS_OFFSET..(FONTS_OFFSET + FONTS_SIZE)]
    }

    /// Return the memory where stack is located
    pub fn stack(&self) -> &[u8] {
        &self.mem[STACK_OFFSET..(STACK_OFFSET + STACK_SIZE)]
    }

    /// Return a copy of memory related to display
    pub fn framebuffer(&self) -> Vec<u8> {
        let mut buf = vec![0; DISPLAY_SIZE];
        buf.copy_from_slice(&self.mem[DISPLAY_OFFSET..(DISPLAY_OFFSET + DISPLAY_SIZE)]);
        buf
    }

    /// Emulate the instruction at program counter.
    /// Currently we are returning false for opcode that are not yet emulated
    /// but it is for testing.
    pub fn emulate_one_insn(&mut self) -> Result<(), Chip8Error> {
        let _ = self.vregs; // TODO: use it for real
        let _ = self.i; // TODO: use it for real

        let opcode = Opcode::new(u16::from_be_bytes(
            self.mem[self.pc..self.pc + OPCODE_SIZE].try_into().unwrap(),
        ));

        log::debug!("pc = {:#06x}, opcode = {}", self.pc, opcode);

        self.pc += OPCODE_SIZE;

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
            0x1 => self.pc = opcode.nnn() as usize,
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
                // VX and VY are in pixels
                let vx = self.vregs[opcode.x() as usize] as usize;
                let vy = self.vregs[opcode.y() as usize] as usize;
                let n = opcode.n() as usize;

                println!("Draw a 8x{n} sprite at ({vx}, {vy})");

                let sprite = &self.mem[self.i as usize..(self.i as usize + n)];
                println!("Sprite is {sprite:?}");

                let mut fb = self.framebuffer().to_vec();

                assert!(vx + 8 < 64); // We have 8 bytes for a line (64 pixels)
                assert!(vy + n < 32); // We have at most 32 lines

                // We have 8 pixels per line
                for (idx, pixels) in sprite.iter().enumerate() {
                    let x = vx / 8; // Transform pixel to bytes
                    let offset = x + x * (vy + idx);
                    //println!("(x:{x}, y:{vy}), idx:{idx}, {pixels:?} -> @ {offset}");
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
            _ => {
                eprintln!("unknown opcode: {opcode}");
                return Err(Chip8Error::UnknownOpcode);
            }
        };

        Ok(())
    }

    pub fn run(&mut self) {
        loop {
            if self.emulate_one_insn().is_err() {
                eprint!("failed to emulate instruction\n");
                break;
            }

            // Draw frame buffer
            self.fb.draw(&self.framebuffer());
        }
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
