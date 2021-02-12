use crate::common::color::Color;
use crate::common::vec3::{Point3, Vec3};
use crate::geometry::world::World;
use crate::{common::ray::Ray, render::camera::Camera};

#[derive(Debug)]
pub struct TakePhotoSettings<'c> {
    camera: &'c Camera,
    world: World,
}

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> Option<f64> {
    // Ray = A + t*B
    // t^2 * b * b + 2t*b*(A-C) + (A-C)*(A-C) - r^2 = 0
    let oc = &ray.origin - center; // A-C
    let a = ray.direction.dot(&ray.direction);
    let b = 2.0 * ray.direction.dot(&oc);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        None
    } else {
        Some((-b - discriminant.sqrt()) / (2.0 * a))
    }
}

impl<'c> TakePhotoSettings<'c> {
    pub const fn new(camera: &'c Camera, world: World) -> Self {
        Self { camera, world }
    }

    ////// seter //////
    // return Self, so we can chain
    pub fn background<BG: Fn(&Ray) -> Color + Send + Sync + 'static>(mut self, bg: BG) -> Self {
        self.world.set_bg(bg);
        self
    }

    // TODO not pub,
    pub fn ray_color(ray: &Ray) -> Color {
        let center = Vec3::new(0.0, 0.0, -1.0);
        if let Some(t) = hit_sphere(&center, 0.5, &ray) {
            if t > 0.0 {
                let n = (ray.at(t) - &center).unit();
                return (0.5 * (n + Vec3::new(1.0, 1.0, 1.0))).into_color();
            }
        }

        let unit_direction = ray.direction.unit();
        // y's range is [-1, 1], t is [0, 1]
        let t = (unit_direction.y + 1.0) * 0.5;
        let a = Color::newf(1.0, 1.0, 1.0);
        let b = Color::newf(0.5, 0.7, 1.0);
        a.gradient(t, b)
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
