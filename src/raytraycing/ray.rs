pub struct Ray {
    pub orig: super::point3::Point3,
    pub dir: super::vec3::Vec3,
}

impl Ray {
    pub fn origin(self) -> super::point3::Point3 {
        return self.orig;
    }
    pub fn direction(self) -> super::vec3::Vec3 {
        return self.dir;
    }
    pub fn at(self, t: f32) -> super::point3::Point3 {
        return self.orig + self.dir * t;
    }
}
