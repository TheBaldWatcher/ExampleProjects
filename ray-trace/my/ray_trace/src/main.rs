mod common;
mod geometry;
mod material;
mod render;

use crate::common::color::Color;
use crate::common::ray::Ray;
use crate::common::vec3::{Point3, Vec3};
use crate::geometry::list::GeometryList;
use crate::geometry::world::default_background;
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
        // } else if t2 > 0.0 {
        //     let p = ray.at(t2);
        //     let n = (p - center) / radius;
        //     Some((t2, n))
        } else {
            None
        }
    }
}

fn background(ray: &Ray) -> Color {
    let ts = vec![
        hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, ray),
        hit_sphere(&Point3::new(0.0, -100.5, -1.0), 100.0, ray),
    ];
    if let Some((_, n)) = ts
        .into_iter()
        .flatten()
        .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
    {
        return (0.5 * (n + Vec3::new(1.0, 1.0, 1.0))).into_color(1);
    }

    let unit = ray.direction.unit();
    let t = 0.5 * (unit.y + 1.0);
    Color::newf(1.0, 1.0, 1.0).gradient(t, Color::newf(0.5, 0.7, 1.0))
}

fn main() {
    const ASPECT_RATIO: f64 = 2.00;
    const IMAGE_HEIGHT: usize = 512;
    // const IMAGE_WIDTH: usize = (IMAGE_HEIGHT as f64 * ASPECT_RATIO) as usize;

    const FILE_PATH: &str =
        "/Users/jeashtower/Desktop/myFiles/ExampleProjects/ray-trace/my/ray_trace/output/render001.ppm";

    env_logger::init();

    let world = GeometryList::default();

    let camera = CameraBuilder::default().aspect_ratio(ASPECT_RATIO).build();

    camera
        .take_photo(world)
        .background(background)
        .height(IMAGE_HEIGHT)
        .samples(10)
        // .gamma.samples
        .shot(Some(FILE_PATH))
        .unwrap();
}
