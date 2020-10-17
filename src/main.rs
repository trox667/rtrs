mod color;
mod ray;
mod vec3;

fn hit_sphere(center: &vec3::Point3, radius: f32, r: &ray::Ray) -> bool {
    let oc = r.origin() - *center;
    let a = r.direction().dot(r.direction());
    let b = r.direction().dot(oc) * 2.0;
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - a * 4.0 * c;

    discriminant > 0.0
}

fn ray_color(r: &ray::Ray) -> color::Color {
    if hit_sphere(&vec3::Point3::new(0.0, 0.0, -1.0), 0.5, r) {
        return color::Color::new(1.0, 0.0, 0.0);
    }
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    color::Color::new(1.0, 1.0, 1.0) * (1.0 - t) + color::Color::new(0.5, 0.7, 1.0) * t
}

fn main() {
    // image
    let aspect_ratio: f32 = 16.0 / 9.0;

    let image_width: usize = 400;
    let image_height: usize = (image_width as f32 / aspect_ratio).floor() as usize;

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

    (0..image_height).rev().for_each(|j| {
        eprintln!("\rScanlines remaining: {}", j);
        (0..image_width).for_each(|i| {
            let u = i as f32 / (image_width - 1) as f32;
            let v = j as f32 / (image_height - 1) as f32;
            let r = ray::Ray::new(
                &origin,
                &(lower_left_corner + horizontal * u + vertical * v - origin),
            );
            let pixel_color = ray_color(&r);

            println!("{}", color::write_color(&pixel_color));
        });
    });
    eprintln!("Done.");
}
