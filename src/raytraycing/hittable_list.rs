use super::hittable::{HitRecord, Hittable};
use super::point3::Point3;
use super::ray::Ray;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj)
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &HitRecord) -> Option<HitRecord> {
        let temp_rec = rec;
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            match object.hit(r, t_min, closest_so_far, &temp_rec) {
                Some(temp_rec) => {
                    hit_anything = true;
                    closest_so_far = temp_rec.t;
                }
                None => {}
            }
        }
        if !hit_anything {
            return None;
        }
        return Some(*temp_rec);
    }
}
