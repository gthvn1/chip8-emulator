use chip8_emulator::{chip8::Chip8, framebuffer::Framebuffer};
use std::{env, process::exit};

fn main() {
    // First argument is the name of the binary
    let a: Vec<String> = env::args().collect();

    if a.len() < 2 {
        println!("ERROR: You need to pass filename for the ROM");
        exit(1);
    }

    let filename = &a[1];
    println!("You pass {filename}");

    let chip = Chip8::new(filename);
    chip.dump_memory();

    let mut fb = Framebuffer::default();
    fb.draw();
}
