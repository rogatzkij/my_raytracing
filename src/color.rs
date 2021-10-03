use crate::vec3;

pub type Color = vec3::Vec3;

pub fn write_color(pixel: &Color) {
    let ir: u32 = (255.999 * pixel.e.0) as u32;
    let ig: u32 = (255.999 * pixel.e.1) as u32;
    let ib: u32 = (255.999 * pixel.e.2) as u32;

    println!("{} {} {}", ir, ig, ib);
}
