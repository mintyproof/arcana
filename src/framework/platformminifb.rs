use super::{Input, Pixels, Platform};
use minifb::{Key, Window, WindowOptions};

#[allow(dead_code)]
pub struct PlatformMinifb {
    window: Window,
}

impl PlatformMinifb {
    pub fn new(window_width: usize, window_height: usize) -> Self {
        let window = Window::new("", window_width, window_height, WindowOptions::default())
            .unwrap_or_else(|e| {
                panic!("{}", e);
            });

        Self { window }
    }
}

impl Platform for PlatformMinifb {
    fn poll_events(&mut self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }

    fn set_pixels(&mut self, pixels: &Pixels) {
        self.window
            .update_with_buffer(&pixels.as_bytes(), pixels.width(), pixels.height())
            .unwrap();
    }

    fn input(&self) -> Input {
        Input {
            forward_move: 0.0,
            sideway_move: 0.0,
            mouse_x: 0,
            mouse_y: 0,
        }
    }

    fn runtime(&self) -> f32 {
        0.0
    }
}
