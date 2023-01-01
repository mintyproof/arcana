pub const BYTES_PER_PIXEL: usize = 3;

#[derive(Clone)]
pub struct Pixels {
    buffer: Vec<u8>,
    width: usize,
    height: usize,
}

impl Pixels {
    pub fn new(width: usize, height: usize) -> Self {
        let mut buffer = Vec::new();
        buffer.resize(width * height * BYTES_PER_PIXEL, 0);

        Self {
            buffer,
            width,
            height,
        }
    }

    pub fn fill(&mut self, colour: (u8, u8, u8)) {
        for i in 0..(self.width * self.height) {
            self.buffer[i] = colour.0;
            self.buffer[i + 1] = colour.1;
            self.buffer[i + 2] = colour.2;
        }
    }

    pub fn draw_pixel(&mut self, position: (usize, usize), colour: (u8, u8, u8)) {
        let offset = self.offset_of(position.0, position.1);
        self.buffer[offset] = colour.0;
        self.buffer[offset + 1] = colour.1;
        self.buffer[offset + 2] = colour.2;
    }

    pub fn resize(&mut self, new_width: usize, new_height: usize) {
        self.buffer
            .resize(new_width * new_height * BYTES_PER_PIXEL, 0);
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
        self.width * BYTES_PER_PIXEL
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.buffer
    }

    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.buffer
    }

    pub fn offset_of(&self, x: usize, y: usize) -> usize {
        y * self.pitch() + x * BYTES_PER_PIXEL
    }
}
