mod raytraycing;

use raytraycing::color::write_color;
use raytraycing::point3::Point3;
use raytraycing::ray::{ray_color, Ray};

fn main() {
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;

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
    println!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in (0..(IMAGE_HEIGHT - 1)).rev() {
        for i in 0..IMAGE_WIDTH {
            let u = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let v = j as f32 / (IMAGE_HEIGHT - 1) as f32;

            let ray = Ray::New(
                ORIGIN,
                LOWER_LEFT_CORNER + HORIZONTAL * u + VERTICAL * v - ORIGIN,
            );

            let pixel_color = ray_color(ray);
            write_color(&pixel_color);
        }
    }
}
