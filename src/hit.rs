use crate::vec3::Vec3;
use crate::material::Material;
pub struct Hit {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

// #[derive(Copy, Clone, Debug)]
// pub struct Material {
//     pub albedo: Vec3,
// }

// impl Material {
//     pub fn color(&self, u: f64, v: f64, point: Vec3) -> Vec3 {
//         self.albedo
//     }
// }

//make pub fn new
impl Hit {
    pub fn new(t: f64, point: Vec3, normal: Vec3, material: Material) -> Self {
        Self {
            t,
            point,
            normal,
            material,
        }
    }
}


