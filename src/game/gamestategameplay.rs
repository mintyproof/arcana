use super::{GameState, GameStateUpdate};
use crate::framework::Pixels;
use crate::world::{Tile, World};

pub struct GameStateGameplay {}

impl GameState for GameStateGameplay {
    fn on_update(&mut self, _delta_time: f32) -> GameStateUpdate {
        GameStateUpdate::Continue
    }

    fn on_draw(&mut self, delta_time: f32, pixels: &mut Pixels) {
        for y in 0..256 {
            for x in 0..256 {
                let c = (x ^ y) as u8;
                pixels.draw_pixel((x, y), (c, c, c));
            }
        }

        pixels.draw_text((8, 8), &format!("fps: {:.0}", (1.0 / delta_time).floor()))
    }
}
