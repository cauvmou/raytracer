use crate::{math::{Vec3, Ray}, surface::{Scene, HitInfo}, color::Color, light::SceneLights, INV_PI};

pub trait Material {
    fn calc_mat(&self, hit_position: Vec3, hit_normal: Vec3, scene: &Scene, lights: Option<&SceneLights>, bounce_count: usize) -> Option<HitInfo>;

    fn trace_shadow(&self, ray: &Ray, scene: &Scene, light_dist: f64) -> bool {
        for surface in scene {
            if surface.shadow_hit(ray, light_dist) {
                return true
            }
        }
        false
    }
}

pub struct AlbedoMaterial {
    color: Color,
}

impl AlbedoMaterial {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Material for AlbedoMaterial {
    fn calc_mat(&self, hit_position: Vec3, hit_normal: Vec3, scene: &Scene, lights: Option<&SceneLights>, bounce_count: usize) -> Option<HitInfo> {
        if let Some(lights) = lights {
            let mut color = self.color;
            for light in lights {
                let dir = light.direction(hit_position);

                if self.trace_shadow(
                    &Ray::new(hit_position, dir), scene, light.dist_to(hit_position)
                ) {
                    color = 0.into();
                }
            }
            return Some(HitInfo::new(hit_position, hit_normal).tint(color))
        }
        return Some(HitInfo::new(hit_position, hit_normal).tint(self.color)) 
    }
}

pub struct DiffuseMaterial {
    color: Color,
    diffusion: f64,
}

impl DiffuseMaterial {
    pub fn new(color: Color, diffusion: f64,) -> Self {
        Self { 
            color,
            diffusion,
        }
    }
}

impl Material for DiffuseMaterial {
    fn calc_mat(&self, hit_position: Vec3, hit_normal: Vec3, scene: &Scene, lights: Option<&SceneLights>, bounce_count: usize) -> Option<HitInfo> {
        let mut color: Color = 0.into();

        if let Some(lights) = lights {
            for light in lights {
                let dir = light.direction(hit_position);

                let normal_dot_light = hit_normal * dir;

                if normal_dot_light > 0.0 && !self.trace_shadow(
                    &Ray::new(hit_position, dir), scene, light.dist_to(hit_position)
                ) {
                    let diffusion = self.diffusion * normal_dot_light * INV_PI;
                    color = color + self.color * light.color(hit_position, hit_normal) * diffusion;
                }
            }
        } else {
            color = self.color;
        }

        Some(HitInfo::new(hit_position, hit_normal).tint(color))
    }
}