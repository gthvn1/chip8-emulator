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

use crate::chip8::opcode;
use log;
use std::{fmt, fs::File, io::Read};

/// Chip8 has 4Ko of RAM
const MEMSIZE: usize = 4096;
/// System begin at memory location 512
const ENTRY_POINT: usize = 0x200;
/// Fonts are loaded at offset 0x0
const FONTS_OFFSET: usize = 0x0;
/// Fonts are 8x5 (5 bytes) and from 0x0 to 0xF
/// There are 16 fonts
const _FONTS_WIDTH: usize = 8;
const FONTS_HEIGHT: usize = 5;
const FONTS_SIZE: usize = 80;
/// Stack offset
const STACK_OFFSET: usize = 0x0EA0;
/// Stack size is 96 bytes
const STACK_SIZE: usize = 96;
/// Display offset
const DISPLAY_OFFSET: usize = 0xF00;
/// Display width in pixels
const DISPLAY_WIDTH: usize = 64;
/// Display height in pixels
const DISPLAY_HEIGHT: usize = 32;
/// Display size is 256 bytes
const DISPLAY_SIZE: usize = (DISPLAY_WIDTH * DISPLAY_HEIGHT) / 8;
/// 16 Data registers named V0 to VF
const VREGS_SIZE: usize = 16;
/// Opcode is 2 bytes
const OPCODE_SIZE: usize = 2;
/// Keyboard has 16 values from 0 to F
const KEYBOARD_SIZE: usize = 16;

pub enum Chip8Error {
    NotImplemented(opcode::Opcode),
    UnknownOpcode(opcode::Opcode),
    StackOverflow,
    StackUnderflow,
    VregsOverflow,
    MemoryFull,
    WrongKey,
    UndefinedHexadecimal(usize),
}

impl fmt::Display for Chip8Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Chip8Error::NotImplemented(opcode) => write!(f, "Opcode <{opcode}> is not implemented"),
            Chip8Error::UnknownOpcode(opcode) => write!(f, "Opcode <{opcode}> is unknown"),
            Chip8Error::StackOverflow => write!(f, "Stack overflow detected"),
            Chip8Error::StackUnderflow => write!(f, "Stack underflow detected"),
            Chip8Error::VregsOverflow => write!(f, "Vregs overflow detected"),
            Chip8Error::MemoryFull => write!(f, "Memory is full"),
            Chip8Error::WrongKey => write!(f, "Key is not valid"),
            Chip8Error::UndefinedHexadecimal(v) => {
                write!(f, "Hexadecimal error: Expected a value under 16, got {v}")
            }
        }
    }
}

impl fmt::Debug for Chip8Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

pub struct Chip8 {
    /// 4K memory
    mem: [u8; MEMSIZE],
    /// program counter
    pc: usize,
    /// stack pointer
    sp: usize,
    /// Data registers from V0 to VF
    vregs: [u8; VREGS_SIZE],
    /// 16-bit register for memory address
    i: u16,
    // delay timer
    delay_timer: u16,
    // sound timer
    sound_timer: u16,
    // Keyboard status, true means key is pressed
    keyboard: [bool; KEYBOARD_SIZE],
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
            pc: ENTRY_POINT,
            sp: STACK_OFFSET,
            vregs: [0; VREGS_SIZE],
            i: 0,
            delay_timer: 0,
            sound_timer: 0,
            keyboard: [false; KEYBOARD_SIZE],
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

    /// Return a reference to memory related to display
    pub fn get_framebuffer(&self) -> &[u8] {
        &self.mem[DISPLAY_OFFSET..(DISPLAY_OFFSET + DISPLAY_SIZE)]
    }

    /// Return a copy of memory related to display
    pub fn get_copy_of_framebuffer(&self) -> Vec<u8> {
        let mut buf = vec![0; DISPLAY_SIZE];
        buf.copy_from_slice(&self.mem[DISPLAY_OFFSET..(DISPLAY_OFFSET + DISPLAY_SIZE)]);
        buf
    }

    /// Emulate the instruction at program counter.
    /// Currently we are returning false for opcode that are not yet emulated
    /// but it is for testing.
    pub fn emulate_insn(&mut self) -> Result<(), Chip8Error> {
        let opcode = opcode::Opcode::new(u16::from_be_bytes(
            self.mem[self.pc..self.pc + OPCODE_SIZE].try_into().unwrap(),
        ));

        log::debug!("pc = {:#06x}, opcode = {}", self.pc, opcode);

        self.pc += OPCODE_SIZE;

        // The emulate insn is called at 60 FPS so we can update timer here
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }

