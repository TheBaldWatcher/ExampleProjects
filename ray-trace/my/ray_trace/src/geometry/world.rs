use crate::common::color::Color;
use crate::common::ray::Ray;
use crate::geometry::aabb::AABB;
use crate::geometry::bvh::BoundingVolumeHierachies;
use crate::geometry::hit::HitRecord;
use crate::geometry::Geometry;
use std::fmt::{Debug, Formatter};
use std::ops::Range;

pub struct World {
    bvh: BoundingVolumeHierachies,
    bg_func: Box<dyn Fn(&Ray) -> Color + Send + Sync>,
}

pub fn default_background(ray: &Ray) -> Color {
    let unit = ray.direction.unit();
    let t = 0.5 * (unit.y + 1.0);
    Color::newf(1.0, 1.0, 1.0).gradient(t, Color::newf(0.5, 0.7, 1.0))
}

impl Debug for World {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("World {}")
    }
}

impl World {
    pub fn new(bvh: BoundingVolumeHierachies) -> Self {
        Self {
            bvh,
            bg_func: Box::new(default_background),
        }
    }

    pub fn set_bg<F>(&mut self, f: F)
    where
        F: Fn(&Ray) -> Color + Send + Sync + 'static,
    {
        self.bg_func = Box::new(f);
    }

    pub fn background(&self, ray: &Ray) -> Color {
        let f = &self.bg_func;
        f(ray)
    }
}

impl Geometry for World {
    fn hit(&self, ray: &Ray, unit_limit: Range<f64>) -> Option<HitRecord<'_>> {
        self.bvh.hit(ray, unit_limit)
    }

    fn bbox(&self, time_limit: Range<f64>) -> Option<AABB> {
        self.bvh.bbox(time_limit)
    }
}
