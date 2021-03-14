use crate::common::ray::Ray;
use crate::common::vec3::{Point3, Vec3};
use crate::geometry::aabb::AABB;
use crate::geometry::hit::HitRecord;
use crate::material::Material;
use std::ops::Range;

mod aabb;
mod bvh;
pub(crate) mod hit;
pub(crate) mod list;
mod sphere;
pub(crate) mod world;

// TODO Send+Sync
pub trait Geometry: Send + Sync {
    // 法向量
    fn normal(&self, p: &Point3) -> Vec3 {
        unimplemented!(
            "{}'s normal function should not be called directly",
            std::any::type_name::<Self>()
        )
    }

    // getter
    fn material(&self) -> &dyn Material {
        unimplemented!(
            "{}'s material function should not be called directly",
            std::any::type_name::<Self>()
        )
    }

    fn hit(&self, ray: &Ray, unit_limit: Range<f64>) -> Option<HitRecord<'_>>;
    fn bbox(&self, time_limit: Range<f64>) -> Option<AABB>;
}
