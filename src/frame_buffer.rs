use std::convert::Infallible;

use embedded_graphics::{
    Pixel,
    pixelcolor::Rgb888,
    prelude::{DrawTarget, OriginDimensions, RgbColor, Size},
};

use crate::math_3d::Color;

pub struct FrameBuffer {
    size: usize,
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>,
    // pub pixels_ref: &'a Vec<u8>
}

#[allow(dead_code)]
impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = vec![0; width * height * 3];
        // let pixels_ref: &Vec<u8> = &pixels;
        Self {
            size: pixels.len(),
            width,
            height,
            pixels,
            // pixels_ref
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.pixels
    }

    pub fn clean(&mut self, c: Color) {
        for i in (0..self.size).step_by(3) {
            self.pixels[i] = c.0;
            self.pixels[i + 1] = c.1;
            self.pixels[i + 2] = c.2;
        }
    }

    pub fn pixel(&mut self, x: u32, y: u32, c: Color) {
        let idx = ((y * self.width as u32 + x) * 3) as usize;

        if idx + 2 < self.pixels.len() {
            self.pixels[idx] = c.0; // R
            self.pixels[idx + 1] = c.1; // G  
            self.pixels[idx + 2] = c.2; // B
        }
    }
}

impl OriginDimensions for FrameBuffer {
    fn size(&self) -> embedded_graphics::prelude::Size {
        Size::new(self.width as u32, self.height as u32)
    }
}

impl DrawTarget for FrameBuffer {
    type Color = Rgb888;
    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics::Pixel<Self::Color>>,
    {
        for Pixel(pos, color) in pixels {
            // Vérifie les bornes avec i32 (car pos.x/y sont des i32)
            if pos.x >= 0 && pos.y >= 0 && pos.x < self.width as i32 && pos.y < self.height as i32 {
                let x = pos.x as usize;
                let y = pos.y as usize;
                let w = self.width;

                let idx = (y * w + x) * 3;

                if idx + 2 < self.pixels.len() {
                    self.pixels[idx] = color.r();
                    self.pixels[idx + 1] = color.g();
                    self.pixels[idx + 2] = color.b();
                }
            }
        }

        Ok(())
    }
}
