use super::{GameState, GameStateGameplay, GameStateUpdate};
use crate::framework::{Input, Pixels};

pub struct GameStateInit {}

impl GameState for GameStateInit {
    fn on_update(&mut self, _delta_time: f32, _input: &Input) -> GameStateUpdate {
        GameStateUpdate::Replace(Box::new(GameStateGameplay::new()))
    }

    fn on_draw(&mut self, _: f32, pixels: &mut Pixels) {
        pixels.fill((0, 0, 0));
    }
}
