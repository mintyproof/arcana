use crate::pixels::Pixels;
use crate::platform::Platform;
use crate::platformsdl2::PlatformSDL2;

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
    Quit
}

pub trait GameState {
    fn on_update(&mut self, delta_time: f32) -> GameStateUpdate;
    fn on_draw(&mut self, delta_time: f32, pixels: &mut Pixels);
}

pub struct GameStateInit {}

impl GameState for GameStateInit {
    fn on_update(&mut self, _delta_time: f32) -> GameStateUpdate {
        GameStateUpdate::Replace(Box::new(GameStateGameplay {}))
    }

    fn on_draw(&mut self, _: f32, pixels: &mut Pixels) {
        pixels.fill((0, 0, 0));
    }
}

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

        pixels.draw_text((16, 16), &format!("fps: {:.0}", (1.0 / delta_time).floor()))
    }
}

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
                let state_result = state.on_update(delta_time);
                state.on_draw(delta_time, &mut self.pixels);

                state_result
            } else {
                break 'game_loop; // no states left on the stack, so end the game!
            };
            
            match state_result {
                GameStateUpdate::Continue => (),
                GameStateUpdate::Push(result_state) => {
                    self.push_state(result_state);
                },
                GameStateUpdate::Pop => {
                    self.pop_state();
                },
                GameStateUpdate::Replace(result_state) => {
                    self.pop_state();
                    self.push_state(result_state);
                },
                GameStateUpdate::Quit => break 'game_loop
            }

            self.platform.set_pixels(&self.pixels);
        }

        Ok(())
    }
}
