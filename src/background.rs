use image::{DynamicImage, GenericImageView};

use crate::{math::Vec3, INV_TAU, INV_PI, color::Color, surface::{Surface, HitInfo}, materials::Material};

pub struct BackgroundSurface {
    background: Box<dyn Material>,
}

impl BackgroundSurface {
    pub fn new(path: &str) -> Self {
        let image = match image::open(path) {
            Ok(image) => image,
            Err(_) => {
                let mut image = image::RgbImage::new(1, 1);
                image.put_pixel(0, 0, image::Rgb([0, 0, 0]));
                image.into()
            },
        };
        let width = image.width();
        let height = image.height();
        Self {
            background: Box::new(Background { image, width, height })
        }
    }
}

#[derive(Debug, Clone)]
struct Background {
    image: DynamicImage,
    width: u32,
    height: u32,
}

impl Background {
    pub fn sample(&self, vec: Vec3) -> Color {
        let vec = vec.normalize();
        let u = ((vec.x / vec.z).atan() + std::f64::consts::PI) * INV_TAU;
        let v = (vec.y.asin() + std::f64::consts::FRAC_PI_2) * INV_PI;

        let x = (self.width as f64 * u) as u32;
        let y = (self.height as f64 * v) as u32;

        self.image.get_pixel(x, y).0.into()
    }
}

impl Material for Background {
    fn calc_mat(&self, _prev_ray: &crate::math::Ray, _hit_position: Vec3, hit_normal: Vec3, _scene: &crate::surface::Scene, _lights: Option<&crate::light::SceneLights>, _bounce_count: usize) -> Option<crate::surface::HitInfo> {
        let color = self.sample(hit_normal);
        Some(HitInfo::new(hit_normal*f64::INFINITY, hit_normal).tint(color))
    }
}

impl Surface for BackgroundSurface {
    fn get_normal(&self, hit: &Vec3) -> Vec3 {
        -hit.normalize()
    }

    fn get_material(&self) -> &Box<dyn crate::materials::Material> {
        &self.background
    }

    fn surface_hit(&self, ray: &crate::math::Ray, min_distance: f64) -> Option<Vec3> {
        if min_distance >= f64::INFINITY {
            return Some(ray.direction)
        }
        None
    }

    fn shadow_hit(&self, _ray: &crate::math::Ray, _light_dist: f64) -> bool {
        false
    }
}