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
            let r = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let g = j as f32 / (IMAGE_HEIGHT - 1) as f32;
            let b = 0.25 as f32;
            let ir = (r * 255.999) as i32;
            let ig = (g * 255.999) as i32;
            let ib = (b * 255.999) as i32;

            println!("{} {} {}", ir, ig, ib);
        });
    });
    eprintln!("Done.");
}
