use crate::common::color::{Color, RgbFloat};
use crate::common::ray::Ray;
use crate::common::vec3::Vec3;
use crate::geometry::world::World;
use crate::render::camera::Camera;
use crate::render::take_photo_settins::TakePhotoSettings;
use std::{
    fs::File,
    io::{BufWriter, Write},
};

#[derive(Debug)]
pub struct PPMImage {
    width: usize,
    height: usize,
    colors: Vec<Color>,
}

impl PPMImage {
    // pub fn new(width: usize, height: usize) -> Self {
    //     let mut colors = vec![Color::default(); width * height];
    //
    //     let focal_length = 1.0;
    //     let origin = Vec3::new(0.0, 0.0, 0.0);
    //     let horizontal = Vec3::new(2.0 * 16.0 / 9.0, 0.0, 0.0);
    //     let vertical = Vec3::new(0.0, 2.0, 0.0);
    //     let lower_left_corner =
    //         &origin - &horizontal / 2.0 - &vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);
    //     // let camera = Camera::new();
    //     // let take_photo_settings = TakePhotoSettings::new(&camera);
    //     let world = World::new();
    //     for i in (0..height).rev() {
    //         for j in 0..width {
    //             let u = j as f64 / ((width - 1) as f64);
    //             let v = i as f64 / ((height - 1) as f64);
    //             let ray = Ray::new(
    //                 origin.clone(),
    //                 &lower_left_corner + u * &horizontal + v * &vertical - &origin,
    //             );
    //             colors[i * width + j] = TakePhotoSettings::ray_color(&ray, &world);
    //         }
    //     }
    //
    //     Self {
    //         width,
    //         height,
    //         colors,
    //     }
    // }
    //
    // // TODO 输出路径
    // pub fn save(&self) -> std::io::Result<()> {
    //     let path =
    //         "/Users/jeashtower/Desktop/myFiles/ExampleProjects/ray-trace/my/ray_trace/output";
    //     let f = path.to_owned() + "/001.ppm";
    //     let mut file = File::create(f)?;
    //     write!(
    //         &mut file,
    //         "P3\n{width} {height}\n255\n",
    //         width = self.width,
    //         height = self.height
    //     )?;
    //
    //     for row in 0..self.height {
    //         for col in 0..self.width {
    //             let index = row * self.width + col;
    //             let color = &self.colors[index].int_form();
    //             writeln!(
    //                 &mut file,
    //                 "{r} {g} {b}",
    //                 r = color.r,
    //                 g = color.g,
    //                 b = color.b
    //             )?;
    //         }
    //     }
    //
    //     Ok(())
    // }
}
