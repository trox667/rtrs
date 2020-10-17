use std::ops::{Add, Div, Mul, Sub};

pub type Point3 = Vec3;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    e: [f32; 3],
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            e: [
                self.x() + other.x(),
                self.y() + other.y(),
                self.z() + other.z(),
            ],
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            e: [
                self.x() - other.x(),
                self.y() - other.y(),
                self.z() - other.z(),
            ],
        }
    }
}

impl Div for Vec3 {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self {
            e: [
                self.x() / other.x(),
                self.y() / other.y(),
                self.z() / other.z(),
            ],
        }
    }
}
impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, other: f32) -> Self {
        Self {
            e: [self.x() / other, self.y() / other, self.z() / other],
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            e: [
                self.x() * other.x(),
                self.y() * other.y(),
                self.z() * other.z(),
            ],
        }
    }
}
impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        Self {
            e: [self.x() * other, self.y() * other, self.z() * other],
        }
    }
}

impl Vec3 {
    pub fn empty() -> Self {
        Self { e: [0.0, 0.0, 0.0] }
    }

    pub fn new(e0: f32, e1: f32, e2: f32) -> Self {
        Self { e: [e0, e1, e2] }
    }

    pub fn x(&self) -> f32 {
        self.e[0]
    }

    pub fn y(&self) -> f32 {
        self.e[1]
    }

    pub fn z(&self) -> f32 {
        self.e[2]
    }

    pub fn negate(&self) -> Self {
        Self {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }

    pub fn get(&self, i: usize) -> f32 {
        self.e[i]
    }

    pub fn length_squared(&self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn length(&self) -> f32 {
        Self::length_squared(self).sqrt()
    }

    pub fn dot(&self, v: Self) -> Self {
        Self {
            e: [self.x() * v.x(), self.y() * v.y(), self.z() * v.z()],
        }
    }

    pub fn cross(&self, v: Self) -> Self {
        Self {
            e: [
                self.y() * v.z() - self.z() * v.y(),
                self.z() * v.x() - self.x() * v.z(),
                self.x() * v.y() - self.y() * v.x(),
            ],
        }
    }

    pub fn unit_vector(self) -> Self {
        let l = self.length();
        self / l
    }
}
