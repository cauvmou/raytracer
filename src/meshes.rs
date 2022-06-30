use crate::{surface::Surface, materials::Material, math::Vec3};

pub struct Plane {
    pub material: Box<dyn Material>,
    pub point: Vec3,
    pub normal: Vec3,
}

impl Surface for Plane {
    fn get_material(&self) -> &Box<dyn crate::materials::Material> {
        &self.material
    }

    fn surface_hit(&self, ray: &crate::math::Ray) -> Option<crate::math::Vec3> {
        let dn = ray.origin * self.normal;

        if dn != 0.0 {
            let t = (self.point - ray.origin) * self.normal / dn;

            if t > 0.0 {
                return Some(ray.origin + ray.direction * t)
            }
        }
        None
    }
}