mod raytraycing;

use image;

use raytraycing::camera::Camera;
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

    // Build image
    let mut imgbuf = image::ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("start!");
    for j in (0..(IMAGE_HEIGHT - 1)).rev() {
        for i in 0..IMAGE_WIDTH {
            let u = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let v = j as f32 / (IMAGE_HEIGHT - 1) as f32;

            let x: u32 = i;
            let y: u32 = IMAGE_HEIGHT - 1 - j;
            let ray = camera.get_ray(u, v);
            let pixel_color = ray_color(&ray, &world);
            write_color(&mut imgbuf, x, y, &pixel_color);
        }
    }
    imgbuf.save("picture.png").unwrap();
    println!("done!");

    HittableList::new();
}
