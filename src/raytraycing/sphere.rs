use super::hittable::{HitRecord, Hittable};
use super::ray;

use super::point3::{dot, Point3};

pub struct Sphere {
    center: Point3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &ray::Ray, t_min: f32, t_max: f32, rec: &HitRecord) -> Option<HitRecord> {
        let oc = r.origin() - self.center;

        let a = r.direction().length_squared();
        let half_b = dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant: f32 = half_b - a * c;
        if discriminant < 0.0 {
            return None;
        };

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let outward_normal = (r.at(rec.t) - self.center) / self.radius;
        let mut result = HitRecord {
            p: r.at(rec.t),
            normal: (rec.p - self.center) / self.radius,
            t: root,
            front_face: true,
        };
        result.set_face_normal(r, &outward_normal);

        return Some(result);
    }
}
