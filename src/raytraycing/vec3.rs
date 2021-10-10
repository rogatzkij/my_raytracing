use std::ops;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub e: (f32, f32, f32),
}

impl Vec3 {
    pub fn New(x: f32, y: f32, z: f32) -> Vec3 {
        return Vec3 { e: (x, y, z) };
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Vec3 {
        Self {
            e: (self.e.0 * other, self.e.1 * other, self.e.2 * other),
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            e: (
                self.e.0 + other.e.0,
                self.e.1 + other.e.1,
                self.e.2 + other.e.2,
            ),
        }
    }
}
