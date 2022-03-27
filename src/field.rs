use crate::{HEIGHT, WIDTH};
pub struct Field(pub Vec<(u32, u32)>);

impl Field {
    pub fn new() -> Field {
        let mut field = Field(vec![]);
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                field.0.push((x, y));
            }
        }

        field
    }
}
