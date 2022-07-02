use crate::{surface::Surface, materials::Material, math::{Vec3, self}, light::EPSILON};

pub struct Plane {
    pub material: Box<dyn Material>,
    pub point: Vec3,
    pub normal: Vec3,
}

impl Plane {
    pub fn new(material: Box<dyn Material>, point: Vec3, normal: Vec3) -> Self {
        Self { material, point, normal: normal.normalize() }
    }
}

impl Surface for Plane {
    fn get_material(&self) -> &Box<dyn crate::materials::Material> {
        &self.material
    }

    fn surface_hit(&self, ray: &crate::math::Ray, min_distance: f64) -> Option<crate::math::Vec3> {
        let dn = ray.direction * self.normal;

        if dn != 0.0 {
            let t = (self.point - ray.origin) * self.normal / dn;

            if t > EPSILON && t < min_distance {
                return Some(ray.origin + ray.direction * t)
            }
        }
        None
    }

    fn get_normal(&self, hit: &Vec3) -> Vec3 {
        self.normal
    }

    fn shadow_hit(&self, ray: &math::Ray, light_dist: f64) -> bool {
        let dn = ray.direction * self.normal;

        if dn != 0.0 {
            let t = (self.point - ray.origin) * self.normal / dn;

            if t > EPSILON && t < light_dist {
                return true
            }
        }
        false
    }
}

pub struct Sphere {
    pub material: Box<dyn Material>,
    origin: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(material: Box<dyn Material>, origin: Vec3, radius: f64) -> Self {
        Self { material, origin, radius }
    }
}

impl Surface for Sphere {
    fn get_material(&self) -> &Box<dyn Material> {
        &self.material
    }

    fn surface_hit(&self, ray: &crate::math::Ray, min_distance: f64) -> Option<Vec3> {
        let aux = ray.origin - self.origin;

        let d_sqr = ray.direction.mag2();

        let p_half = aux * ray.direction / d_sqr;
        let q = (aux.mag2() - self.radius * self.radius) / d_sqr;

        let discriminant = p_half * p_half - q;

        if discriminant >= 0.0 {
            let sqrt_discriminant = discriminant.sqrt();

            let mut t = -p_half - sqrt_discriminant;

            if !(t > EPSILON && t < min_distance) {
                t = -p_half + sqrt_discriminant;

                if !(t > EPSILON && t < min_distance) {
                    return None
                }
            }

            return Some(ray.origin + ray.direction * t)
        }

        None
    }

    fn get_normal(&self, hit: &Vec3) -> Vec3 {
        (*hit - self.origin).normalize()
    }

    fn shadow_hit(&self, ray: &math::Ray, light_dist: f64) -> bool {
        let aux = ray.origin - self.origin;

        let inv_d_sqr = 1.0 / (ray.direction.mag2());

        let p_half = aux * ray.direction * inv_d_sqr;
        let q = (aux.mag2() - self.radius * self.radius) * inv_d_sqr;

        let discriminant = p_half * p_half - q;

        if discriminant >= 0.0 {
            let sqrt_discriminant = discriminant.sqrt();

            let mut t = -p_half - sqrt_discriminant;

            if !(t > EPSILON && t < light_dist) {
                t = -p_half + sqrt_discriminant;

                if !(t > EPSILON && t < light_dist) {
                    return false
                }
            }

            return true
        }

        false
    }
}