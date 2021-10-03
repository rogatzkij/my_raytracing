mod color;
mod vec3;

fn main() {
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    println!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in (0..(IMAGE_HEIGHT - 1)).rev() {
        for i in 0..IMAGE_WIDTH {
            let r: f32 = i as f32 / IMAGE_WIDTH as f32;
            let g: f32 = j as f32 / IMAGE_HEIGHT as f32;
            let b: f32 = 0.25;

            let pixel = color::Color::New(r, g, b);
            color::write_color(&pixel);
        }
    }
}
