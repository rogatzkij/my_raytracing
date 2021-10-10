use super::color::Color;
use super::point3::Point3;
use super::vec3::{unit_vector, Vec3};

pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn New(orig: Point3, dir: Vec3) -> Self {
        Self {
            orig: orig,
            dir: dir,
        }
    }
    pub fn origin(self) -> Point3 {
        return self.orig;
    }
    pub fn direction(self) -> Vec3 {
        return self.dir;
    }
    pub fn at(self, t: f32) -> Point3 {
        return self.orig + self.dir * t;
    }
}

pub fn ray_color(r: Ray) -> Color {
    let unit_direction: Vec3 = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);

    return Color::New(1.0, 1.0, 1.0) * (1.0 - t) + Color::New(0.5, 0.7, 1.0) * t;
}
