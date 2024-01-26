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

    let mut fb = Framebuffer::new(64, 32);
    let mut chip = Chip8::new(filename);

    // Should start with a blank screen
    fb.draw(chip.framebuffer());
    std::thread::sleep(std::time::Duration::from_secs_f32(2.0));

    // First instruction of IBM logo is clean screen so it should become
    // black after 2 seconds...
    chip.emulate_one_insn();
    fb.draw(chip.framebuffer());

    chip.dump_memory();

    // Sleep before closing window
    // TODO: find a better way :)
    std::thread::sleep(std::time::Duration::from_secs_f32(2.0));
}
