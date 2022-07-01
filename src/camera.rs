use std::{path::Iter, io};

use image::{ImageFormat, Rgb, ImageError};

use crate::{math::{Vec3, Ray}, surface::{Surface, Scene, HitInfo}, light::SceneLights};

pub struct Camera {
    eye: Vec3,
    look_at: Vec3,
    screen_dist: f64,
    up: Vec3,

    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {
    pub fn new(eye: Vec3, look_at: Vec3, screen_dist: f64, up: Vec3) -> Self {
        let w = (eye - look_at).normalize();
        let u = (up.cross(w)).normalize();
        let v = w.cross(u);

        Self {
            eye,
            look_at,
            screen_dist,
            up: up.normalize(),
            u, v, w
        }
    }

    fn pixel_to_ray(&self, x: usize, y: usize, screen: &Screen) -> Ray {
        let factor_u = (x as f64 + 0.5) * (screen.real.0 / (screen.width as f64)) - 0.5 * screen.real.0;
        let factor_v = 0.5 * screen.real.1 - (y as f64 + 0.5) * (screen.real.1 / (screen.height as f64)); 

        Ray {
            origin: self.eye,
            direction: Vec3::linear_combine(
                factor_u, &self.u, 
                factor_v, &self.v, 
                -self.screen_dist, &self.w
            ),
        }
    }

    pub fn trace(&self, ray: Ray, scene: &Scene, bounce_count: usize) -> Option<HitInfo> {
        let mut min_distance = f64::INFINITY;
        let mut result: Option<HitInfo> = None;

        for surface in scene {
            if let Some(info) = surface.hit(&ray, scene, bounce_count, min_distance) {
                min_distance = (info.position - self.eye).mag();
                result = Some(info);
            }
        }

        result
    }

    pub fn trace_shadow(&self, ray: &Ray, scene: &Scene) -> bool {
        for surface in scene {
            if surface.shadow_hit(ray) {
                return true
            }
        }
        false
    }

    pub fn light(&self, info: &HitInfo, lights: Option<&SceneLights>, scene: &Scene) -> [u8; 3] {
        if let Some(lights) = lights {
            for light in lights {
                let ray = Ray { origin: info.position, direction: light.direction(&info) };
                if self.trace_shadow(&ray, &scene) {
                    return [0, 0, 0]
                }
            }
        }
        info.to_color_rgb()
    }

    pub fn render_scene(&self, scene: &Scene, lights: Option<&SceneLights>, mut screen: Screen) -> Screen {
        for (x,y) in screen.clone().into_iter() {
            let ray = self.pixel_to_ray(x, y, &screen);
            //println!("Ray: {:?}", ray);
            if let Some(result) = self.trace(ray, &scene, 10) {
                screen.pixels[x][y] = self.light(&result, lights, scene);
            }
        }
        screen
    }
    
}

#[derive(Debug, Clone)]
pub struct Screen {
    pub real: (f64, f64),
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Vec<[u8; 3]>>,

    current: (usize, usize),
}

impl Screen {
    pub fn new(width: usize, height: usize, realWidth: f64, realHeight: f64) -> Self {
        Self { real: (realWidth, realHeight), width, height, pixels: Self::init_screen(width, height), current: (0, 0) }
    }

    fn init_screen(width: usize, height: usize) -> Vec<Vec<[u8; 3]>> {
        let mut res = Vec::with_capacity(width);
        for x in 0..width {
            res.push(Vec::with_capacity(height));
            for y in 0..height {
                res[x].push([0, 0, 0])
            }
        }
        res
    }

    pub fn export(&self, path: &str) -> Result<(), ImageError> {
        let mut image = image::RgbImage::new(self.width as u32, self.height as u32);
        for (x, y) in self.clone().into_iter() {
            image.put_pixel(x as u32, y as u32, image::Rgb(self.pixels[x][y]))
        }
        image.save_with_format(path, ImageFormat::Png)
    }
}

impl Iterator for Screen {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.current.0 += 1;
        if self.current.0 >= self.width {
            self.current.0 = 0;
            self.current.1 += 1;
        }
        if self.current.1 >= self.height {
            return None;
        }

        Some((self.current.0, self.current.1))
    }
}