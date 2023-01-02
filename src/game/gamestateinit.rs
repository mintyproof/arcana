use super::{GameState, GameStateGameplay, GameStateUpdate};
use crate::framework::Pixels;

pub struct GameStateInit {}

impl GameState for GameStateInit {
    fn on_update(&mut self, _delta_time: f32) -> GameStateUpdate {
        GameStateUpdate::Replace(Box::new(GameStateGameplay::new()))
    }

    fn on_draw(&mut self, _: f32, pixels: &mut Pixels) {
        pixels.fill((0, 0, 0));
    }
}
