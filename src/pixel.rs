pub struct Pixel {
    pub coordinates: (u32, u32),
    pub is_filled: bool,
}

impl Pixel {
    pub fn new(x: u32, y: u32) -> Pixel {
        Pixel {
            coordinates: (x, y),
            is_filled: false,
        }
    }
}
