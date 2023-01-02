use super::{Input, Pixels};

pub trait Platform {
    /// signals the platform to poll events- window changes, inputs, etc.
    ///
    /// returns true if execution should continue, false if the game should exit.
    fn poll_events(&mut self) -> bool;

    /// writes the provided pixels to the screen.
    fn set_pixels(&mut self, pixels: &Pixels);

    /// returns the inputs being provided.
    fn input(&self) -> Input;

    /// returns how long the platform has been running for.
    fn runtime(&self) -> f32;
}
