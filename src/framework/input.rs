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
    pub mouse_y: u32,
}
