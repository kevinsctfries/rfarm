use super::crop::Crop;

pub struct Plot {
    pub crop: Option<Crop>,
}

impl Plot {
    pub fn empty() -> Plot {
        Plot { crop: None }
    }

    pub fn plant(&mut self, crop: Crop) {
        self.crop = Some(crop);
    }

    pub fn tick(&mut self) {
        if let Some(crop) = &mut self.crop {
            crop.grow();
        }
    }
}
