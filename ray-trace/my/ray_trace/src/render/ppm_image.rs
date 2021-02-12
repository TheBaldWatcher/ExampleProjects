use crate::render::color::RgbFloat;
use {
    super::color::Color,
    std::{
        fs::File,
        io::{BufWriter, Write},
    },
};

#[derive(Debug)]
pub struct PPMImage {
    width: usize,
    height: usize,
    colors: Vec<Color>,
}

impl PPMImage {
    pub fn new(width: usize, height: usize) -> Self {
        let mut colors = vec![Color::default(); width * height];
        for i in 0..height {
            for j in 0..width {
                colors[i * width + j] = Color::RgbF(RgbFloat::new(
                    i as f64 / ((width - 1) as f64),
                    j as f64 / ((height - 1) as f64),
                    0.25,
                ))
            }
        }
        Self {
            width,
            height,
            colors,
        }
    }

    // TODO 输出路径
    pub fn save(&self) -> std::io::Result<()> {
        let path =
            "/Users/jeashtower/Desktop/myFiles/ExampleProjects/ray-trace/my/ray_trace/output";
        let f = path.to_owned() + "/001.ppm";
        let mut file = File::create(f)?;
        write!(
            &mut file,
            "P3\n{width} {height}\n255\n",
            width = self.width,
            height = self.height
        )?;

        for row in 0..self.height {
            for col in 0..self.width {
                let index = row * self.width + col;
                let color = &self.colors[index].int_form();
                writeln!(
                    &mut file,
                    "{r} {g} {b}",
                    r = color.r,
                    g = color.g,
                    b = color.b
                )?;
            }
        }

        Ok(())
    }
}
