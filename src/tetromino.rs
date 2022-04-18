use crate::{field::Field, WIDTH};
use rand::seq::SliceRandom;

pub struct Tetromino {
    pub pixels: Vec<(u32, u32)>,
    pub center: (u32, u32),
}

impl Tetromino {
    pub fn new() -> Result<Tetromino, &'static str> {
        let center = WIDTH / 2;
        match tetromino_pool(center).choose(&mut rand::thread_rng()) {
            None => Err("Tetromino pool is empty."),
            Some(value) => Ok(Tetromino { pixels: value.clone(), center: (center, 0) }),
        }
    }

    pub fn is_set(&self, requested_x: u32, requested_y: u32) -> bool {
        self.pixels
            .iter()
            .any(|val| val.0 == requested_x && val.1 == requested_y)
    }

    pub fn move_next(&mut self, field_height: u32) -> Result<bool, &'static str> {
        let new_max_y = match self.pixels.iter().map(|value| value.1 + 1).max() {
            None => Err("Could not determine new max y for tetromino"),
            Some(value) => Ok(value),
        }?;

        self.pixels = self.pixels.iter().map(|value| (value.0, value.1 + 1)).collect();
        self.center = (self.center.0, self.center.1 + 1);

        Ok(new_max_y == field_height - 1)
    }

    pub fn move_left(&mut self) -> Result<(), &'static str> {
        let new_min_x = match self.pixels.iter().map(|value| value.0 as i32 - 1).min() {
            None => Err("Could not determine new min x for tetromino"),
            Some(value) => Ok(value),
        }?;

        match new_min_x < 0 {
            true => Ok(()),
            false => {
                self.pixels = self.pixels.iter().map(|value| (value.0 - 1, value.1)).collect();
                self.center = (self.center.0 - 1, self.center.1);
                Ok(())
            }
        }
    }

    pub fn move_right(&mut self) -> Result<(), &'static str> {
        let new_max_x = match self.pixels.iter().map(|value| value.0 + 1).max() {
            None => Err("Could not determine new max x for tetromino"),
            Some(value) => Ok(value),
        }?;

        match new_max_x == WIDTH {
            true => Ok(()),
            false => {
                self.pixels = self.pixels.iter().map(|value| (value.0 + 1, value.1)).collect();
                self.center = (self.center.0 + 1, self.center.1);
                Ok(())
            }
        }
    }

    pub fn has_collision(&mut self, field: &Field) -> bool {
        for coord in self.pixels.iter() {
            if field.is_set(coord.0, coord.1 + 1) {
                return true;
            }
        }
        false
    }

    pub fn transform(&mut self) {
        self.pixels = self.pixels.iter().map(|value| {
            let center = (self.center.0 as i32, self.center.1 as i32);

            let x = value.0 as i32 - center.0;
            let y = value.1 as i32 - center.1;

            let transform_x = (center.0 - y) as u32;
            let transform_y = (center.1 + x) as u32;

            (transform_x, transform_y)
        }).collect();
    }
}

fn tetromino_pool(center: u32) -> [Vec<(u32, u32)>; 5] {
    [
        tetromino_straight(center),
        tetromino_square(center),
        tetromino_t(center),
        tetromino_l(center),
        tetromino_skew(center),
    ]
}

fn tetromino_straight(center: u32) -> Vec<(u32, u32)> {
    vec![
        (center - 2, 0),
        (center - 1, 0),
        (center, 0),
        (center + 1, 0),
    ]
}

fn tetromino_square(center: u32) -> Vec<(u32, u32)> {
    vec![(center, 0), (center, 1), (center - 1, 0), (center - 1, 1)]
}

fn tetromino_t(center: u32) -> Vec<(u32, u32)> {
    vec![(center - 1, 0), (center, 0), (center + 1, 0), (center, 1)]
}

fn tetromino_l(center: u32) -> Vec<(u32, u32)> {
    vec![
        (center - 1, 0),
        (center - 1, 1),
        (center - 1, 2),
        (center, 2),
    ]
}

fn tetromino_skew(center: u32) -> Vec<(u32, u32)> {
    vec![(center, 0), (center + 1, 0), (center - 1, 1), (center, 1)]
}
