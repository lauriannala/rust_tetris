use std::collections::HashSet;

use crate::{pixel::Pixel, Tetromino, HEIGHT, WIDTH};
pub struct Field {
    pub pixels: Vec<Pixel>,
    pub filled_pixels: HashSet<(i32, i32)>,
}

impl Field {
    pub fn new() -> Field {
        let mut field = Field {
            pixels: vec![],
            filled_pixels: HashSet::new(),
        };
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                field.pixels.push(Pixel::new(x as i32, y as i32));
            }
        }

        field
    }

    pub fn is_set(&self, requested_x: i32, requested_y: i32) -> bool {
        match self.filled_pixels.get(&(requested_x, requested_y)) {
            None => false,
            Some(_value) => true,
        }
    }

    /// Fill tetromino. Returns true if row was completed in the process.
    pub fn fill_tetromino(&mut self, tetromino: &Tetromino) -> Vec<u32> {
        let mut rows_completed: Vec<u32> = vec![];
        for (x, y) in &tetromino.pixels {
            for pixel in &mut self.pixels {
                if pixel.coordinates.0 == *x && pixel.coordinates.1 == *y {
                    self.filled_pixels.insert((*x, *y));

                    let mut row_completed = true;
                    // Check the row.
                    for x in 0..WIDTH {
                        if !self.filled_pixels.contains(&(x as i32, *y)) {
                            row_completed = false;
                        }
                    }
                    if row_completed {
                        rows_completed.push(*y as u32);
                        println!("Row completed: {:?}", *y);
                    } else {
                        println!("Not yet...");
                    }
                }
            }
        }
        rows_completed
    }

    pub fn complete_row(&mut self, row: u32) {
        println!("Completing row: {:?}", &row);
    }
}
