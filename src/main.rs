use chip8_emulator::chip8::Chip8;
use std::env;
use std::process::exit;

use chip8_emulator::raylib_bindings::{
    begin_drawing, clear_background, close_window, color, end_drawing, init_window, set_target_fps,
    window_should_close,
};

const RESOLUTION: (i32, i32) = (64, 32);

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

    let pixel_width = 10;
    let pixel_height = 10;

    let screen_width = RESOLUTION.0 * pixel_width;
    let screen_height = RESOLUTION.1 * pixel_height;

    let mut chip = Chip8::default();
    chip.load(filename).unwrap();

    init_window(screen_width, screen_height, "Chip8 emulator".to_string());

    set_target_fps(60);

    while !window_should_close()
    // Detect window close button or ESC key
    {
        // Update
        if let Err(e) = chip.step() {
            log::error!("{e}");
            break;
        }

        begin_drawing();
        clear_background(color::RAYWHITE);

        // TODO: display the framebuffer
        let _ = chip.get_copy_of_framebuffer();

        end_drawing();
    }

    // De-Initialization
    close_window(); // Close window and OpenGL context
}
