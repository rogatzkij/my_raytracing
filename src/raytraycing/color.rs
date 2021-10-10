pub type Color = super::vec3::Vec3;

impl Color {
    pub fn r(&self) -> f32 {
        return self.e.0;
    }
    pub fn g(&self) -> f32 {
        return self.e.1;
    }
    pub fn b(&self) -> f32 {
        return self.e.2;
    }
}

pub fn write_color(pixel: &Color) {
    let ir: u32 = (255.999 * pixel.r()) as u32;
    let ig: u32 = (255.999 * pixel.g()) as u32;
    let ib: u32 = (255.999 * pixel.b()) as u32;

    println!("{} {} {}", ir, ig, ib);
}
