use std::ops::{Add, Mul, Div, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn normalize(self) -> Self {
        let n = 1.0 / self.mag();
        self * n
    }

    fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn mag2(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn mag(&self) -> f64 {
        self.mag2().sqrt()
    }

    pub fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn linear_combine(
        a: f64, vec_a: &Vec3,
        b: f64, vec_b: &Vec3,
        c: f64, vec_c: &Vec3,        
    ) -> Self {
        Self {
            x: a * vec_a.x + b * vec_b.x + c * vec_c.x,
            y: a * vec_a.y + b * vec_b.y + c * vec_c.y,
            z: a * vec_a.z + b * vec_b.z + c * vec_c.z,
        }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = f64;

    fn mul(self, rhs: Vec3) -> Self::Output {
        self.dot(rhs)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}


impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl From<(f64, f64, f64)> for Vec3 {
    fn from(vec: (f64, f64, f64)) -> Self {
        Self { x: vec.0, y: vec.1, z: vec.2 }
    }
}

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}