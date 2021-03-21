use crate::common::color::Color;
use crate::common::vec3::Point3;
use std::sync::Arc;

pub trait Texture: Send + Sync {
    fn color(&self, u: f64, v: f64, point: &Point3) -> Color;
}

impl<T: Texture> Texture for Arc<T> {
    fn color(&self, u: f64, v: f64, point: &Point3) -> Color {
        self.as_ref().color(u, v, point)
    }
}
