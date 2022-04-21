pub struct Pixel {
    pub coordinates: (i32, i32),
}

impl Pixel {
    pub fn new(x: i32, y: i32) -> Pixel {
        Pixel {
            coordinates: (x, y),
        }
    }
}
