use minifb::{Window, WindowOptions};

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

pub struct Framebuffer {
    window: Window,
    width: usize,
    height: usize,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let window = Window::new(
            "Chip8 Emulation",
            width,
            height,
            WindowOptions {
                borderless: false,
                scale: minifb::Scale::X16,
                ..WindowOptions::default()
            },
        )
        .unwrap();

        Self {
            window,
            width,
            height,
        }
    }

    #[allow(clippy::identity_op)]
    pub fn draw(&mut self, buffer: &[u8]) {
        // TODO: convert buffer to buf
        // buffer is an Vec of <u8> of size 256 so each bit is a pixel
        // And each bit will be translated by a black or white pixel depending
        // of its value.
        assert_eq!(buffer.len() * 8, self.width * self.height);

        let mut buf: Vec<u32> = vec![0; self.width * self.height];

        for (i, byte) in buffer.iter().enumerate() {
            buf[i * 8 + 0] = white_or_black(*byte, 0x80);
            buf[i * 8 + 1] = white_or_black(*byte, 0x40);
            buf[i * 8 + 2] = white_or_black(*byte, 0x20);
            buf[i * 8 + 3] = white_or_black(*byte, 0x10);
            buf[i * 8 + 4] = white_or_black(*byte, 0x8);
            buf[i * 8 + 5] = white_or_black(*byte, 0x4);
            buf[i * 8 + 6] = white_or_black(*byte, 0x2);
            buf[i * 8 + 7] = white_or_black(*byte, 0x1);
        }

        self.window
            .update_with_buffer(&buf, self.width, self.height)
            .unwrap();
    }
}
