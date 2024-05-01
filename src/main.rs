use chip8_emulator::emulator::Chip8;
use std::env;
use std::process::exit;

use chip8_emulator::raylib_bindings::{
    begin_drawing, clear_background, close_window, color, draw_rectangle, end_drawing, init_window,
    is_key_pressed, keys, set_target_fps, window_should_close,
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

    while !window_should_close()
    // Detect window close button or ESC key
    {
        // Update
        // Check key pressed
        // Original layout
        //  1	2	3	C
        //  4	5	6	D
        //  7	8	9	E
        //  A	0	B	F
        chip.reset_keyboard();

        if is_key_pressed(keys::KEY_A) {
            chip.set_key(0, true)
        };
        if is_key_pressed(keys::KEY_Z) {
            chip.set_key(1, true)
        };
        if is_key_pressed(keys::KEY_E) {
            chip.set_key(2, true)
        };
        if is_key_pressed(keys::KEY_R) {
            chip.set_key(3, true)
        };
        if is_key_pressed(keys::KEY_T) {
            chip.set_key(4, true)
        };
        if is_key_pressed(keys::KEY_Q) {
            chip.set_key(5, true)
        };
        if is_key_pressed(keys::KEY_S) {
            chip.set_key(6, true)
        };
        if is_key_pressed(keys::KEY_D) {
            chip.set_key(7, true)
        };
        if is_key_pressed(keys::KEY_F) {
            chip.set_key(8, true)
        };
        if is_key_pressed(keys::KEY_G) {
            chip.set_key(9, true)
        };
        if is_key_pressed(keys::KEY_W) {
            chip.set_key(10, true)
        };
        if is_key_pressed(keys::KEY_X) {
            chip.set_key(11, true)
        };
        if is_key_pressed(keys::KEY_C) {
            chip.set_key(12, true)
        };
        if is_key_pressed(keys::KEY_V) {
            chip.set_key(13, true)
        };
        if is_key_pressed(keys::KEY_B) {
            chip.set_key(14, true)
        };
        if is_key_pressed(keys::KEY_N) {
            chip.set_key(15, true)
        };

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
