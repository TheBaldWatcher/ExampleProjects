use crate::common::ray::Ray;
use crate::common::vec3::{Point3, Vec3};
use crate::geometry::list::GeometryList;
use crate::geometry::world::World;
use crate::render::take_photo_settins::TakePhotoSettings;

#[derive(Debug)]
pub struct Camera {
    origin: Point3,
    left_bottom: Point3,
    horizontal_full: Vec3,
    vertical_full: Vec3,
    horizontal_unit: Vec3,
    vertical_unit: Vec3,
    pub(crate) aspect_ratio: f64,
    // aperture: f64,
    shutter_speed: f64,
}

impl Camera {
    pub fn new(
        look_from: &Point3,
        look_at: &Point3,
        // vup: &Vec3,
        // fov: f64,
        aspect_ratio: f64,
        // aperture: f64,
        focus_distance: f64,
        // shutter_speed: f64,
    ) -> Self {
        // let fov = d2r(fov);
        // let h = (fov / 2.0).tan();
        // let vh = 2.0 * h;
        let vh = 2.0;
        let vw = vh * aspect_ratio;

        let w = (look_at - look_from).unit();
        // let horizontal_unit = w.cross(Vec3(0.0,1.0,0.0))
        let horizontal_unit = w.cross(&Vec3::new(0.0, 1.0, 0.0)).unit();
        let vertical_unit = horizontal_unit.cross(&w).unit();

        let horizontal_full = focus_distance * vw * &horizontal_unit;
        let vertical_full = focus_distance * vh * &vertical_unit;
        let left_bottom = look_from - &horizontal_full / 2.0 - &vertical_full / 2.0 +
            //focus_distance *   hongfendong
            w;
        Self {
            origin: look_from.clone(),
            left_bottom: left_bottom,
            horizontal_full,
            vertical_full,
            horizontal_unit,
            vertical_unit,
            aspect_ratio,
            shutter_speed: 0.0,
        }
    }

    pub fn ray(&self, u: f64, v: f64) -> Ray {
        let rd = Vec3::default(); // hongfendong
        let offset = &self.horizontal_unit * rd.x + &self.vertical_unit * rd.y;
        let origin = &self.origin + offset;
        let direction =
            &self.left_bottom + u * &self.horizontal_full + v * &self.vertical_full - &origin;
        // dbg!(
        //     &self.left_bottom,
        //     u * &self.horizontal_full,
        //     v * &self.vertical_full,
        //     &origin
        // );
        Ray::new(origin, direction)
    }

    pub fn take_photo(&self, world: GeometryList) -> TakePhotoSettings<'_> {
        let world = world.build(0.0..self.shutter_speed);
        TakePhotoSettings::new(self, world)
    }
}

pub struct CameraBuilder {
    look_from: Point3,
    look_at: Point3,
    // vup:Vec3,
    // fov :f64,
    aspect_ratio: f64,
    // aperture:f64,
    focus_distance: f64,
    // shutter_spped:f64,
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self {
            look_from: Point3::default(),
            look_at: Point3::new(0.0, 0.0, -1.0),
            //     vup
            // fov
            aspect_ratio: 16.0 / 9.0,
            //     aperture
            focus_distance: 1.0,
            // shutter_speed
        }
    }
}

impl CameraBuilder {
    pub const fn look_from(mut self, look_from: Point3) -> Self {
        self.look_from = look_from;
        self
    }

    pub const fn look_at(mut self, look_at: Point3) -> Self {
        self.look_at = look_at;
        self
    }

    pub const fn aspect_ratio(mut self, aspect_ratio: f64) -> Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    pub const fn focus_distance(mut self, focus_distance: f64) -> Self {
        self.focus_distance = focus_distance;
        self
    }

    pub fn build(self) -> Camera {
        Camera::new(
            &self.look_from,
            &self.look_at,
            // &self.vup,
            // self.fov,
            self.aspect_ratio,
            // self.aperture,
            self.focus_distance,
            // self.shutter_speed,
        )
    }
}
