use crate::common::color::Color;
use crate::common::vec3::{Point3, Vec3};
use crate::geometry::world::World;
use crate::geometry::Geometry;
use crate::render::painter::Painter;
use crate::{common::ray::Ray, render::camera::Camera};
use std::f64::INFINITY;
use std::path::Path;

#[derive(Debug)]
pub struct TakePhotoSettings<'c> {
    camera: &'c Camera,
    world: World,
    max_reflection: usize, // depth
    picture_height: usize,
    samples: usize, // 每个pixel的采样
}

impl<'c> TakePhotoSettings<'c> {
    pub const fn new(camera: &'c Camera, world: World) -> Self {
        Self {
            camera,
            world,
            max_reflection: 8,
            picture_height: 108,
            samples: 50,
        }
    }

    ////// seter //////
    // return Self, so we can chain
    pub fn background<BG: Fn(&Ray) -> Color + Send + Sync + 'static>(mut self, bg: BG) -> Self {
        self.world.set_bg(bg);
        self
    }

    pub const fn max_reflection(mut self, max_reflection: usize) -> Self {
        self.max_reflection = max_reflection;
        self
    }

    pub const fn height(mut self, height: usize) -> Self {
        self.picture_height = height;
        self
    }

    pub const fn samples(mut self, samples: usize) -> Self {
        self.samples = samples;
        self
    }

    // TODO not pub,
    fn ray_color(ray: &Ray, world: &World, remain_reflection: usize) -> Color {
        if remain_reflection == 0 {
            return Color::default();
        }
        if let Some(hit) = world.hit(ray, 0.001..INFINITY) {
            let material = hit.material;
            let emitted = material
                .emitted(hit.u, hit.v, &hit.point)
                .unwrap_or_default()
                .into_color(1);

            // scatter成新的光线
            if let Some(scattered) = material.scatter(ray, hit) {
                // return emitted.gradient(
                //     // hongfendong scattered.color *
                //     0.5,
                //     Self::ray_color(&scattered.ray, world, remain_reflection - 1),
                // );
                return 0.5 * Self::ray_color(&scattered.ray, world, remain_reflection - 1);
            }
            return emitted;
        }

        world.background(ray).into()
    }

    pub fn shot<P: AsRef<Path>>(&self, path: Option<P>) -> std::io::Result<()> {
        // TODO what is this?
        // #[allow(
        //     clippy::cast_sign_loss,
        //     clippy::cast_precision_loss,
        //     clippy::cast_possible_truncation
        // )]

        Painter::new(
            (self.picture_height as f64 * self.camera.aspect_ratio).round() as usize,
            self.picture_height,
        )
        // gama.sapmles/thread.parallel
        .samples(self.samples)
        .draw(&path, |u, v| -> Color {
            let ray = self.camera.ray(u, v);
            Self::ray_color(&ray, &self.world, self.max_reflection) //hongfendong
        })
    }
}
