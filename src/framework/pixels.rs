fn pack_rgba(r: u8, g: u8, b: u8, a: u8) -> u32 {
    return (a as u32) << 24 | (r as u32) << 16 | (g as u32) << 8 | b as u32;
}

#[derive(Clone)]
pub struct Pixels {
    buffer: Vec<u32>,
    width: usize,
    height: usize,
}

impl Pixels {
    pub fn new(width: usize, height: usize) -> Self {
        let mut buffer = Vec::new();
        buffer.resize(width * height, 0);

        Self {
            buffer,
            width,
            height,
        }
    }

    pub fn fill(&mut self, colour: (u8, u8, u8)) {
        self.buffer
            .fill(pack_rgba(colour.0, colour.1, colour.2, 255));
    }

    pub fn draw_pixel(&mut self, position: (usize, usize), colour: (u8, u8, u8)) {
        let offset = self.offset_of(position.0, position.1);
        self.buffer[offset] = pack_rgba(colour.0, colour.1, colour.2, 255);
    }

    pub fn draw_text(&mut self, position: (usize, usize), text: &str) {
        let mut position = position;
        for c in text.chars() {
            let font_char = font8x8::BASIC_UNICODE[c as usize];

            for y in font_char.byte_array() {
                for bit in 0..8 {
                    match y & 1 << bit {
                        0 => self.draw_pixel((position.0 + bit, position.1), (0, 0, 0)),
                        _ => self.draw_pixel((position.0 + bit, position.1), (255, 255, 255)),
                    }
                }
                position.1 += 1;
            }

            position.0 += 8;
            position.1 -= 8; // reset y position for next character!
        }
    }

    pub fn resize(&mut self, new_width: usize, new_height: usize) {
        self.buffer.resize(new_width * new_height, 0);
        self.buffer.fill(0);

        self.width = new_width;
        self.height = new_height;
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn pitch(&self) -> usize {
        self.width
    }

    pub fn as_bytes(&self) -> &[u32] {
        &self.buffer
    }

    pub fn as_bytes_mut(&mut self) -> &mut [u32] {
        &mut self.buffer
    }

    pub fn offset_of(&self, x: usize, y: usize) -> usize {
        y * self.pitch() + x
    }
}
