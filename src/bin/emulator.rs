use chip8_emulator::chip8::Chip8;
use std::env;
use std::process::exit;

use chip8_emulator::raylib_bindings::{
    begin_drawing, clear_background, close_window, color, draw_text, end_drawing, init_window,
    set_target_fps, window_should_close,
};

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
    //chip.run();

    // JUST FOR TESTING THAT RAYLIB is working
    init_window(200, 200, "Chip8 emulator".to_string());

    set_target_fps(60); // Set our game to run at 60 frames-per-second

    // Main game loop
    while !window_should_close()
    // Detect window close button or ESC key
    {
        // Update
        // TODO: Update your variables here
        // Draw
        begin_drawing();

        clear_background(color::RAYWHITE);

        draw_text(
            "Congrats! You created your first window!".to_string(),
            190,
            200,
            20,
            color::LIGHTGRAY,
        );

        end_drawing();
    }

    // De-Initialization
    close_window(); // Close window and OpenGL context
                    //
                    // First argument is the name of the binary
                    //let a: Vec<String> = env::args().collect();
}
