use glam::{IVec3, Vec3};

pub type Tile = u8;

const SIZE_X: usize = 16;
const SIZE_Y: usize = 16;
const SIZE_Z: usize = 16;

pub struct RaycastResult {
    pub tile: Tile,
    pub normal: Vec3
}

pub struct World {
    size: (usize, usize, usize),
    tiles: Vec<Tile>,
}

impl World {
    pub fn new() -> Self {
        let size = (SIZE_X, SIZE_Y, SIZE_Z);
        let mut tiles = Vec::new();
        tiles.resize(SIZE_X * SIZE_Y * SIZE_Z, 0);

        Self { size, tiles }
    }

    pub fn raycast(&self, pos: Vec3, dir: Vec3, distance: usize) -> Option<RaycastResult> {
        let mut t_pos = IVec3::new(
            pos.x.floor() as i32,
            pos.y.floor() as i32,
            pos.z.floor() as i32,
        );

        let t_step = IVec3::new(
            if dir.x < 0.0 { -1 } else { 1 },
            if dir.y < 0.0 { -1 } else { 1 },
            if dir.z < 0.0 { -1 } else { 1 },
        );

        let t_delta = (1.0 / dir).abs();

        let mut t_max = Vec3::ZERO;
        if t_step.x < 0 {
            t_max.x = (pos.x - t_pos.x as f32) * t_delta.x;
        } else {
            t_max.x = (t_pos.x as f32 + 1.0 - pos.x) * t_delta.x;
        }
        if t_step.y < 0 {
            t_max.y = (pos.y - t_pos.y as f32) * t_delta.y;
        } else {
            t_max.y = (t_pos.y as f32 + 1.0 - pos.y) * t_delta.y;
        }
        if t_step.z < 0 {
            t_max.z = (pos.z - t_pos.z as f32) * t_delta.z;
        } else {
            t_max.z = (t_pos.z as f32 + 1.0 - pos.z) * t_delta.z;
        }

        let mut steps_taken = 0;
        loop {
            steps_taken += 1;

            if t_max.x < t_max.y || t_max.z < t_max.y {
                if t_max.x < t_max.z {
                    t_max.x += t_delta.x;
                    t_pos.x += t_step.x;
                } else {
                    t_max.z += t_delta.z;
                    t_pos.z += t_step.z;
                }
            } else {
                t_max.y += t_delta.y;
                t_pos.y += t_step.y;
            }

            if let Some(tile) = self.get(t_pos.x as usize, t_pos.y as usize, t_pos.z as usize) {
                if tile != 0 {
                    return Some(RaycastResult {
                        tile,
                        normal: Vec3::new(1.0, 1.0, 1.0)
                    });
                }
            }

            if steps_taken > distance {
                break;
            }
        }

        None
    }

    pub fn in_bounds(&self, x: usize, y: usize, z: usize) -> bool {
        /* x >= 0 && y >= 0 && z >= 0 && */
        x < self.size.0 && y < self.size.1 && z < self.size.2
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, tile: Tile) {
        let index = self.offset_of(x, y, z);
        self.tiles[index] = tile;
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> Option<Tile> {
        if !self.in_bounds(x, y, z) {
            return None;
        }

        let index = self.offset_of(x, y, z);
        Some(self.tiles[index])
    }

    pub fn size_x(&self) -> usize {
        self.size.0
    }

    pub fn size_y(&self) -> usize {
        self.size.1
    }

    pub fn size_z(&self) -> usize {
        self.size.2
    }

    fn offset_of(&self, x: usize, y: usize, z: usize) -> usize {
        (self.size.0 * self.size.1 * z) + (self.size.0 * y) + x
    }
}
