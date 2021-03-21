use crate::common::ray::Ray;
use crate::common::vec3::Vec3;
use crate::geometry::hit::HitRecord;
use crate::material::{Material, ScatterRecord};
use crate::texture::Texture;

#[derive(Debug, Clone)]
pub enum LambertianMathType {
    Approximate,
    // True,
    // Hemisphere,
}

impl LambertianMathType {
    pub fn scatter_ray(&self, ray: &Ray, hit: HitRecord<'_>) -> Ray {
        match self {
            Self::Approximate => Ray::new(hit.point, hit.normal + Vec3::random_in_unit_sphere()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Lambertian<T: Texture> {
    texture: T,
    math_type: LambertianMathType,
}

impl<T: Texture> Lambertian<T> {
    pub fn new(texture: T) -> Self {
        Self {
            texture,
            math_type: LambertianMathType::Approximate,
        }
    }

    pub fn math_type(mut self, value: LambertianMathType) -> Self {
        self.math_type = value;
        self
    }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, ray: &Ray, hit: HitRecord<'_>) -> Option<ScatterRecord> {
        let color = self.texture.color(hit.u, hit.v, &hit.point);
        let new_ray = self.math_type.scatter_ray(ray, hit);
        Some(ScatterRecord {
            color,
            ray: new_ray,
        })
    }
}
