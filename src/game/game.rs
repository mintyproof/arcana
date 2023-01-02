use super::{GameState, GameStateUpdate};
use crate::framework::*;

pub struct Game {
    platform: Box<dyn Platform>,
    states: Vec<Box<dyn GameState>>,
    pixels: Pixels,
}

impl Game {
    /// creates a new game.
    pub fn new() -> Self {
        Self {
            platform: Box::new(PlatformSDL2::new(768, 768)),
            states: Vec::new(),
            pixels: Pixels::new(256, 256),
        }
    }

    /// pushes a new state onto the game state stack.
    pub fn push_state(&mut self, state: Box<dyn GameState>) {
        self.states.push(state);
    }

    /// pops the top state off of the stack.
    pub fn pop_state(&mut self) {
        self.states.pop();
    }

    /// begins the main game loop, which runs until the game is exited manually or an error is encountered.
    pub fn run(&mut self) -> Result<(), String> {
        let mut previous_runtime = self.platform.runtime();
        'game_loop: loop {
            if self.platform.poll_events() == false {
                break 'game_loop;
            }

            let current_runtime = self.platform.runtime();
            let delta_time = current_runtime - previous_runtime;
            previous_runtime = current_runtime;

            let state_result = if let Some(state) = self.states.last_mut() {
                let state_result = state.on_update(delta_time, &self.platform.input());
                state.on_draw(delta_time, &mut self.pixels);

                state_result
            } else {
                break 'game_loop; // no states left on the stack, so end the game!
            };

            match state_result {
                GameStateUpdate::Continue => (),
                GameStateUpdate::Push(result_state) => {
                    self.push_state(result_state);
                }
                GameStateUpdate::Pop => {
                    self.pop_state();
                }
                GameStateUpdate::Replace(result_state) => {
                    self.pop_state();
                    self.push_state(result_state);
                }
                GameStateUpdate::Quit => break 'game_loop,
            }

            self.platform.set_pixels(&self.pixels);
        }

        Ok(())
    }
}
