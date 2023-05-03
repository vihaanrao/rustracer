use crate::vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub enum Texture {
    SolidColor(Vec3),
}

#[derive(Copy, Clone, Debug)]
pub struct Material {
    pub albedo: Texture,
}

impl Material {
    pub fn new(albedo: Texture) -> Self {
        Self { albedo }
    }

    pub fn color(self, _u: f64, _v: f64, _point: Vec3) -> Vec3 {
        match self.albedo {
            Texture::SolidColor(color) => color,
        }
    }
}