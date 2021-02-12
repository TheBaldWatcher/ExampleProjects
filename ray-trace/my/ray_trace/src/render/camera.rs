use crate::common::vec3::{Point3, Vec3};
use crate::geometry::world::World;
use crate::render::take_photo_settins::TakePhotoSettings;

#[derive(Debug)]
pub struct Camera {
    origin: Point3,
    horizontal_full: Vec3,
    vertical_full: Vec3,
    horizontal_unit: Vec3,
    vertical_unit: Vec3,
    aspect_ratio: f64,
    // aperture: f64,
    // shutter_speed: f64,
}

impl Camera {
    pub fn new(
        // look_from: &Point3,
        // look_at: &Point3,
        // vup: &Vec3,
        // fov: f64,
        aspect_ratio: f64,
        // aperture: f64,
        // focus_distance: f64,
        // shutter_speed: f64,
    ) -> Self {
        Self {
            origin: Point3::new(0.0, 0.0, 0.0),
            horizontal_full: Vec3::new(0.0, 0.0, 0.0),
            vertical_full: Vec3::new(0.0, 0.0, 0.0),
            horizontal_unit: Vec3::new(0.0, 0.0, 0.0),
            vertical_unit: Vec3::new(0.0, 0.0, 0.0),
            aspect_ratio: 16.0 / 9.0,
        }
    }

    pub fn take_photo(
        &self,
        //world: GeometryList
    ) -> TakePhotoSettings<'_> {
        let world = World::new();
        TakePhotoSettings::new(self, world)
    }
}
