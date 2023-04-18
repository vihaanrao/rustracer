//vec3 class is used to represent colors as a 3-tuple of floats (r,g,b), spatial coordinates (x,y,z)
use std::ops;
#[derive(Debug)]
pub struct Vec3 {
    e: [f32; 3],
}

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        Vec3 { 
            e: [e0, e1, e2]
         }
    }
}
//implement fn overload for + operator
impl ops::Add for Vec3 { 
    type Output = Vec3;

    fn add(self, rhs_: Vec3) -> Self::Output {
        Vec3::new(self.e[0] + rhs_.e[0], 
            self.e[1] + rhs_.e[1], 
            self.e[2] + rhs_.e[2])
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs_: Vec3) -> Self::Output {
        Vec3::new(self.e[0] * rhs_.e[0], 
            self.e[1] * rhs_.e[1], 
            self.e[2] * rhs_.e[2])
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vec3_add() {
        //test ray
    }

    fn test_vec3_multiply() {
        //test ray direction
    }

    fn test_vec3_divide() {
        //test point at parameter
    }
}