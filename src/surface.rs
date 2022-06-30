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

    fn get_material(&self) -> &Box<dyn Material>;

    fn surface_hit(&self, ray: &Ray, min_distance: f64) -> Option<Vec3>;
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