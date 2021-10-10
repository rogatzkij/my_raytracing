use std::ops;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub e: (f32, f32, f32),
}

impl Vec3 {
    pub fn New(x: f32, y: f32, z: f32) -> Vec3 {
        return Vec3 { e: (x, y, z) };
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f32 {
        self.e.0 * self.e.0 + self.e.1 * self.e.1 + self.e.2 * self.e.2
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

// Arithmetic operations
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

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            e: (
                self.e.0 - other.e.0,
                self.e.1 - other.e.1,
                self.e.2 - other.e.2,
            ),
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, other: f32) -> Vec3 {
        Self {
            e: (self.e.0 * other, self.e.1 * other, self.e.2 * other),
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        Self {
            e: (self.e.0 / other, self.e.1 / other, self.e.2 / other),
        }
    }
}
