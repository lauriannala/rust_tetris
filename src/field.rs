use std::collections::HashSet;

use crate::{pixel::Pixel, Tetromino, HEIGHT, WIDTH};
pub struct Field {
    pub pixels: Vec<Pixel>,
    pub filled_pixels: HashSet<(u32, u32)>,
}

impl Field {
    pub fn new() -> Field {
        let mut field = Field {
            pixels: vec![],
            filled_pixels: HashSet::new(),
        };
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                field.pixels.push(Pixel::new(x, y));
            }
        }

        field
    }

    pub fn is_set(&self, requested_x: u32, requested_y: u32) -> bool {
        match self.filled_pixels.get(&(requested_x, requested_y)) {
            None => false,
            Some(_value) => true,
        }
    }

    pub fn fill_tetromino(&mut self, tetromino: &Tetromino) {
        for (x, y) in &tetromino.0 {
            for pixel in &mut self.pixels {
                if pixel.coordinates.0 == *x && pixel.coordinates.1 == *y {
                    self.filled_pixels.insert((*x, *y));
                }
            }
        }
    }
}
