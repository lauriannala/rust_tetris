use crate::{pixel::Pixel, HEIGHT, WIDTH};
pub struct Field(pub Vec<Pixel>);

impl Field {
    pub fn new() -> Field {
        let mut field = Field(vec![]);
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                field.0.push(Pixel::new(x, y));
            }
        }

        field
    }
}
