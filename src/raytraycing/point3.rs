use super::vec3::Vec3;

pub type Point3 = Vec3;

pub fn dot(u: &Vec3, v: &Vec3) -> f32 {
    u.x() * v.x() + u.y() * v.y() + u.z() * v.z()
}
