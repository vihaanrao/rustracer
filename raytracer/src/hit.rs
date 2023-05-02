use crate::vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Hit {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
}

impl Hit {
    pub fn new(t: f64, point: Vec3, normal: Vec3) -> Self {
        Self { t, point, normal }
    }
}
