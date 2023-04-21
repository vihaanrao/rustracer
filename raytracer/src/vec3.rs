//vec3 class is used to represent colors as a 3-tuple of floats (r,g,b), spatial coordinates (x,y,z)
use std::ops;
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    e: [f32; 3],
}

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        Vec3 { 
            e: [e0, e1, e2]
         }
    }

    pub fn x(self) -> f32 {
        self.e[0]
    }

    pub fn y(self) -> f32 {
        self.e[1]
    }

    pub fn z(self) -> f32 {
        self.e[2]
    }

    pub fn length(self) -> f32 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()
    }

    pub fn unitVector(v: &Vec3) -> Vec3 {
        *v / v.length()
    }

    pub fn r(self) -> f32 {
        self.e[0]
    }

    pub fn g(self) -> f32 {
        self.e[1]
    }

    pub fn b(self) -> f32 {
        self.e[2]
    }

}
//implement fn overload for + operator
impl ops::Add for Vec3 { 
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            self.e[0] + rhs.e[0], 
            self.e[1] + rhs.e[1], 
            self.e[2] + rhs.e[2])
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            e: [self.e[0] * rhs, 
                self.e[1] * rhs, 
                self.e[2] * rhs]
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3 {
            e: [self.e[0] / rhs, 
                self.e[1] / rhs, 
                self.e[2] / rhs]
        }
    }
}


//test vec3
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vec3_add() {
        //test ray
        assert_eq!(
        //add passing test
            Vec3::new(1.0, 2.0, 3.0) + Vec3::new(1.0, 2.0, 3.0),
        //add failing test
            Vec3::new(6.0, 5.0, 11.0)
        ); 
    }
    //#[allow(dead_code)]
    #[test]
    fn test_vec3_mul() {
        //test ray direction
        //add failing test
        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0) * 2.0, Vec3::new(2.0, 4.0, 6.0),
        );
    }
    
    #[test]
    fn test_vec3_div() {
        //test point at parameter
        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0) / 2.0, Vec3::new(0.5, 1.0, 1.5), //passing test
        );

    }
}