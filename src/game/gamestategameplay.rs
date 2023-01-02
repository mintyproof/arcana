use super::{GameState, GameStateUpdate};
use crate::framework::{Input, Pixels};
use crate::world::{Tile, World};
use glam::{Vec2, Vec3, Vec4, Mat4};

pub struct GameStateGameplay {
    world: World
}

impl GameStateGameplay {
    pub fn new() -> Self {
        let mut world = World::new();
        world.set(0, 0, 0, 1);
        world.set(0, 3, 0, 1);
        world.set(3, 0, 0, 1);
        world.set(3, 3, 0, 1);
        world.set(0, 0, 3, 1);
        world.set(0, 3, 3, 1);
        world.set(3, 0, 3, 1);
        world.set(3, 3, 3, 1);

        Self { world }
    }
}

impl GameState for GameStateGameplay {
    fn on_update(&mut self, _delta_time: f32, _input: &Input) -> GameStateUpdate {
        GameStateUpdate::Continue
    }

    fn on_draw(&mut self, delta_time: f32, pixels: &mut Pixels) {
        let pos = Vec3::new(2.0, 2.0, -3.0);
        let view = Mat4::look_at_lh(pos, Vec3::new(2.0, 2.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
        let proj = Mat4::perspective_lh(90.0_f32.to_radians(), 1.0, 0.001, 100.0);

        for y in 0..pixels.width() {
            for x in 0..pixels.height() {
                let screen_coord = Vec2::new(
                    2.0 * x as f32 / pixels.width() as f32 - 1.0,
                    2.0 * y as f32 / pixels.height() as f32 - 1.0
                );
                let dir = {
                    let clip_space = Vec4::new(screen_coord.x, screen_coord.y, 1.0, 1.0);
                    let direction = (proj * view).inverse() * clip_space;
                    Vec3::new(direction.x, direction.y, direction.z).normalize()
                };

                let result = self.world.raycast(pos, dir, 16);

                if result != 0 {
                    pixels.draw_pixel((x, y), (255, 255, 255));
                }
            }
        }

        pixels.draw_text((8, 8), &format!("fps: {:.0}", (1.0 / delta_time).floor()))
    }
}
