use crate::{vec3::Vec3, random_in_unit_sphere, ray::Ray, hit::Hit};

#[derive(Copy, Clone, Debug)]
pub enum Texture {
    SolidColor(Vec3),
    Metal(Vec3, f64),
    Dielectric(f64),
    Glass(f64, f64), // Add Glass with an index of refraction and reflection coefficient
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
            Texture::Metal(color, _) => color,
            Texture::Dielectric(_) => Vec3::new(1.0, 1.0, 1.0),
            Texture::Glass(_, _) => Vec3::new(1.0, 1.0, 1.0), // Default color for glass material
        }
    }

    pub fn scatter(&self, ray_in: Ray, hit: &Hit) -> Option<(Vec3, Ray)> {
        match self.albedo {
                        
            Texture::SolidColor(_) => {
                let target = hit.point + hit.normal + random_in_unit_sphere();
                Some((self.color(0.0, 0.0, hit.point), Ray::new(hit.point, target - hit.point)))
            }
            Texture::Metal(color, fuzz) => {
                let reflected = ray_in.direction.unit_vector().reflect(&hit.normal);
                let scattered = Ray::new(hit.point, reflected + fuzz * random_in_unit_sphere());
                if scattered.direction.dot(hit.normal) > 0.0 {
                    Some((color, scattered))
                } else {
                    None
                }
            }
            Texture::Dielectric(ref_idx) => {
                let attenuation = Vec3::new(1.0, 1.0, 1.0);
                let front_face = ray_in.direction.dot(hit.normal) < 0.0; // Determine if it's a front face hit
                let etai_over_etat = if front_face { 1.0 / ref_idx } else { ref_idx };
    
                let unit_direction = ray_in.direction.unit_vector();
                let cos_theta = (-unit_direction).dot(hit.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
    
                let cannot_refract = etai_over_etat * sin_theta > 1.0;
                let direction = if cannot_refract || Self::schlick(cos_theta, etai_over_etat) > rand::random() {
                    unit_direction.reflect(&hit.normal)
                } else {
                    Self::refract(unit_direction, hit.normal, etai_over_etat)
                };
    
                Some((attenuation, Ray::new(hit.point, direction)))
            }
            Texture::Glass(ref_idx, reflection_coeff) => {
                let front_face = ray_in.direction.dot(hit.normal) < 0.0;
                let etai_over_etat = if front_face { 1.0 / ref_idx } else { ref_idx };

                let unit_direction = ray_in.direction.unit_vector();
                let cos_theta = (-unit_direction).dot(hit.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let cannot_refract = etai_over_etat * sin_theta > 1.0;
                let direction = if cannot_refract || Self::schlick(cos_theta, etai_over_etat) > rand::random() {
                    unit_direction.reflect(&hit.normal)
                } else {
                    Self::refract(unit_direction, hit.normal, etai_over_etat)
                };

                let scattered = Ray::new(hit.point, direction);
                let attenuation = Vec3::new(1.0, 1.0, 1.0) * reflection_coeff;
                Some((attenuation, scattered))
            }
        }
    }
            //add texture glass 
            fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
                let cos_theta = (-uv).dot(n).min(1.0);
                let r_out_perp = etai_over_etat * (uv + cos_theta * n);
                let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
                r_out_perp + r_out_parallel
            }
        
            fn schlick(cosine: f64, ref_idx: f64) -> f64 {
                let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
                r0 = r0 * r0;
                r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
        }
    

    }

