use crate::common::color::Color;
use crate::common::ray::Ray;
use crate::common::vec3::{Point3, Vec3};
use crate::geometry::hit::HitRecord;
use std::sync::Arc;

#[derive(Debug)]
pub struct ScatterRecord {
    pub color: Color,
    pub ray: Ray,
}

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, hit: HitRecord<'_>) -> Option<ScatterRecord> {
        None
    }

    fn emitted(&self, point: &Point3) -> Option<Vec3> {
        None
    }
}

impl<M: Material> Material for Arc<M> {
    fn emitted(&self, point: &Point3) -> Option<Vec3> {
        Some(point.clone())
    }
}
