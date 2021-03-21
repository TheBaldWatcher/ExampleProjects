mod common;
mod geometry;
mod material;
mod render;
mod texture;

use crate::common::color::Color;
use crate::common::ray::Ray;
use crate::common::vec3::{Point3, Vec3};
use crate::geometry::list::GeometryList;
use crate::geometry::sphere::Sphere;
use crate::geometry::world::default_background;
use crate::material::lambertian::{Lambertian, LambertianMathType};
use crate::render::camera::CameraBuilder;

// use crate::render;
fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> Option<(f64, Vec3)> {
    // ||(P - C)|| == R
    let oc = &ray.origin - center;
    let a = ray.direction.length_squared();
    let half_b = oc.dot(&ray.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    // dbg!(center, radius, ray, a, half_b, c);
    if discriminant < 0.0 {
        None
    } else {
        let t1 = (-half_b - discriminant.sqrt()) / a;
        let t2 = (-half_b + discriminant.sqrt()) / a;
        if t1 > 0.0 {
            let p = ray.at(t1);
            let n = (p - center) / radius;
            Some((t1, n))
        } else if t2 > 0.0 {
            let p = ray.at(t2);
            let n = (p - center) / radius;
            Some((t2, n))
        } else {
            None
        }
    }
}

fn main() {
    const ASPECT_RATIO: f64 = 2.00;
    const IMAGE_HEIGHT: usize = 100;
    // const IMAGE_WIDTH: usize = (IMAGE_HEIGHT as f64 * ASPECT_RATIO) as usize;

    const FILE_PATH: &str =
        "/Users/jeashtower/Desktop/myFiles/ExampleProjects/ray-trace/my/ray_trace/output/render001.ppm";

    env_logger::init();

    let mut world = GeometryList::default();
    world
        .add(Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            Lambertian::new(Color::newf(0.5, 0.5, 0.5)).math_type(LambertianMathType::Approximate),
        ))
        .add(Sphere::new(
            Point3::new(0.0, 0.0, -1.0),
            0.5,
            Lambertian::new(Color::newf(0.5, 0.5, 0.5)).math_type(LambertianMathType::Approximate),
        ));

    let camera = CameraBuilder::default().aspect_ratio(ASPECT_RATIO).build();

    camera
        .take_photo(world)
        .max_reflection(8)
        .height(IMAGE_HEIGHT)
        .samples(100)
        .gamma(true)
        // .gamma.samples
        .shot(Some(FILE_PATH))
        .unwrap();
}
