pub struct Vec3 {
    pub e: (f32, f32, f32),
}

impl Vec3 {
    pub fn New(x: f32, y: f32, z: f32) -> Vec3 {
        return Vec3 { e: (x, y, z) };
    }
}
