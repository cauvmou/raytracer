use crate::{math::Vec3, surface::{Scene, HitInfo}};

pub trait Material {
    fn calc_mat(&self, hit: Vec3, scene: &Scene, bounce_count: usize) -> Option<HitInfo>;
}

pub struct AlbedoMaterial {
    color: [f64; 3],
}

impl AlbedoMaterial {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { color: [
            (r as f64 / 255.0),
            (g as f64 / 255.0),
            (b as f64 / 255.0),
        ] }
    }
}

impl Material for AlbedoMaterial {
    fn calc_mat(&self, hit: Vec3, scene: &Scene, bounce_count: usize) -> Option<HitInfo> {
        Some(HitInfo {
            color: self.color,
            position: hit,
        })
    }
}