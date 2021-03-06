use std::f32::{MAX as f32_MAX, MIN as f32_MIN};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
        Vec3 {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
        }
    }

    pub fn dot(u: Vec3, v: Vec3) -> f32 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn test_eq() {
        assert_eq!(Vec3::new(1.0, -2.0, 0.0), Vec3::new(1.0, -2.0, 0.0));
        assert_ne!(Vec3::new(1.0, -2.0, 0.0), Vec3::new(-1.0, 0.0, 0.0));
    }

    #[test]
    fn test_add() {
        let vec1 = Vec3::new(1.0, 2.0, 3.0) + Vec3::new(4.0, -5.0, 0.0);
        let vec2 = Vec3::new(5.0, -3.0, 3.0);
        assert_eq!(vec1, vec2);
    }

    #[test]
    fn test_sub() {
        let vec1 = Vec3::new(1.0, 2.0, 3.0) - Vec3::new(4.0, -5.0, 0.0);
        let vec2 = Vec3::new(-3.0, 7.0, 3.0);
        assert_eq!(vec1, vec2);
    }

    #[test]
    fn test_mul() {
        assert_eq!(Vec3::new(1.0, -2.0, 0.0) * 5.0, Vec3::new(5.0, -10.0, 0.0));
    }

    #[test]
    fn test_elem_mul() {
        assert_eq!(
            Vec3::new(1.0, -2.0, 0.0) * Vec3::new(1.0, -2.0, 0.0),
            Vec3::new(1.0, 4.0, 0.0)
        );
    }

    #[test]
    fn test_div() {
        assert_eq!(Vec3::new(1.0, -2.0, 0.0) / 2.0, Vec3::new(0.5, -1.0, 0.0));
    }

    #[test]
    fn test_neg() {
        assert_eq!(-Vec3::new(1.0, -2.0, 3.0), Vec3::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn test_dot() {
        assert_eq!(
            Vec3::dot(Vec3::new(1.0, 2.0, 3.0), Vec3::new(-2.0, 2.0, 3.0)),
            -2.0 + 4.0 + 9.0
        )
    }

    #[test]
    fn test_cross() {
        assert_eq!(
            Vec3::cross(Vec3::new(1.0, 2.0, 3.0), Vec3::new(2.0, 3.0, 4.0)),
            Vec3::new(8.0 - 9.0, 6.0 - 4.0, 3.0 - 4.0)
        )
    }

    #[test]
    fn test_length() {
        assert_eq!(Vec3::new(2.0, -2.0, 1.0).length(), 3.0);
    }

    #[test]
    fn test_unit() {
        assert_eq!(
            Vec3::new(2.0, -2.0, 1.0).unit_vector(),
            Vec3::new(2.0 / 3.0, -2.0 / 3.0, 1.0 / 3.0)
        );
    }
}
