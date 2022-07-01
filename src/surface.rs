use crate::{math::{Ray, Vec3}, materials::Material};

pub type Scene = Vec<Box<dyn Surface>>;

pub trait Surface {

    fn hit(&self, ray: &Ray, scene: &Scene, bounce_count: usize, min_distance: f64) -> Option<HitInfo> {
        match self.surface_hit(ray, min_distance) {
            Some(hit) => {
                if (hit - ray.origin).mag() < min_distance {
                    self.get_material().calc_mat(hit, scene, bounce_count)
                } else {
                    None
                }
            },
            None => None,
        }
    }

    fn shadow(&self, ray: &Ray) -> bool {
        self.shadow_hit(ray)
    }

    fn get_normal(&self, hit: &Vec3) -> Vec3;

    fn get_material(&self) -> &Box<dyn Material>;

    fn surface_hit(&self, ray: &Ray, min_distance: f64) -> Option<Vec3>;

    fn shadow_hit(&self, ray: &Ray) -> bool;
}

pub struct HitInfo {
    pub color: [f64; 3],
    pub position: Vec3,
}

impl HitInfo {
    pub fn to_color_rgb(&self) -> [u8; 3] {
        [
            (self.color[0] * 255.0) as u8,
            (self.color[1] * 255.0) as u8,
            (self.color[2] * 255.0) as u8,
        ]
    }
}