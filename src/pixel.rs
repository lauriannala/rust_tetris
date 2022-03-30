pub struct Pixel {
    pub coordinates: (u32, u32),
}

impl Pixel {
    pub fn new(x: u32, y: u32) -> Pixel {
        Pixel {
            coordinates: (x, y),
        }
    }
}
