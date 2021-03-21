use crate::common::color::Color;
use crate::common::ray::Ray;
use crate::common::vec3::Vec3;
use crate::render::take_photo_settings::TakePhotoSettings;
use log::info;
use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::process::Output;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

#[derive(Debug)]
pub struct Painter {
    pub width: usize,
    pub height: usize,
    samples: usize,
    // gamma//
    // threads
    // parallel
}

struct PainterOutputContext<'c> {
    file: BufWriter<Box<dyn Write>>,
    cancel: &'c AtomicBool,
}

impl Painter {
    pub const fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            samples: 50,
        }
    }

    pub const fn samples(mut self, samples: usize) -> Self {
        self.samples = samples;
        self
    }
    ////////// output file //////////
    fn create_output_file(
        &self,
        path: Option<&Path>,
    ) -> std::io::Result<BufWriter<Box<dyn Write>>> {
        let mut file: BufWriter<Box<dyn Write>> = if let Some(path) = path {
            BufWriter::new(Box::new(File::create(&path)?))
        } else {
            BufWriter::new(Box::new(std::io::sink()))
        };

        write!(
            &mut file,
            "P3\n{width} {height}\n255\n",
            width = self.width,
            height = self.height
        )?;

        Ok(file)
    }

    fn create_output_context<'c>(
        &self,
        path: Option<&Path>,
        cancel: &'c AtomicBool,
    ) -> std::io::Result<PainterOutputContext<'c>> {
        let file = self.create_output_file(path)?;
        Ok(PainterOutputContext { file, cancel })
    }

    ///////// pixel //////////
    fn calculate_uv(&self, row: usize, col: usize) -> (f64, f64) {
        if self.samples == 1 {
            let u = (col as f64) / self.width as f64;
            let v = ((self.height - 1 - row) as f64) / self.height as f64;
            (u, v)
        } else {
            // 采样时，加扰动
            // > The “less than” before the 1 is important as we will sometimes take advantage of that.
            let u = (col as f64 + thread_rng().gen_range(0.0, 1.0)) / self.width as f64;
            let v = ((self.height - 1 - row) as f64 + thread_rng().gen_range(0.0, 1.0))
                / self.height as f64;
            (u, v)
        }
    }
    fn render_pixel<F>(&self, row: usize, col: usize, uv_color: &F) -> (u8, u8, u8)
    where
        F: Fn(f64, f64) -> Color + Send + Sync,
    {
        let color_need_average: Vec3 = (0..self.samples)
            .map(|_| {
                let (u, v) = self.calculate_uv(row, col);
                uv_color(u, v)
            })
            .map(|e| e.into())
            .sum();

        let color = color_need_average
            .into_color(self.samples)
            .int_form()
            .into_owned();
        (color.r, color.g, color.b)
    }

    fn seq_render_row<F>(&self, row: usize, uv_color: &F) -> Vec<(u8, u8, u8)>
    where
        F: Fn(f64, f64) -> Color + Send + Sync,
    {
        (0..self.width)
            .map(|col| self.render_pixel(row, col, &uv_color))
            .collect::<Vec<_>>()
    }

    fn seq_render_row_iter<'c, F>(
        &'c self,
        uv_color: F,
    ) -> impl Iterator<Item = Vec<(u8, u8, u8)>> + 'c
    where
        F: Fn(f64, f64) -> Color + Send + Sync + 'c,
    {
        (0..self.height).map(move |row| self.seq_render_row(row, &uv_color))
    }

    /////////// pixels to file ///////
    fn do_row_pixels_to_file(
        context: &mut PainterOutputContext<'_>,
        pixels: Vec<(u8, u8, u8)>,
    ) -> std::io::Result<()> {
        for pixel in pixels {
            writeln!(context.file, "{} {} {}", pixel.0, pixel.1, pixel.2)?;
        }
        context.file.flush()
    }

    fn row_pixels_to_file(
        &self,
        context: &mut PainterOutputContext<'_>,
        row: usize,
        pixels: Vec<(u8, u8, u8)>,
    ) -> std::io::Result<()> {
        info!("Scan line remaining: {}", self.height - row);
        Self::do_row_pixels_to_file(context, pixels).map_err(|e| {
            context.cancel.store(true, Ordering::Relaxed);
            e
        })
    }

    pub fn draw<P, F>(&self, path: &Option<P>, uv_color: F) -> std::io::Result<()>
    where
        P: AsRef<Path>,
        F: Fn(f64, f64) -> Color + Send + Sync,
    {
        let path = match path {
            Some(ref path) => Some(path.as_ref()),
            None => None,
        };

        if
        /*self.parallel*/
        false {
            unimplemented!()
        } else {
            let cancel = AtomicBool::new(false);
            let mut context = self.create_output_context(path, &cancel)?;

            for (row, pixels) in self.seq_render_row_iter(uv_color).enumerate() {
                self.row_pixels_to_file(&mut context, row, pixels)?;
            }

            Ok(())
        }
    }
}
