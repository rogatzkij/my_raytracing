mod raytraycing;

use image::{Pixel, Rgba, RgbaImage};

use raytraycing::color::{write_color, Color};
use raytraycing::point3::{dot, Point3};
use raytraycing::ray::{ray_color, Ray};

fn main() {
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 1024;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
    // Objects
    let sphere_center = &Point3::New(0.0, 0.0, -1.0);
    // Camera
    const VIEWPORT_HEIGHT: f32 = 2.0;
    const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f32 = 1.0;

    let ORIGIN = Point3::New(0.0, 0.0, 0.0);
    let HORIZONTAL = Point3::New(VIEWPORT_WIDTH, 0.0, 0.0);
    let VERTICAL = Point3::New(0.0, VIEWPORT_HEIGHT, 0.0);
    let LOWER_LEFT_CORNER =
        ORIGIN - HORIZONTAL / 2.0 - VERTICAL / 2.0 - Point3::New(0.0, 0.0, FOCAL_LENGTH);

    // Build image
    let mut imgbuf = image::ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("start!");
    for j in (0..(IMAGE_HEIGHT - 1)).rev() {
        for i in 0..IMAGE_WIDTH {
            let u = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let v = j as f32 / (IMAGE_HEIGHT - 1) as f32;

            let ray = &Ray::New(
                ORIGIN,
                LOWER_LEFT_CORNER + HORIZONTAL * u + VERTICAL * v - ORIGIN,
            );

            let mut pixel_color = Color::New(1.0, 0.0, 0.0);
            if !hit_sphere(sphere_center, 0.5, ray) {
                pixel_color = ray_color(ray);
            }

            let pixel = imgbuf.get_pixel_mut(i, j);
            let image::Rgb(data) = *pixel;
            *pixel = image::Rgb([
                (pixel_color.r() * 255.999) as u8,
                (pixel_color.g() * 255.999) as u8,
                (pixel_color.b() * 255.999) as u8,
            ]);
        }
    }
    imgbuf.save("picture.png").unwrap();
    println!("done!");
}

fn hit_sphere(center: &Point3, radius: f32, ray: &Ray) -> bool {
    let oc = &(ray.origin() - *center);
    let direction = &ray.direction();

    let a: f32 = dot(direction, direction);
    let b: f32 = 2.0 * dot(oc, direction);
    let c: f32 = dot(oc, oc) - radius * radius;

    let discriminant: f32 = b * b - 4.0 * a * c;
    return discriminant > 0.0;
}
