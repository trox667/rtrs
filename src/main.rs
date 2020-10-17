mod color;
mod ray;
mod vec3;

const IMAGE_WIDTH: usize = 256;
const IMAGE_HEIGHT: usize = 256;

fn main() {
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    (0..IMAGE_HEIGHT).rev().for_each(|j| {
        eprintln!("\rScanlines remaining: {}", j);
        (0..IMAGE_WIDTH).for_each(|i| {
            let pixel_color = color::Color::new(
                i as f32 / (IMAGE_WIDTH - 1) as f32,
                j as f32 / (IMAGE_HEIGHT - 1) as f32,
                0.25,
            );
            println!("{}", color::write_color(&pixel_color));
        });
    });
    eprintln!("Done.");
}
