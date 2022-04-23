use crate::{field::Field, WIDTH};
use rand::seq::SliceRandom;

pub enum TetrominoType {
    STRAIGHT,
    SQUARE,
    T,
    L,
    SKEW,
}

pub struct Tetromino {
    pub pixels: Vec<(i32, i32)>,
    pub center: (i32, i32),
    pub tetromino_type: &'static TetrominoType,
    pub straight_tetromino_index: Option<usize>,
}

impl Tetromino {
    pub fn new() -> Result<Tetromino, &'static str> {
        let start_point = (WIDTH / 2) as i32;
        match tetromino_pool(start_point).choose(&mut rand::thread_rng()) {
            None => Err("Tetromino pool is empty."),
            Some(value) => Ok(Tetromino {
                pixels: value.1.clone(),
                center: get_center(start_point, value.0),
                tetromino_type: value.0,
                straight_tetromino_index: if let TetrominoType::STRAIGHT = value.0 {
                    Some(0)
                } else {
                    None
                },
            }),
        }
    }

    pub fn is_set(&self, requested_x: i32, requested_y: i32) -> bool {
        self.pixels
            .iter()
            .any(|val| val.0 == requested_x && val.1 == requested_y)
    }
    pub fn move_next(&mut self, field_height: u32) -> Result<bool, &'static str> {
        let new_max_y = match self.pixels.iter().map(|value| value.1 + 1).max() {
            None => Err("Could not determine new max y for tetromino"),
            Some(value) => Ok(value),
        }?;

        self.pixels = self
            .pixels
            .iter()
            .map(|value| (value.0, value.1 + 1))
            .collect();
        self.center = (self.center.0, self.center.1 + 1);

        Ok(new_max_y == field_height as i32 - 1)
    }

    pub fn move_left(&mut self) -> Result<(), &'static str> {
        let new_min_x = match self.pixels.iter().map(|value| value.0 as i32 - 1).min() {
            None => Err("Could not determine new min x for tetromino"),
            Some(value) => Ok(value),
        }?;

        match new_min_x < 0 {
            true => Ok(()),
            false => {
                self.pixels = self
                    .pixels
                    .iter()
                    .map(|value| (value.0 - 1, value.1))
                    .collect();
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

        match new_max_x == WIDTH as i32 {
            true => Ok(()),
            false => {
                self.pixels = self
                    .pixels
                    .iter()
                    .map(|value| (value.0 + 1, value.1))
                    .collect();
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

    pub fn transform(&mut self) -> Result<(), &'static str> {
        match self.tetromino_type {
            &TetrominoType::SQUARE => Ok(()),
            &TetrominoType::STRAIGHT => match self.straight_tetromino_index {
                Some(index) => {
                    let incremented_index = if index != 3 { index + 1 } else { 0 };
                    self.pixels =
                        tetromino_straight_options(self.center)[incremented_index].clone();
                    self.straight_tetromino_index = Some(incremented_index);
                    Ok(())
                }
                None => Err("Straight tetromino index missing from tetromino."),
            },
            _ => {
                let center = (self.center.0 as i32, self.center.1 as i32);
                self.pixels = self
                    .pixels
                    .iter()
                    .map(|value| {
                        let x = value.0 as i32 - center.0;
                        let y = value.1 as i32 - center.1;

                        let transform_x = center.0 - y;
                        let transform_y = center.1 + x;

                        (transform_x, transform_y)
                    })
                    .collect();
                Ok(())
            }
        }
    }
}

fn get_center(start_point: i32, tetromino_type: &'static TetrominoType) -> (i32, i32) {
    match tetromino_type {
        &TetrominoType::STRAIGHT => (start_point - 1, 1),
        &TetrominoType::SQUARE => (start_point, 0),
        &TetrominoType::T => (start_point, 0),
        &TetrominoType::L => (start_point, 1),
        &TetrominoType::SKEW => (start_point, 1),
    }
}

fn tetromino_pool(starting_point: i32) -> [(&'static TetrominoType, Vec<(i32, i32)>); 5] {
    [
        (
            &TetrominoType::STRAIGHT,
            tetromino_straight_options((starting_point, 0))[0].clone(),
        ),
        (&TetrominoType::SQUARE, tetromino_square(starting_point)),
        (&TetrominoType::T, tetromino_t(starting_point)),
        (&TetrominoType::L, tetromino_l(starting_point)),
        (&TetrominoType::SKEW, tetromino_skew(starting_point)),
    ]
}

fn tetromino_straight_options(starting_point: (i32, i32)) -> [Vec<(i32, i32)>; 4] {
    [
        vec![
            (starting_point.0 - 2, starting_point.1),
            (starting_point.0 - 1, starting_point.1),
            (starting_point.0, starting_point.1),
            (starting_point.0 + 1, starting_point.1),
        ],
        vec![
            (starting_point.0, starting_point.1 - 1),
            (starting_point.0, starting_point.1),
            (starting_point.0, starting_point.1 + 1),
            (starting_point.0, starting_point.1 + 2),
        ],
        vec![
            (starting_point.0 - 2, starting_point.1 + 1),
            (starting_point.0 - 1, starting_point.1 + 1),
            (starting_point.0, starting_point.1 + 1),
            (starting_point.0 + 1, starting_point.1 + 1),
        ],
        vec![
            (starting_point.0 - 1, starting_point.1 - 1),
            (starting_point.0 - 1, starting_point.1),
            (starting_point.0 - 1, starting_point.1 + 1),
            (starting_point.0 - 1, starting_point.1 + 2),
        ],
    ]
}

fn tetromino_square(starting_point: i32) -> Vec<(i32, i32)> {
    vec![
        (starting_point, 0),
        (starting_point, 1),
        (starting_point - 1, 0),
        (starting_point - 1, 1),
    ]
}

fn tetromino_t(starting_point: i32) -> Vec<(i32, i32)> {
    vec![
        (starting_point - 1, 0),
        (starting_point, 0),
        (starting_point + 1, 0),
        (starting_point, 1),
    ]
}

fn tetromino_l(starting_point: i32) -> Vec<(i32, i32)> {
    vec![
        (starting_point - 1, 0),
        (starting_point - 1, 1),
        (starting_point, 1),
        (starting_point + 1, 1),
    ]
}

fn tetromino_skew(starting_point: i32) -> Vec<(i32, i32)> {
    vec![
        (starting_point, 0),
        (starting_point + 1, 0),
        (starting_point - 1, 1),
        (starting_point, 1),
    ]
}
