use super::hittable::{HitRecord, Hittable};
use super::ray;
use super::vec3;

#[derive(Debug, Default, Copy, Clone)]
pub struct Sphere {
    center: vec3::Point3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: &vec3::Point3, radius: f32) -> Self {
        Self {
            center: center.clone(),
            radius,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &ray::Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;

        // https://de.wikipedia.org/wiki/Diskriminante
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        // https://de.wikipedia.org/wiki/Quadratische_Gleichung#L%C3%B6sungsformel_f%C3%BCr_die_allgemeine_quadratische_Gleichung_(a-b-c-Formel)
        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                hit_record.t = temp;
                hit_record.p = r.point_at(hit_record.t);
                let outward_normal = (hit_record.p - self.center) / self.radius;
                hit_record.set_face_normal(r, &outward_normal);
                return true;
            }

            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                hit_record.t = temp;
                hit_record.p = r.point_at(hit_record.t);
                let outward_normal = (hit_record.p - self.center) / self.radius;
                hit_record.set_face_normal(r, &outward_normal);
                return true;
            }
        }

        false
    }
}
