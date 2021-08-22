pub struct Tetromino(Vec<(u32, u32)>);

impl Tetromino {
    pub fn new() -> Tetromino {
        Tetromino(vec![(0, 0), (1, 0), (2, 0), (3, 0)])
    }

    pub fn is_set(&self, requested_x: u32, requested_y: u32) -> bool {
        self.0.iter().any(|val| val.0 == requested_x && val.1 == requested_y)
    }

    pub fn move_next(&mut self) {
        self.0 = self.0.iter().map(|value| (value.0, value.1 + 1)).collect()
    }
}
