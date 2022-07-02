use std::ops::{Add, Mul};

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self {
            r,g,b
        }
    }
}

impl Add<Color> for Color {
    type Output = Self;

    fn add(self, rhs: Color) -> Self::Output {
        Self::new(
            self.r + rhs.r,
            self.g + rhs.g,
            self.b + rhs.b,
        )
    }
}

impl Mul<Color> for Color {
    type Output = Self;

    fn mul(self, rhs: Color) -> Self::Output {
        Self::new(
            self.r * rhs.r,
            self.g * rhs.g,
            self.b * rhs.b,
        )
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(
            self.r * rhs,
            self.g * rhs,
            self.b * rhs,
        )
    }
}

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        let red = (value & 0xff000000) >> 6;
        let green = (value & 0xff0000) >> 4;
        let blue = (value & 0xff00) >> 2;
        [red as u8, green as u8, blue as u8].into()
    }
}

impl From<[u8; 3]> for Color {
    fn from(color: [u8; 3]) -> Self {
        Self::new(
            color[0] as f64 / 255.0,
            color[1] as f64 / 255.0,
            color[2] as f64 / 255.0,
        )
    }
}

impl From<[u8; 4]> for Color {
    fn from(color: [u8; 4]) -> Self {
        Self::new(
            color[0] as f64 / 255.0,
            color[1] as f64 / 255.0,
            color[2] as f64 / 255.0,
        )
    }
}

impl Into<[u8; 3]> for Color {
    fn into(self) -> [u8; 3] {
        [(self.r * 255.0) as u8, (self.g * 255.0) as u8, (self.b * 255.0) as u8]
    }
}