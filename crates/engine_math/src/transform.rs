use glam::{Mat4, Quat, Vec3};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Transform {
    pub const IDENTITY: Self = Self {
        translation: Vec3::ZERO,
        rotation: Quat::IDENTITY,
        scale: Vec3::ONE,
    };

    pub fn identity() -> Self {
        Self::IDENTITY
    }

    pub fn from_xy(x: f32, y: f32) -> Self {
        Self {
            translation: Vec3::new(x, y, 0.0),
            ..Self::IDENTITY
        }
    }

    pub fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Self {
            translation: Vec3::new(x, y, z),
            ..Self::IDENTITY
        }
    }

    pub fn with_translation(mut self, translation: Vec3) -> Self {
        self.translation = translation;
        self
    }

    pub fn with_scale(mut self, scale: Vec3) -> Self {
        self.scale = scale;
        self
    }

    pub fn with_scale_xy(mut self, x: f32, y: f32) -> Self {
        self.scale = Vec3::new(x, y, 1.0);
        self
    }

    pub fn with_rotation_z(mut self, radians: f32) -> Self {
        self.rotation = Quat::from_rotation_z(radians);
        self
    }

    pub fn matrix(self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation)
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::IDENTITY
    }
}