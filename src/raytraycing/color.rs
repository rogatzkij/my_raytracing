use super::vec3::Vec3;

pub type Color = Vec3;

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

pub fn write_color(
    img: &mut image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>,
    x: u32,
    y: u32,
    pixel_color: &Color,
) {
    let pixel = img.get_pixel_mut(x, y);
    let image::Rgb(data) = *pixel;
    *pixel = image::Rgb([
        (pixel_color.r() * 255.999) as u8,
        (pixel_color.g() * 255.999) as u8,
        (pixel_color.b() * 255.999) as u8,
    ]);
}