        match opcode.per_4bits() {
            // clear screen
            (0x0, 0x0, 0xE, 0x0) => {
                self.mem[DISPLAY_OFFSET..(DISPLAY_OFFSET + DISPLAY_SIZE)]
                    .copy_from_slice(&[0; DISPLAY_SIZE]);
            }
            // return from subroutine
            (0x0, 0x0, 0xE, 0xE) => {
                if self.sp == STACK_OFFSET {
                    return Err(Chip8Error::StackUnderflow);
                }

                // Read PC from the top of the stack
                let pc_hi = self.mem[self.sp] as usize;
                let pc_low = self.mem[self.sp + 1] as usize;

                self.pc = (pc_hi << 8) | pc_low;

                // Decrement the stack pointer
                self.sp -= 2;
            }
            // Jump to addr
            (0x1, _, _, _) => self.pc = opcode.nnn() as usize,
            // Call addr
            (0x2, _, _, _) => {
                self.sp += 2; // Increment stack pointer
                if self.sp >= STACK_OFFSET + STACK_SIZE {
                    return Err(Chip8Error::StackOverflow);
                }

                // Save PC
                let pc_hi: u8 = (self.pc >> 8) as u8;
                let pc_low: u8 = self.pc as u8;

                self.mem[self.sp] = pc_hi;
                self.mem[self.sp + 1] = pc_low;

                // Set the new PC
                self.pc = opcode.nnn() as usize;
            }
            // SE Vx, byte
            (0x3, x, _, _) => {
                if x >= VREGS_SIZE {
                    return Err(Chip8Error::VregsOverflow);
                }

                if self.vregs[x] == opcode.nn() {
                    self.pc += OPCODE_SIZE;
                }
            }
            // SNE Vx, byte
            (0x4, x, _, _) => {
                if x >= VREGS_SIZE {
                    return Err(Chip8Error::VregsOverflow);
                }

                if self.vregs[x] != opcode.nn() {
                    self.pc += OPCODE_SIZE;
                }
            }
            // SE Vx, Vy
            (0x5, x, y, 0x0) => {
                if x >= VREGS_SIZE || y >= VREGS_SIZE {
                    return Err(Chip8Error::VregsOverflow);
                }

                if self.vregs[x] == self.vregs[y] {
                    self.pc += OPCODE_SIZE;
                }
            }
            // LD Vx, byte
            (0x6, x, _, _) => {
                if x >= VREGS_SIZE {
                    return Err(Chip8Error::VregsOverflow);
                }
                self.vregs[x] = opcode.nn();
            }
            // ADD Vx, byte
            (0x7, x, _, _) => {
                if x >= VREGS_SIZE {
                    return Err(Chip8Error::VregsOverflow);
                }
                //self.vregs[x] += opcode.nn();
                self.vregs[x] = (self.vregs[x] as usize + opcode.nn() as usize) as u8;
            }
            // LD Vx, Vy
            (0x8, x, y, 0x0) => {
                if x >= VREGS_SIZE || y >= VREGS_SIZE {
                    return Err(Chip8Error::VregsOverflow);
                }
                self.vregs[x] = self.vregs[y];
            }
            // OR Vx, Vy
            (0x8, x, y, 0x1) => {
                if x >= VREGS_SIZE || y >= VREGS_SIZE {
                    return Err(Chip8Error::VregsOverflow);
                }
                self.vregs[x] |= self.vregs[y];
            }
            // AND Vx, Vy
            (0x8, x, y, 0x2) => {
                if x >= VREGS_SIZE || y >= VREGS_SIZE {
                    return Err(Chip8Error::VregsOverflow);
                }
                self.vregs[x] &= self.vregs[y];
            }
            // XOR Vx, Vy
            (0x8, x, y, 0x3) => {
                if x >= VREGS_SIZE || y >= VREGS_SIZE {
                    return Err(Chip8Error::VregsOverflow);
                }
                self.vregs[x] ^= self.vregs[y];
            }
            // ADD Vx, Vy
            (0x8, x, y, 0x4) => {
                if x >= VREGS_SIZE || y >= VREGS_SIZE {
                    return Err(Chip8Error::VregsOverflow);
                }
                let sum = self.vregs[x] as usize + self.vregs[y] as usize;
                self.vregs[0xF] = if sum > 255 { 1 } else { 0 };
                self.vregs[x] = sum as u8;
            }
            // SUB Vx, Vy
            (0x8, x, y, 0x5) => {
                if x >= VREGS_SIZE || y >= VREGS_SIZE {
                    return Err(Chip8Error::VregsOverflow);
                }

                self.vregs[0xF] = if self.vregs[x] > self.vregs[y] { 1 } else { 0 };
                self.vregs[x] = (self.vregs[x] as isize - self.vregs[y] as isize) as u8;
            }
            // SHR Vx {, Vy}
            (0x8, x, y, 0x6) => {
                if x >= VREGS_SIZE || y >= VREGS_SIZE {
                    return Err(Chip8Error::VregsOverflow);
                }

                self.vregs[0xF] = if self.vregs[x] & 0x1 == 0x1 { 1 } else { 0 };
                self.vregs[x] /= 2;
            }
            // SUBN Vx, Vy
            (0x8, x, y, 0x7) => {
                if x >= VREGS_SIZE || y >= VREGS_SIZE {
                    return Err(Chip8Error::VregsOverflow);
                }
                self.vregs[0xF] = if self.vregs[y] > self.vregs[x] { 1 } else { 0 };
                self.vregs[x] = self.vregs[y] - self.vregs[x];
            }
            // SHL Vx {, Vy}
            (0x8, x, y, 0xE) => {
                if x >= VREGS_SIZE || y >= VREGS_SIZE {
                    return Err(Chip8Error::VregsOverflow);
                }
                self.vregs[0xF] = if self.vregs[x] & 0x80 == 0x80 { 1 } else { 0 };
                self.vregs[x] *= 2;
            }
            // SNE Vx, Vy
            (0x9, x, y, 0x0) => {
                if x >= VREGS_SIZE || y >= VREGS_SIZE {
                    return Err(Chip8Error::VregsOverflow);
                }

                if self.vregs[x] != self.vregs[y] {
                    self.pc += OPCODE_SIZE;
                }
            }
            // LD I, addr
            (0xA, _, _, _) => self.i = opcode.nnn(),
            // Vx = rand() & NN
            (0xC, x, _, _) => {
                if x >= VREGS_SIZE {
                    return Err(Chip8Error::VregsOverflow);
                }

                let rand = unsafe {
                    let mut r = 0_u16;
                    if core::arch::x86_64::_rdrand16_step(&mut r) == 0 {
                        log::warn!("failed to generate random number");
                    };

                    r as u8
                };
                self.vregs[x] = rand & opcode.nn();
            }
            // DRAW Vx, Vy, nibble
            (0xD, x, y, n) => {
                // Draw a sprite 8xN at coordinate (VX, VY)
                // VX and VY are in pixels
                let mut vx = 0;
                let mut vy = 0;

                self.check_vregs(x, &mut vx)?;
                self.check_vregs(y, &mut vy)?;

                log::debug!("Draw a 8x{n} sprite at ({vx}, {vy})");

                let sprite = &self.mem[self.i as usize..(self.i as usize + n)];
                log::debug!("Sprite is {sprite:?}");

                self.vregs[0xF] = 0; // Will be set if a pixel is set from set to unset

                // We need to use a copy of the framebuffer because sprite has an immutable
                // borrow on self.mem.
                let mut fb_copy = self.get_copy_of_framebuffer();
                let fb_origin = fb_copy.clone(); // Keep a copy to check if a pixel has been set

                for (idx, pixels) in sprite.iter().enumerate() {
                    log::debug!("  idx {idx}, pixels {pixels}");
                    // We need to find in which coordinate the pixel falls. Pixel 0-7 are in first
                    // byte, 8-15 in the second and so on.
                    let start_idx = vx / 8;
                    let end_idx = (vx + 7) / 8;
                    let offset = vx % 8;

                    if offset == 0 {
                        // It is aligned so easy because 8 bits fall into the same bucket in frame
                        // buffer.
                        fb_copy[start_idx + ((vy + idx) * 8)] ^= pixels;
                    } else {
                        // It is not aligned so we need to shift pixels at the right place.
                        fb_copy[start_idx + ((vy + idx) * 8)] ^= pixels >> offset;
                        fb_copy[end_idx + ((vy + idx) * 8)] ^= pixels << (8 - offset);
                    }
                }

                if fb_origin != fb_copy {
                    // At least one bit has been set
                    self.vregs[0xF] = 1;
                    // Update the real framebuffer
                    self.mem[DISPLAY_OFFSET..(DISPLAY_OFFSET + DISPLAY_SIZE)]
                        .copy_from_slice(&fb_copy);
                }
            }
            // SKP Vx
            (0xE, x, 0x9, 0xE) => {
                let mut key = 0;
                self.check_vregs(x, &mut key)?;

                if key >= KEYBOARD_SIZE {
                    return Err(Chip8Error::WrongKey);
                }

                if self.keyboard[key] {
                    self.pc += OPCODE_SIZE;
                }
            }
            // SKNP Vx
            (0xE, x, 0xA, 0x1) => {
                // skip next instruction if key is NOT pressed
                let mut key = 0;
                self.check_vregs(x, &mut key)?;

                if key >= KEYBOARD_SIZE {
                    return Err(Chip8Error::WrongKey);
                }

                if !self.keyboard[key] {
                    self.pc += OPCODE_SIZE;
                }
            }
            // LD Vx, DT
            (0xF, x, 0x0, 0x7) => {
                if x >= VREGS_SIZE {
                    return Err(Chip8Error::VregsOverflow);
                }

                self.vregs[x] = self.delay_timer as u8;
            }
            // LD DT, Vx
            (0xF, x, 0x1, 0x5) => {
                if x >= VREGS_SIZE {
                    return Err(Chip8Error::VregsOverflow);
                }

                let vx = self.vregs[x] as u16;
                self.delay_timer = vx;
            }
            // LD ST, Vx
            (0xF, x, 0x1, 0x8) => {
                if x >= VREGS_SIZE {
                    return Err(Chip8Error::VregsOverflow);
                }

                let vx = self.vregs[x] as u16;
                self.sound_timer = vx;
            }
            // ADD I, Vx
            (0xF, x, 0x1, 0xE) => {
                if x >= VREGS_SIZE {
                    return Err(Chip8Error::VregsOverflow);
                }

                self.i += self.vregs[x] as u16;
            }
            // LD F, Vx
            (0xF, x, 0x2, 0x9) => {
                // I is set to the location of the hexadecimal sprite corresponding to the
                // value of Vx
                if x >= VREGS_SIZE {
                    return Err(Chip8Error::VregsOverflow);
                }

                let vx = self.vregs[x] as u16;
                // There are 16 hexadecimal sprites from 0 to F.
                if vx >= 16_u16 {
                    return Err(Chip8Error::UndefinedHexadecimal(vx as usize));
                }

                self.i = FONTS_OFFSET as u16 + FONTS_HEIGHT as u16 * vx;
            }
            // LD B, Vx
            (0xF, x, 0x3, 0x3) => {
                let vx = self.vregs[x];
                let idx = self.i as usize;
                self.mem[idx] = (vx / 100) % 10; // hundreds digit
                self.mem[idx + 1] = (vx / 10) % 10; // tens digit
                self.mem[idx + 2] = vx % 10; // ones digit
            }
            // LD [I], Vx
            (0xF, x, 0x5, 0x5) => {
                if x >= VREGS_SIZE {
                    return Err(Chip8Error::VregsOverflow);
                }

                for i in 0..=x {
                    self.mem[self.i as usize + i] = self.vregs[i];
                }
            }
            // LD Vx, [I]
            (0xF, _, 0x6, 0x5) => {
                // Set V0 to Vx from memory starting at location i
                // TODO: check the range of i ?

                for x in 0..16 {
                    self.vregs[x] = self.mem[self.i as usize + x];
                }
            }
            _ => return Err(Chip8Error::NotImplemented(opcode)),
        };

        Ok(())
    }

    pub fn step(&mut self) -> Result<(), Chip8Error> {
        self.emulate_insn()
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

    pub fn reset_keyboard(&mut self) {
        self.keyboard = [false; KEYBOARD_SIZE];
    }

    pub fn set_key(&mut self, key: usize, pressed: bool) {
        log::info!("set key called with {key} {pressed}");
        if key < KEYBOARD_SIZE {
            self.keyboard[key] = pressed;
            log::info!("{key} pressed");
        }
    }

    pub fn check_vregs(&self, src: usize, dst: &mut usize) -> Result<(), Chip8Error> {
        if src >= VREGS_SIZE {
            return Err(Chip8Error::VregsOverflow);
        }

        *dst = self.vregs[src] as usize;
        Ok(())
    }
}
