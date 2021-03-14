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
    // depth
    picture_height: usize,
    samples: usize, // 每个pixel的采样
}

impl<'c> TakePhotoSettings<'c> {
    pub const fn new(camera: &'c Camera, world: World) -> Self {
        Self {
            camera,
            world,
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

    pub const fn height(mut self, height: usize) -> Self {
        self.picture_height = height;
        self
    }

    pub const fn samples(mut self, samples: usize) -> Self {
        self.samples = samples;
        self
    }

    // TODO not pub,
    fn ray_color(ray: &Ray, world: &World) -> Color {
        let center = Vec3::new(0.0, 0.0, -1.0);
        if let Some(hit) = world.hit(ray, 0.001..INFINITY) {
            let material = hit.material;
            let emmited = material.emitted(&hit.point).unwrap_or_default();
            // hongfendong
            return emmited.into_color(1);
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
            Self::ray_color(&ray, &self.world) //hongfendong
        })
    }
}
