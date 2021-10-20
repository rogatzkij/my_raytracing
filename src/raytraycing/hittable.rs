use super::point3::{dot, Point3};
use super::ray::Ray;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Point3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Point3) {
        self.front_face = dot(&r.direction(), outward_normal) < 0.0;

        if self.front_face {
            self.normal = *outward_normal * -1.0;
        }
    }
}

impl Default for HitRecord {
    fn default() -> HitRecord {
        HitRecord {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Point3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &HitRecord) -> Option<HitRecord>;
}
