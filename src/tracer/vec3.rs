use std::f32::{MAX as f32_MAX, MIN as f32_MIN};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;
    #[test]
    fn test_add() {
        let vec1 = Vec3::new(1.0, 2.0, 3.0) + Vec3::new(1.0, -3.0, 1.0);
        let vec2 = Vec3::new(2.0, -1.0, 4.0);
        assert_eq!(vec1, vec2)
    }
}
