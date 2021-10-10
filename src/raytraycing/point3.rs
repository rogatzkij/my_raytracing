use super::vec3::Vec3;

pub type Point3 = Vec3;

impl Point3 {
    pub fn x(&self) -> f32 {
        return self.e.0;
    }
    pub fn y(&self) -> f32 {
        return self.e.1;
    }
    pub fn z(&self) -> f32 {
        return self.e.2;
    }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f32 {
    u.e.0 * v.e.0 + u.e.1 * v.e.1 + u.e.2 * v.e.2
}
