pub struct Crop {
    pub age: u32,
    pub height: u32,
}

impl Crop {
    pub fn new() -> Crop {
        Crop { age: 0, height: 1 }
    }

    pub fn grow(&mut self) {
        self.age += 1;

        if self.age % 5 == 0 {
            self.height += 1;
        }
    }
}
