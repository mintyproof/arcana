use crate::{
    pixels::Pixels,
    platform::{Platform, Input},
};
use sdl2::{
    event::Event,
    keyboard::Keycode,
    render::{Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
};

#[allow(dead_code)]
struct SdlContext {
    sdl: sdl2::Sdl,
    video: sdl2::VideoSubsystem,
    timer: sdl2::TimerSubsystem,
    event_pump: sdl2::EventPump,
}

impl SdlContext {
    pub fn new() -> Self {
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();
        let timer = sdl.timer().unwrap();
        let event_pump = sdl.event_pump().unwrap();

        Self {
            sdl,
            video,
            timer,
            event_pump,
        }
    }
}

#[allow(dead_code)]
pub struct PlatformSDL2 {
    sdl: SdlContext,
    canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
    display_texture: Texture,
    performance_counter_at_start: u64
}

impl PlatformSDL2 {
    pub fn new(window_width: u32, window_height: u32) -> Self {
        let sdl = SdlContext::new();
        let window = sdl
            .video
            .window("", window_width, window_height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        let canvas = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();
        let texture_creator = canvas.texture_creator();
        let display_texture = texture_creator
            .create_texture_streaming(sdl2::pixels::PixelFormatEnum::RGB24, 1, 1)
            .unwrap();
        
        let performance_counter_at_start = sdl.timer.performance_counter();

        Self {
            sdl,
            canvas,
            texture_creator,
            display_texture,
            performance_counter_at_start
        }
    }

    fn create_or_refresh_texture(&mut self, width: usize, height: usize) {
        self.display_texture = self.texture_creator
            .create_texture_streaming(sdl2::pixels::PixelFormatEnum::RGB24, width as u32, height as u32)
            .unwrap();
    }
}

impl Platform for PlatformSDL2 {
    fn poll_events(&mut self) -> bool {
        for event in self.sdl.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return false,
                _ => {}
            }
        }

        true
    }

    fn set_pixels(&mut self, pixels: &Pixels) {
        let query = self.display_texture.query();
        if pixels.width() != query.width as usize || pixels.height() != query.height as usize {
            self.create_or_refresh_texture(pixels.width(), pixels.height());
        }

        self.display_texture
            .with_lock(None, |buffer: &mut [u8], _: usize| {
                buffer.copy_from_slice(&pixels.as_bytes());
            })
            .unwrap();
        self.canvas.copy(&self.display_texture, None, None).unwrap();
        self.canvas.present();
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
        let performance_frequency = self.sdl.timer.performance_frequency();
        let performance_counter_now = self.sdl.timer.performance_counter();
        (performance_counter_now - self.performance_counter_at_start) as f32 / performance_frequency as f32
    }
}
