use crate::{math::{Ray, Vec3}, materials::Material, color::Color, light::SceneLights};

pub type Scene = Vec<Box<dyn Surface>>;

pub trait Surface {

    fn hit(&self, ray: &Ray, scene: &Scene, lights: Option<&SceneLights>, bounce_count: usize, min_distance: f64) -> Option<HitInfo> {
        if let Some(hit) = self.surface_hit(ray, min_distance) {
            if (hit - ray.origin).mag() < min_distance {
                return self.get_material().calc_mat(ray, hit, self.get_normal(&hit), scene, lights, bounce_count)
            }
        }
        None
    }

    fn get_normal(&self, hit: &Vec3) -> Vec3;

    fn get_material(&self) -> &Box<dyn Material>;

    fn surface_hit(&self, ray: &Ray, min_distance: f64) -> Option<Vec3>;

    fn shadow_hit(&self, ray: &Ray, light_dist: f64) -> bool;
}

#[derive(Debug, Copy, Clone)]
pub struct HitInfo {
    color: Option<Color>,
    pub position: Vec3,
    pub normal: Vec3,
}

impl HitInfo {
    pub fn new(position: Vec3, normal: Vec3) -> Self {
        Self {
            position,
            normal,
            color: None,
        }
    }

    pub fn tint(&mut self, color: Color) -> Self {
        self.color = Some(color);
        *self
    }

    pub fn color(&self) -> Color{
        if let Some(color) = self.color {
            color
        } else {
            0.into()
        }
    }
}