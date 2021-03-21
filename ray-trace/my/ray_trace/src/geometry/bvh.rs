use crate::common::ray::Ray;
use crate::common::vec3::Point3;
use crate::geometry::aabb::AABB;
use crate::geometry::hit::HitRecord;
use crate::geometry::Geometry;
use std::fmt::{Debug, Formatter};
use std::ops::{BitOr, Range};

#[derive(Default)]
pub struct BoundingVolumeHierachies {
    bbox: Option<AABB>,
    left: Option<Box<dyn Geometry>>,
    right: Option<Box<dyn Geometry>>,
}

impl Debug for BoundingVolumeHierachies {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("BVH {{ box: {:?} }}", self.bbox))
    }
}

impl BoundingVolumeHierachies {
    pub fn new(objects: Vec<Box<dyn Geometry>>, time_limit: Range<f64>) -> Self {
        if objects.is_empty() {
            Self::default()
        } else {
            let mut objects: Vec<_> = objects.into_iter().map(Some).collect();
            let count = objects.len();
            Self::new_internal(&mut objects, 0..count, time_limit)
        }
    }

    fn new_internal(
        objects: &mut Vec<Option<Box<dyn Geometry>>>,
        index: Range<usize>,
        time_limit: Range<f64>,
    ) -> Self {
        let count = index.end - index.start;

        if count == 1 {
            let left = objects[index.start].take().unwrap();
            let bbox = left
                .bbox(time_limit)
                .expect("No bounding box in bvh_node constructor.");
            Self {
                bbox: Some(bbox),
                left: Some(left),
                right: None,
            }
        } else if count == 2 {
            let left = objects[index.start].take().unwrap();
            let right = objects[index.start + 1].take().unwrap();
            let left_bbox = left
                .bbox(time_limit.clone())
                .expect("No bounding box in bvh_node constructor.");
            let right_bbox = right
                .bbox(time_limit.clone())
                .expect("No bounding box in bvh_node constructor.");

            Self {
                bbox: Some(left_bbox | right_bbox),
                left: Some(left),
                right: Some(right),
            }
        } else {
            unimplemented!()
        }
    }
}

impl Geometry for BoundingVolumeHierachies {
    fn hit(&self, ray: &Ray, unit_limit: Range<f64>) -> Option<HitRecord<'_>> {
        let bbox = self.bbox.as_ref()?;
        if !bbox.hit(ray, unit_limit.clone()) {
            return None;
        }

        let hit_left = self
            .left
            .as_ref()
            .and_then(|left| left.hit(ray, unit_limit.clone()));
        let hit_right = self.right.as_ref().and_then(|right| {
            let right_limit = unit_limit.start
                ..hit_left
                    .as_ref()
                    .map_or(unit_limit.end, |record| record.unit);
            right.hit(ray, right_limit)
        });

        // TODO 不明白
        // Right has small t then left if it return `Some`, so right appear first
        hit_right.or(hit_left)
    }

    fn bbox(&self, _time_limit: Range<f64>) -> Option<AABB> {
        self.bbox.clone()
    }
}
