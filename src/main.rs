pub mod game;
pub mod pixels;
pub mod platform;
pub mod platformsdl2;

use crate::game::{Game, GameStateInit};

fn main() {
    let mut game = Game::new();
    game.push_state(Box::new(GameStateInit {}));
    game.run().unwrap();
}
