use minifb::{Key, Window, WindowOptions};

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn white_or_black(v: u8, mask: u8) -> u32 {
    let white: u32 = from_u8_rgb(0xFF, 0xFF, 0xFF);
    let black: u32 = from_u8_rgb(0, 0, 0);

    if v & mask == 0 {
        black
    } else {
        white
    }
}
const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Framebuffer {
    buffer: Vec<u32>,
    window: Window,
}

impl Framebuffer {
    pub fn new() -> Self {
        let buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

        let mut window = Window::new(
            "Chip8 Emulation",
            WIDTH,
            HEIGHT,
            WindowOptions {
                borderless: false,
                scale: minifb::Scale::X16,
                ..WindowOptions::default()
            },
        )
        .unwrap();

        // Limit to 60 fps
        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        Self { buffer, window }
    }

    #[allow(clippy::identity_op)]
    pub fn set_sprite_line_at(&mut self, at: usize, line: &u8) {
        // allow writing at + 0 for readability...
        self.buffer[at + 0] = white_or_black(*line, 0x80);
        self.buffer[at + 1] = white_or_black(*line, 0x40);
        self.buffer[at + 2] = white_or_black(*line, 0x20);
        self.buffer[at + 3] = white_or_black(*line, 0x10);
        self.buffer[at + 4] = white_or_black(*line, 0x08);
        self.buffer[at + 5] = white_or_black(*line, 0x04);
        self.buffer[at + 6] = white_or_black(*line, 0x02);
        self.buffer[at + 7] = white_or_black(*line, 0x01);
    }

    pub fn draw(&mut self) {
        // SPACE INVADER SPRITE
        let space_invader: Vec<u8> = vec![0xBA, 0x7C, 0xD6, 0xFE, 0x54, 0xAA];

        for (i, v) in space_invader.iter().enumerate() {
            self.set_sprite_line_at(i * WIDTH, v);
        }

        // 0
        let zero: Vec<u8> = vec![0xF0, 0x90, 0x90, 0x90, 0xF0];
        let un: Vec<u8> = vec![0x20, 0x60, 0x20, 0x20, 0x70];
        let deux: Vec<u8> = vec![0xF0, 0x10, 0xF0, 0x80, 0xF0];

        for (i, v) in zero.iter().enumerate() {
            self.set_sprite_line_at(i * WIDTH + 0x8, v);
        }

        for (i, v) in un.iter().enumerate() {
            self.set_sprite_line_at(i * WIDTH + 0x10, v);
        }

        for (i, v) in deux.iter().enumerate() {
            self.set_sprite_line_at(i * WIDTH + 0x18, v);
        }

        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            //for i in self.buffer.iter_mut() {
            //    *i = if *i == 0 {
            //        0xFF
            //    } else {
            //        0x00
            //    }
            //}

            self.window
                .update_with_buffer(&self.buffer, WIDTH, HEIGHT)
                .unwrap();
        }
    }
}

impl Default for Framebuffer {
    fn default() -> Self {
        Self::new()
    }
}
