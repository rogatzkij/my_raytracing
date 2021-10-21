use super::hittable::{HitRecord, Hittable};
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
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut result: HitRecord = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            match object.hit(r, t_min, closest_so_far) {
                Some(hit_record) => {
                    hit_anything = true;
                    closest_so_far = hit_record.t;

                    result = HitRecord {
                        front_face: hit_record.front_face,
                        t: hit_record.t,
                        p: hit_record.p,
                        normal: hit_record.normal,
                    };
                }
                None => {}
            }
        }
        if !hit_anything {
            return None;
        }
        return Some(result);
    }
}
