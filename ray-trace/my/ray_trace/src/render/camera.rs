use crate::common::vec3::{Point3, Vec3};

#[derive(Debug)]
pub struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        // TODO
        Self {
            origin: Point3::new(0.0, 0.0, 0.0),
            horizontal: Vec3::new(0.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}
