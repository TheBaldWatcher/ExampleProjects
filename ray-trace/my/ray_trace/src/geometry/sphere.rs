use crate::common::ray::Ray;
use crate::common::vec3::{Point3, Vec3};
use crate::geometry::aabb::AABB;
use crate::geometry::hit::HitRecord;
use crate::geometry::Geometry;
use crate::material::Material;
use std::ops::Range;
use std::os::macos::raw::time_t;

pub struct Sphere<M: Material> {
    center: Point3,
    radius: f64,
    material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Point3, radius: f64, material: M) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl<M: Material> Geometry for Sphere<M> {
    fn normal(&self, p: &Point3) -> Vec3 {
        (p - &self.center).unit()
    }

    fn material(&self) -> &dyn Material {
        &self.material
    }

    fn hit(&self, ray: &Ray, unit_limit: Range<f64>) -> Option<HitRecord<'_>> {
        // Ray = A + t*B
        // t^2 * b * b + 2t*b*(A-C) + (A-C)*(A-C) - r^2 = 0
        let oc = &ray.origin - &self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrt = discriminant.sqrt();

        let root_1 = (-half_b - sqrt) / a;
        if unit_limit.contains(&root_1) {
            return Some(HitRecord::new(ray, self, root_1));
        }

        let root_2 = (-half_b + sqrt) / a;
        if unit_limit.contains(&root_2) {
            return Some(HitRecord::new(ray, self, root_2));
        }

        unreachable!()
    }

    fn bbox(&self, time_limit: Range<f64>) -> Option<AABB> {
        Some(
            // TODO if
            AABB::new(
                &self.center - Vec3::new(self.radius, self.radius, self.radius),
                &self.center - Vec3::new(self.radius, self.radius, self.radius),
            ),
        )
    }
}
