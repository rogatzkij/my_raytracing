pub type Point3 = super::vec3::Vec3;

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
