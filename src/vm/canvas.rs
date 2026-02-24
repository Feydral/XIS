use std::io::Write;
use crate::math::numerics::float3::Float3;

pub struct Canvas {
    width: u32,
    height: u32,
    pixels: Vec<Float3>,
}

impl Canvas {
    pub fn init(width: u32, height: u32) -> Self {
        Canvas {
            width,
            height,
            pixels: vec![Float3::ZERO; width as usize * height as usize],
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Float3) {
        if x < self.width && y < self.height {
            let index = y as usize * self.width as usize + x as usize;
            self.pixels[index] = color;
        }
    }

    pub fn set_pixels(&mut self, x: u32, y: u32, colors: &[&[Float3]]) {
        let width = colors[0].len() as u32;
        let height = colors.len() as u32;

        for j in 0..height {
            for i in 0..width {
                self.set_pixel(x + i, y + j, colors[j as usize][i as usize]);
            }
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Float3 {
        if x < self.width && y < self.height {
            let index = y as usize * self.width as usize + x as usize;
            self.pixels[index]
        } else {
            Float3::ZERO
        }
    }

    pub fn clear(&mut self) {
        self.pixels.fill(Float3::ZERO);
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}