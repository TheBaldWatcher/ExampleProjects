use crate::common::vec3::Vec3;
use crate::render::color::Color;
use crate::{common::ray::Ray, render::camera::Camera};

#[derive(Debug)]
pub struct TakePhotoSettings<'c> {
    camera: &'c Camera,
}

impl<'c> TakePhotoSettings<'c> {
    pub const fn new(camera: &'c Camera) -> Self {
        Self { camera }
    }

    // TODO not pub,
    pub fn ray_color(ray: &Ray) -> Color {
        let unit_direction = ray.direction.unit();
        // y's range is [-1, 1], t is [0, 1]
        let t = (unit_direction.y + 1.0) * 0.5;
        let a = Color::newf(1.0, 1.0, 1.0);
        let b = Color::newf(0.5, 0.7, 1.0);
        a.mix(t, b)
    }
}

////////// UT //////////
#[test]
fn test_ray_color() {
    {
        let ray_left = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, -1.0, 0.0));
        assert_eq!(TakePhotoSettings::ray_color(&ray_left).int_form().r, 127);
        assert_eq!(TakePhotoSettings::ray_color(&ray_left).int_form().g, 127);
        assert_eq!(TakePhotoSettings::ray_color(&ray_left).int_form().b, 127);
    }
}
