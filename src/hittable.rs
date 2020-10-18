use super::ray;
use super::vec3;

pub trait Hittable {
    fn hit(&self, r: &ray::Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool;
}

#[derive(Debug, Default, Copy, Clone)]
pub struct HitRecord {
    pub p: vec3::Point3,
    pub normal: vec3::Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: &vec3::Point3, normal: &vec3::Vec3, t: f32) -> Self {
        Self {
            p: p.clone(),
            normal: normal.clone(),
            t,
            front_face: true,
        }
    }

    pub fn copy_from(&mut self, other: &HitRecord) {
        self.p = other.p.clone();
        self.normal = other.normal.clone();
        self.t = other.t.clone();
        self.front_face = other.front_face.clone();
    }

    pub fn set_face_normal(&mut self, r: &ray::Ray, outward_normal: &vec3::Vec3) {
        self.front_face = if r.direction().dot(*outward_normal) < 0.0 {
            true
        } else {
            false
        };
        self.normal = if self.front_face {
            outward_normal.clone()
        } else {
            outward_normal.negate()
        };
    }
}
