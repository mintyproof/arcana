use crate::pixels::Pixels;

/// represents the player's inputs into the game- e.g. movement and pressed actions like jump / interact.
pub struct Input {
    /// a value between -1.0 and 1.0 representing the player's forwards and backwards movement.
    ///
    /// * ` 1.0` : wants to move forwards at full speed  
    /// * ` 0.0` : does not want to move forwards nor backwards  
    /// * `-1.0` : wants to move backwards at full speed
    pub forward_move: f32,

    /// a value between -1.0 and 1.0 representing the player's left and right movement.
    ///
    /// * ` 1.0` : wants to move to the right at full speed  
    /// * ` 0.0` : does not want to move left nor right
    /// * `-1.0` : wants to move to the left at full speed6
    pub sideway_move: f32,
    
    pub mouse_x: u32,
    pub mouse_y: u32
}

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
