use crate::{field::Field, WIDTH};
use rand::seq::SliceRandom;

pub struct Tetromino(pub Vec<(u32, u32)>);

impl Tetromino {
    pub fn new() -> Result<Tetromino, &'static str> {
        let center = WIDTH / 2;
        match tetromino_pool(center).choose(&mut rand::thread_rng()) {
            None => Err("Tetromino pool is empty."),
            Some(value) => Ok(Tetromino(value.clone())),
        }
    }

    pub fn is_set(&self, requested_x: u32, requested_y: u32) -> bool {
        self.0
            .iter()
            .any(|val| val.0 == requested_x && val.1 == requested_y)
    }

    pub fn move_next(&mut self, field_height: u32) -> Result<bool, &'static str> {
        let new_max_y = match self.0.iter().map(|value| value.1 + 1).max() {
            None => Err("Could not determine new max y for tetromino"),
            Some(value) => Ok(value),
        }?;

        self.0 = self.0.iter().map(|value| (value.0, value.1 + 1)).collect();

        Ok(new_max_y == field_height - 1)
    }

    pub fn has_collision(&mut self, field: &Field) -> bool {
        for coord in self.0.iter() {
            if field.is_set(coord.0, coord.1 + 1) {
                return true;
            }
        }
        false
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
