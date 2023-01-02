pub mod framework;
pub mod game;
pub mod math;
pub mod world;

use crate::game::{Game, GameStateInit};

fn main() {
    let mut game = Game::new();
    game.push_state(Box::new(GameStateInit {}));
    game.run().unwrap();
}
