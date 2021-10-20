mod raytraycing;

use image;

use raytraycing::color::{write_color, Color};
use raytraycing::point3::{dot, Point3};
use raytraycing::ray::{ray_color, Ray};

use raytraycing::hittable_list::HittableList;

fn main() {
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 1024;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
    // Objects
    let sphere_center = Point3::new(0.0, 0.0, -1.0);
    let sphere_radius = 0.5;
    // Camera
    const VIEWPORT_HEIGHT: f32 = 2.0;
    const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f32 = 1.0;

    let ORIGIN = Point3::new(0.0, 0.0, 0.0);
    let HORIZONTAL = Point3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let VERTICAL = Point3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let LOWER_LEFT_CORNER =
        ORIGIN - HORIZONTAL / 2.0 - VERTICAL / 2.0 - Point3::new(0.0, 0.0, FOCAL_LENGTH);

    // Build image
    let mut imgbuf = image::ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("start!");
    for j in (0..(IMAGE_HEIGHT - 1)).rev() {
        for i in 0..IMAGE_WIDTH {
            let u = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let v = j as f32 / (IMAGE_HEIGHT - 1) as f32;

            let ray = Ray::new(
                ORIGIN,
                LOWER_LEFT_CORNER + HORIZONTAL * u + VERTICAL * v - ORIGIN,
            );

            let t: f32 = hit_sphere(&sphere_center, sphere_radius, &ray);
            let pixel_color: Color;
            if t > 0.0 {
                let n: Point3 = ray.at(t) - Point3::new(0.0, 0.0, -1.0);
                pixel_color = Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) * 0.5;
            } else {
                pixel_color = ray_color(&ray);
            }

            let x: u32 = i;
            let y: u32 = IMAGE_HEIGHT - 1 - j;
            write_color(&mut imgbuf, x, y, &pixel_color);
        }
    }
    imgbuf.save("picture.png").unwrap();
    println!("done!");

    HittableList::new();
}

fn hit_sphere(center: &Point3, radius: f32, ray: &Ray) -> f32 {
    let oc = ray.origin() - *center;
    let direction = ray.direction();

    let a: f32 = dot(&direction, &direction);
    let b: f32 = 2.0 * dot(&oc, &direction);
    let c: f32 = dot(&oc, &oc) - radius * radius;

    let discriminant: f32 = b * b - 4.0 * a * c;
    if discriminant > 0.0 {
        return -b * discriminant.sqrt() / 2.0 * a;
    }
    return -1.0;
}
