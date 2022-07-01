use crate::{surface::HitInfo, math::Vec3};

pub type SceneLights = Vec<Box<dyn Light>>;
pub static EPSILON: f64 = 0.02;

pub trait Light {
    fn direction(&self, hit: &HitInfo) -> Vec3;
}

pub struct DirectionalLight {
    pub direction: Vec3,
}

impl DirectionalLight {
    pub fn new(direction: Vec3) -> Self {
        Self {
            direction,
        }
    }
}

impl Light for DirectionalLight {
    fn direction(&self, hit: &HitInfo) -> Vec3 {
        self.direction
    }
}