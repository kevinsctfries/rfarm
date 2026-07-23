use crate::crop::Crop;

pub struct Farm {
    pub crop: Crop,
}

impl Farm {
    pub fn new() -> Farm {
        Farm { crop: Crop::new() }
    }

    pub fn tick(&mut self) {
        self.crop.grow();
    }
}
