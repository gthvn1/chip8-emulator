use chip8_emulator::chip8::Chip8;
use log;
use std::env;
use std::process::exit;

fn main() {
    env_logger::init();

    // First argument is the name of the binary
    let a: Vec<String> = env::args().collect();

    if a.len() < 2 {
        log::error!("You need to pass filename for the ROM");
        exit(1);
    }

    let filename = &a[1];
    log::info!("Emulating {filename}");

    let mut chip = Chip8::default();
    chip.load(filename).unwrap();
    //chip.dump_memory();
    chip.run();
}
