mod common;
mod geometry;
mod render;

use crate::render::ppm_image::PPMImage;

// use crate::render;

fn main() {
    const IMAGE_WIDTH: usize = 256 * 16 / 9;
    const IMAGE_HEIGHT: usize = 256;

    let ppm_image = PPMImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    ppm_image.save();
}
