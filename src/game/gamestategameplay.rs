use super::{GameState, GameStateUpdate};
use crate::framework::{Input, Pixels};
use crate::math::Transform;
use crate::world::World;
use glam::{Mat4, Vec2, Vec3, Vec4};

pub struct GameStateGameplay {
    world: World,
    camera: Transform,
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

        let camera = Transform::new(
            Vec3::new(2.0, 2.0, -3.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::ONE,
        );

        Self { world, camera }
    }

    pub fn render_world(&self, pixels: &mut Pixels) {
        let view = self.camera.view();
        let aspect = pixels.width() as f32 / pixels.height() as f32;
        let proj = Mat4::perspective_infinite_lh(90.0_f32.to_radians(), aspect, 0.001);

        for y in 0..pixels.width() {
            for x in 0..pixels.height() {
                let screen_coord = Vec2::new(
                    2.0 * x as f32 / pixels.width() as f32 - 1.0,
                    2.0 * y as f32 / pixels.height() as f32 - 1.0,
                );
                let dir = {
                    let clip_space = Vec4::new(screen_coord.x, screen_coord.y, 1.0, 1.0);
                    let direction = (proj * view).inverse() * clip_space;
                    Vec3::new(direction.x, direction.y, direction.z).normalize()
                };

                if let Some(result) = self.world.raycast(self.camera.position(), dir, 16) {
                    pixels.draw_pixel(
                        (x, y),
                        (
                            (result.normal.x * 255.0).floor() as u8,
                            (result.normal.y * 255.0).floor() as u8,
                            (result.normal.z * 255.0).floor() as u8,
                        ),
                    );
                }
            }
        }
    }

    pub fn render_debug_ui(&self, delta_time: f32, pixels: &mut Pixels) {
        pixels.draw_text((8, 8), &format!("fps: {:.0}", (1.0 / delta_time).floor()));
        pixels.draw_text(
            (8, 16),
            &format!(
                "pos: {:.2} {:.2} {:.2}",
                self.camera.position().x,
                self.camera.position().y,
                self.camera.position().z
            ),
        );
        pixels.draw_text(
            (8, 24),
            &format!(
                "dir: {:.2} {:.2} {:.2}",
                self.camera.rotation().x,
                self.camera.rotation().y,
                self.camera.rotation().z
            ),
        );
    }
}

impl GameState for GameStateGameplay {
    fn on_update(&mut self, delta_time: f32, input: &Input) -> GameStateUpdate {
        self.camera.set_position(
            self.camera.position() + self.camera.forward() * delta_time * input.forward_move,
        );
        GameStateUpdate::Continue
    }

    fn on_draw(&mut self, delta_time: f32, pixels: &mut Pixels) {
        self.render_world(pixels);
        self.render_debug_ui(delta_time, pixels);
    }
}
