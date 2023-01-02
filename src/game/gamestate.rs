use crate::framework::{Input, Pixels};

pub enum GameStateUpdate {
    /// continue with no changes to game state.
    Continue,
    /// pushes a new game state onto the stack above the present state.
    Push(Box<dyn GameState>),
    /// pops the current game state off the stack.
    Pop,
    /// replaces the current game state entirely.
    Replace(Box<dyn GameState>),
    /// exits the game entirely.
    Quit,
}

pub trait GameState {
    fn on_update(&mut self, delta_time: f32, input: &Input) -> GameStateUpdate;
    fn on_draw(&mut self, delta_time: f32, pixels: &mut Pixels);
}
