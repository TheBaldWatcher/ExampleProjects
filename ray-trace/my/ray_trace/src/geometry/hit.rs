use crate::common::ray::Ray;
use crate::common::vec3::{Point3, Vec3};
use crate::geometry::Geometry;
use crate::material::Material;
use std::fmt::{Debug, Formatter};

pub struct HitRecord<'m> {
    pub point: Point3,
    pub normal: Vec3,
    pub material: &'m dyn Material,
    pub unit: f64,
    pub outside: bool,
}

impl Debug for HitRecord<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "HitRecord {{ unit:{} , hit: {:?}, normal: {:?}, outside: {} }}",
            self.unit, self.point, self.normal, self.outside
        ))
    }
}

impl<'m> HitRecord<'m> {
    pub fn new<G: Geometry>(r: &Ray, obj: &'m G, unit: f64) -> Self {
        let point = r.at(unit);
        let mut normal = obj.normal(&point);
        let outside = r.direction.dot(&normal) < 0.0;
        if !outside {
            normal.reverse();
        }
        Self {
            point,
            normal,
            material: obj.material(),
            unit,
            outside,
        }
    }
}
