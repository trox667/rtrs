mod color;
mod hittable;
mod hittable_list;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

use std::sync::Arc;
use std::thread;

fn ray_color(r: &ray::Ray, world: &dyn hittable::Hittable) -> color::Color {
    let mut hit_record = hittable::HitRecord::default();
    if world.hit(r, 0.0, rtweekend::INFINITY, &mut hit_record) {
        return (hit_record.normal + color::Color::new(1.0, 1.0, 1.0)) * 0.5;
    }
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    color::Color::new(1.0, 1.0, 1.0) * (1.0 - t) + color::Color::new(0.5, 0.7, 1.0) * t
}

fn main() {
    // image
    let aspect_ratio: f32 = 16.0 / 9.0;

    let image_width: usize = 4000;
    let image_height: usize = (image_width as f32 / aspect_ratio).floor() as usize;

    // world
    let mut world = hittable_list::HittableList::default();
    world.add(Arc::new(sphere::Sphere::new(
        &vec3::Point3::new(0.0, 0.0, -1.0),
        0.5,
    )));
    world.add(Arc::new(sphere::Sphere::new(
        &vec3::Point3::new(0.0, -100.5, -1.0),
        100.0,
    )));

    // camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = vec3::Point3::default();
    let horizontal = vec3::Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = vec3::Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - vec3::Vec3::new(0.0, 0.0, focal_length);

    // render
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    let multithreading = false;

    // MT Solution
    if multithreading {
        let mut positions = vec![];
        for j in (0..image_height).rev() {
            for i in 0..image_width {
                positions.push((j, i));
            }
        }

        let chunks = positions
            .chunks(1000)
            .map(|chunk| {
                let mut v = Vec::new();
                v.extend_from_slice(chunk);
                return v;
            })
            .collect::<Vec<Vec<(usize, usize)>>>();

        let mut threads = vec![];
        chunks.iter().for_each(|chunk| {
            let world = Arc::new(world.clone());
            let chunk = chunk.clone();
            threads.push(thread::spawn(move || {
                chunk
                    .iter()
                    .map(|(j, i)| {
                        let u = *i as f32 / (image_width - 1) as f32;
                        let v = *j as f32 / (image_height - 1) as f32;
                        let r = ray::Ray::new(
                            &origin,
                            &(lower_left_corner + horizontal * u + vertical * v - origin),
                        );
                        let pixel_color = ray_color(&r, world.as_ref());
                        (j * image_width + i, pixel_color)
                    })
                    .collect::<Vec<(usize, color::Color)>>()
            }));
        });

        let mut pixels = vec![];
        for thread in threads {
            let data = thread.join().unwrap();
            pixels.push(data);
        }
        let mut res: Vec<(usize, color::Color)> = pixels.into_iter().flatten().collect();
        res.sort_by(|(j, c1), (i, c2)| j.cmp(i));
        res.iter().rev().for_each(|(_, color)| {
            println!("{}", color::write_color(&color));
        });
    } else {
        (0..image_height).rev().for_each(|j| {
            // eprintln!("\rScanlines remaining: {}", j);
            (0..image_width).for_each(|i| {
                let u = i as f32 / (image_width - 1) as f32;
                let v = j as f32 / (image_height - 1) as f32;
                let r = ray::Ray::new(
                    &origin,
                    &(lower_left_corner + horizontal * u + vertical * v - origin),
                );
                let pixel_color = ray_color(&r, &world);

                println!("{}", color::write_color(&pixel_color));
            });
        });
    }
    eprintln!("Done.");
}
