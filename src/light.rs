use crate::{surface::HitInfo, math::Vec3, color::Color};

pub type SceneLights = Vec<Box<dyn Light>>;
pub static EPSILON: f64 = 0.02;

pub trait Light {
    fn direction(&self, hit: Vec3) -> Vec3;

    fn dist_to(&self, point: Vec3) -> f64;

    fn color(&self, position: Vec3, normal: Vec3) -> Color;
}

pub struct DirectionalLight {
    pub direction: Vec3,
    color: Color,
}

impl DirectionalLight {
    pub fn new(direction: Vec3, color: Color, brightness: f64) -> Self {
        Self {
            direction: direction.normalize(),
            color: color * brightness,
        }
    }
}

impl Light for DirectionalLight {
    fn direction(&self, hit: Vec3) -> Vec3 {
        self.direction
    }

    fn dist_to(&self, point: Vec3) -> f64 {
        f64::INFINITY
    }

    fn color(&self, position: Vec3, normal: Vec3) -> Color {
        self.color
    }
}

pub struct PointLight {
    pub point: Vec3,
    color: Color,
    falloff: bool,
}

impl PointLight {
    pub fn new(point: Vec3, color: Color, brightness: f64) -> Self {
        Self { 
            point,
            color: color * brightness,
            falloff: false,
        }
    }

    pub fn with_falloff(mut self) -> Self {
        self.falloff = true;
        self
    }
}

impl Light for PointLight {
    fn direction(&self, hit: Vec3) -> Vec3 {
        (self.point - hit).normalize()
    }

    fn dist_to(&self, point: Vec3) -> f64 {
        (self.point - point).mag()
    }

    fn color(&self, position: Vec3, normal: Vec3) -> Color {
        if self.falloff {
            self.color * (1.0 / (self.point - position).mag2())
        } else {
            self.color
        }
    }
}