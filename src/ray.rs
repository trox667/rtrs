use super::vec3;

#[derive(Default, Debug, Copy, Clone)]
pub struct Ray {
    origin: vec3::Point3,
    direction: vec3::Vec3,
}

impl Ray {
    pub fn new(origin: &vec3::Point3, direction: &vec3::Vec3) -> Self {
        Ray {
            origin: origin.clone(),
            direction: direction.clone(),
        }
    }

    pub fn origin(&self) -> vec3::Point3 {
        self.origin.clone()
    }

    pub fn direction(&self) -> vec3::Vec3 {
        self.direction.clone()
    }

    pub fn point_at(&self, t: f32) -> vec3::Point3 {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn point_at() {
        let reference = crate::vec3::Point3::new(2.0, 0.0, 0.0);
        let p = crate::vec3::Point3::new(0.0, 0.0, 0.0);
        let dir = crate::vec3::Vec3::new(1.0, 0.0, 0.0);
        let ray = super::Ray::new(&p, &dir);
        assert_eq!(reference, ray.point_at(2.0));
    }
}
