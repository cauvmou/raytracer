use crate::{math::{Vec3, Ray}, surface::{Scene, HitInfo}, color::Color, light::SceneLights, INV_PI};

pub trait Material {
    fn calc_mat(&self, prev_ray: &Ray, hit_position: Vec3, hit_normal: Vec3, scene: &Scene, lights: Option<&SceneLights>, bounce_count: usize) -> Option<HitInfo>;

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
    fn calc_mat(&self, prev_ray: &Ray, hit_position: Vec3, hit_normal: Vec3, scene: &Scene, lights: Option<&SceneLights>, bounce_count: usize) -> Option<HitInfo> {
        if let Some(lights) = lights {
            let mut color = self.color;
            for light in lights {
                let dir = light.direction(hit_position, hit_normal);

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
    diffuse_coeff: f64,
}

impl DiffuseMaterial {
    pub fn new(color: Color, diffuse_coeff: f64) -> Self {
        Self { 
            color,
            diffuse_coeff,
        }
    }
}

impl Material for DiffuseMaterial {
    fn calc_mat(&self, prev_ray: &Ray, hit_position: Vec3, hit_normal: Vec3, scene: &Scene, lights: Option<&SceneLights>, bounce_count: usize) -> Option<HitInfo> {
        let mut color: Color = 0.into();

        if let Some(lights) = lights {
            for light in lights {
                let dir = light.direction(hit_position, hit_normal);

                let normal_dot_light = hit_normal * dir;

                if normal_dot_light > 0.0 && !self.trace_shadow(
                    &Ray::new(hit_position, dir), scene, light.dist_to(hit_position)
                ) {
                    let diffusion = self.diffuse_coeff * normal_dot_light * INV_PI;
                    color = color + self.color * light.color(hit_position, hit_normal) * diffusion;
                }
            }
        } else {
            color = self.color;
        }

        Some(HitInfo::new(hit_position, hit_normal).tint(color))
    }
}

pub struct SDMaterial {
    color: Color,
    diffuse_coeff: f64,
    specular_coeff: f64,
    exponent: f64,
}

impl SDMaterial {
    pub fn new(color: Color, diffuse_coeff: f64, specular_coeff: f64, exponent: f64) -> Self {
        Self { 
            color,
            diffuse_coeff,
            specular_coeff,
            exponent,
        }
    }
}

impl Material for SDMaterial {
    fn calc_mat(&self, prev_ray: &Ray, hit_position: Vec3, hit_normal: Vec3, scene: &Scene, lights: Option<&SceneLights>, bounce_count: usize) -> Option<HitInfo> {
        let mut color: Color = [0, 0, 0].into();

        let mut reflection_dir = hit_normal;
        reflection_dir *= -2.0 * (prev_ray.direction * hit_normal);
        reflection_dir += prev_ray.direction;
        reflection_dir = reflection_dir.normalize();

        if let Some(lights) = lights {
            for light in lights {
                let dir = light.direction(hit_position, hit_normal);

                let normal_dot_light = hit_normal * dir;

                if normal_dot_light > 0.0 && !self.trace_shadow(
                    &Ray::new(hit_position, dir), scene, light.dist_to(hit_position)
                ) {
                    let diffusion = self.diffuse_coeff * normal_dot_light * INV_PI;
                    color = color + self.color * light.color(hit_position, hit_normal) * diffusion;

                    let reflection_dot_ray = -(reflection_dir * prev_ray.direction);
                    if reflection_dot_ray > 0.0 {
                        let spec = self.specular_coeff * normal_dot_light * reflection_dot_ray.powf(self.exponent);
                        color = color + light.color(hit_position, hit_normal) * spec;
                    }
                }
            }
        } else {
            color = self.color;
        }

        Some(HitInfo::new(hit_position, hit_normal).tint(color))
    }
}

pub struct SDRMaterial {
    color: Color,
    diffuse_coeff: f64,
    specular_coeff: f64,
    exponent: f64,
    reflection_coeff: f64,
}

impl SDRMaterial {
    pub fn new(color: Color, diffuse_coeff: f64, specular_coeff: f64, exponent: f64, reflection_coeff: f64) -> Self {
        Self { 
            color,
            diffuse_coeff,
            specular_coeff,
            exponent,
            reflection_coeff,
        }
    }
}

impl Material for SDRMaterial {
    fn calc_mat(&self, prev_ray: &Ray, hit_position: Vec3, hit_normal: Vec3, scene: &Scene, lights: Option<&SceneLights>, bounce_count: usize) -> Option<HitInfo> {
        let mut color: Color = [0, 0, 0].into();

        let mut reflection_dir = hit_normal;
        reflection_dir *= -2.0 * (prev_ray.direction * hit_normal);
        reflection_dir += prev_ray.direction;
        reflection_dir = reflection_dir.normalize();

        if let Some(lights) = lights {
            for light in lights {
                let dir = light.direction(hit_position, hit_normal);

                let normal_dot_light = hit_normal * dir;

                if normal_dot_light > 0.0 && !self.trace_shadow(
                    &Ray::new(hit_position, dir), scene, light.dist_to(hit_position)
                ) {
                    let diffusion = self.diffuse_coeff * normal_dot_light * INV_PI;
                    color = color + self.color * light.color(hit_position, hit_normal) * diffusion;

                    let reflection_dot_ray = -(reflection_dir * prev_ray.direction);
                    if reflection_dot_ray > 0.0 {
                        let spec = self.specular_coeff * normal_dot_light * reflection_dot_ray.powf(self.exponent);
                        color = color + light.color(hit_position, hit_normal) * spec;
                    }
                }
            }
        } else {
            color = self.color;
        }

        let reflection_ray = Ray::new(hit_position, reflection_dir);
        let mut min_distance = f64::INFINITY;
        let mut result: Color = 0.into();
        if bounce_count > 0 {
            for surface in scene {
                if let Some(info) = surface.hit(&reflection_ray, scene, lights, bounce_count-1, min_distance) {
                    min_distance = (info.position - hit_position).mag();
                    result = info.color();
                }
            }
        }

        color = color + result * self.reflection_coeff;

        Some(HitInfo::new(hit_position, hit_normal).tint(color))
    }
}