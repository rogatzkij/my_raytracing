use std::ops;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3(f32, f32, f32);

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        return Self(x, y, z);
    }

    pub fn x(&self) -> f32 {
        return self.0;
    }
    pub fn y(&self) -> f32 {
        return self.1;
    }
    pub fn z(&self) -> f32 {
        return self.2;
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.x() * self.x() + self.1 * self.1 + self.z() * self.z()
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

// Arithmetic operations
impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.0 = self.x() + other.x();
        self.1 = self.y() + other.y();
        self.2 = self.z() + other.z();
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, other: f32) -> Vec3 {
        Self(self.x() * other, self.y() * other, self.z() * other)
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        Self(self.x() / other, self.y() / other, self.z() / other)
    }
}
