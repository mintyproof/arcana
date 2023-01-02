use glam::{Mat4, Vec3};

pub struct Transform {
    position: Vec3,
    rotation: Vec3,
    scale: Vec3
}

impl Transform {
    pub fn new(position: Vec3, rotation: Vec3, scale: Vec3) -> Self {
        Self {
            position,
            rotation,
            scale
        }
    }

    pub fn identity() -> Self {
        Transform::new(Vec3::ZERO, Vec3::ZERO, Vec3::ONE)
    }

    pub fn position(&self) -> Vec3 {
        self.position
    }

    pub fn rotation(&self) -> Vec3 {
        self.rotation
    }

    pub fn scale(&self) -> Vec3 {
        self.scale
    }
}