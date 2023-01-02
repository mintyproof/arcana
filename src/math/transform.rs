use glam::{Mat4, Vec3};

pub struct Transform {
    position: Vec3,
    rotation: Vec3,
    scale: Vec3,
    right: Vec3,
    up: Vec3,
    forward: Vec3,
    model: Mat4,
    view: Mat4
}

impl Transform {
    pub fn new(position: Vec3, rotation: Vec3, scale: Vec3) -> Self {
        let mut transform = Self {
            position,
            rotation,
            scale,
            // following values are placeholders; properly set by update()
            right: Vec3::ZERO,
            up: Vec3::ZERO,
            forward: Vec3::ZERO,
            model: Mat4::IDENTITY,
            view: Mat4::IDENTITY
        };

        transform.update();
        
        transform
    }

    pub fn identity() -> Self {
        Transform::new(Vec3::ZERO, Vec3::ZERO, Vec3::ONE)
    }

    pub fn with_position(position: Vec3) -> Self {
        Transform::new(position, Vec3::ZERO, Vec3::ONE)
    }

    pub fn with_rotation(rotation: Vec3) -> Self {
        Transform::new(Vec3::ZERO, rotation, Vec3::ONE)
    }

    fn update(&mut self) {
        // convert rotations to be in the range of -180.0 to 180.0!

        if self.rotation.x.abs() > 180.0 {
            self.rotation.x -= 360.0 * ((self.rotation.x + 180.0).floor() / 360.0) * self.rotation.x.signum();
        }

        if self.rotation.y.abs() > 180.0 {
            self.rotation.y -= 360.0 * ((self.rotation.y + 180.0).floor() / 360.0) * self.rotation.y.signum();
        }
        
        if self.rotation.x.abs() > 180.0 {
            self.rotation.z -= 360.0 * ((self.rotation.z + 180.0).floor() / 360.0) * self.rotation.z.signum();
        }

        // calculate forward, right, and up vectors!

        let rx_rads = self.rotation.x.to_radians();
        let ry_rads = self.rotation.y.to_radians();
        let rz_rads = self.rotation.z.to_radians();

        self.forward.x =  rx_rads.cos() * ry_rads.sin();
        self.forward.y = -rx_rads.sin();
        self.forward.z =  rx_rads.cos() * ry_rads.cos();

        self.right.x =  rz_rads.cos() * -ry_rads.cos();
        self.right.y = -rz_rads.sin();
        self.right.z =  rz_rads.cos() * ry_rads.sin();

        self.up = self.right.cross(self.forward);

        // calculate view and model matrices!

        self.model = Mat4::IDENTITY; // TODO
        self.view = Mat4::look_at_lh(self.position, self.position + self.forward, self.up);
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

    pub fn left(&self) -> Vec3 {
        -self.right
    }

    pub fn right(&self) -> Vec3 {
        self.right
    }

    pub fn down(&self) -> Vec3 {
        -self.up
    }

    pub fn up(&self) -> Vec3 {
        self.up
    }

    pub fn backward(&self) -> Vec3 {
        -self.forward
    }

    pub fn forward(&self) -> Vec3 {
        self.forward
    }

    pub fn model(&self) -> Mat4 {
        self.model
    }

    pub fn view(&self) -> Mat4 {
        self.view
    }
}