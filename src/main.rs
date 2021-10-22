mod raytraycing;

use image;
use rand::prelude::*;

use raytraycing::camera::Camera;
use raytraycing::color::Color;
use raytraycing::hittable_list::HittableList;
use raytraycing::point3::Point3;
use raytraycing::sphere::Sphere;

use raytraycing::color::write_color;
use raytraycing::ray::ray_color;

fn main() {
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 800;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
    // World
    let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
    let earth = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0);

    let mut world = HittableList::new();
    world.add(Box::new(sphere));
    world.add(Box::new(earth));

    // Camera
    let camera = Camera::new();
    let samples_per_pixel = 100;

    // Build image
    let mut imgbuf = image::ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    let mut rng = rand::thread_rng();

    println!("start!");
    for j in (0..(IMAGE_HEIGHT - 1)).rev() {
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u_delta: f32 = rng.gen();
                let v_delta: f32 = rng.gen();

                let u = (i as f32 + u_delta) / (IMAGE_WIDTH - 1) as f32;
                let v = (j as f32 + v_delta) / (IMAGE_HEIGHT - 1) as f32;

                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world);
            }
            pixel_color = pixel_color / samples_per_pixel as f32;

            let x: u32 = i;
            let y: u32 = IMAGE_HEIGHT - 1 - j;
            write_color(&mut imgbuf, x, y, &pixel_color);
        }
    }
    imgbuf.save("picture.png").unwrap();
    println!("done!");

    HittableList::new();
}
