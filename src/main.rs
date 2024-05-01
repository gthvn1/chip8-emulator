use chip8_emulator::emulator::Chip8;
use std::env;
use std::process::exit;

use chip8_emulator::raylib_bindings::{
    begin_drawing, clear_background, close_window, color, draw_rectangle, end_drawing, init_window,
    is_key_pressed, is_key_released, keys, set_target_fps, window_should_close,
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

    let pixel_width = 20_i32;
    let pixel_height = 20_i32;

    // Use a window of 1280 x 640
    let screen_width: i32 = RESOLUTION.0 * pixel_width;
    let screen_height: i32 = RESOLUTION.1 * pixel_height;

    let mut chip = Chip8::default();
    chip.load(filename).unwrap();

    init_window(screen_width, screen_height, "Chip8 emulator".to_string());

    set_target_fps(360);

    // Check key pressed
    // Original layout
    //  1	2	3	C
    //  4	5	6	D
    //  7	8	9	E
    //  A	0	B	F
    let keymap = [
        keys::KEY_A,
        keys::KEY_Z,
        keys::KEY_E,
        keys::KEY_R,
        keys::KEY_T,
        keys::KEY_Q,
        keys::KEY_S,
        keys::KEY_D,
        keys::KEY_F,
        keys::KEY_G,
        keys::KEY_H,
        keys::KEY_U,
        keys::KEY_J,
        keys::KEY_I,
        keys::KEY_K,
        keys::KEY_O,
    ];

    while !window_should_close()
    // Detect window close button or ESC key
    {
        // Update keyboard state
        for (i, k) in keymap.iter().enumerate() {
            if is_key_pressed(*k) {
                chip.set_key(i, true)
            }
            if is_key_released(*k) {
                chip.set_key(i, false)
            }
        }

        // Step to next instruction
        // NOTE: Delay and Sound timer are updated by step()
        if let Err(e) = chip.step() {
            log::error!("{e}");
            break;
        }

        begin_drawing();
        clear_background(color::BLACK);

        let fb = chip.get_framebuffer();

        for (i, byte) in fb.iter().enumerate() {
            let pw = pixel_width;
            let ph = pixel_height;

            let x: i32 = ((i as i32 * 8) % RESOLUTION.0) * pw;
            let y: i32 = (i as i32 / 8) * ph;

            // We draw a 20x20 rectangle for each bit set to 1
            if byte & 0x80 == 0x80 {
                draw_rectangle(x, y, pw, ph, color::GREEN);
            }
            if byte & 0x40 == 0x40 {
                draw_rectangle(x + pw, y, pw, ph, color::GREEN);
            }
            if byte & 0x20 == 0x20 {
                draw_rectangle(x + 2 * pw, y, pw, ph, color::GREEN);
            }
            if byte & 0x10 == 0x10 {
                draw_rectangle(x + 3 * pw, y, pw, ph, color::GREEN);
            }
            if byte & 0x8 == 0x8 {
                draw_rectangle(x + 4 * pw, y, pw, ph, color::GREEN);
            }
            if byte & 0x4 == 0x4 {
                draw_rectangle(x + 5 * pw, y, pw, ph, color::GREEN);
            }
            if byte & 0x2 == 0x2 {
                draw_rectangle(x + 6 * pw, y, pw, ph, color::GREEN);
            }
            if byte & 0x1 == 0x1 {
                draw_rectangle(x + 7 * pw, y, pw, ph, color::GREEN);
            }
        }

        end_drawing();
    }

    // De-Initialization
    close_window(); // Close window and OpenGL context
}
