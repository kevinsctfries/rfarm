#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn distance_to(&self, other: Point) -> f32 {
        let dx = (self.x - other.x) as f32;
        let dy = (self.y - other.y) as f32;

        (dx * dx + dy * dy).sqrt()
    }

    // The four orthogonally adjacent points (east, west, south, north).
    pub fn orthogonal_neighbors(&self) -> [Point; 4] {
        [
            Point {
                x: self.x + 1,
                y: self.y,
            },
            Point {
                x: self.x - 1,
                y: self.y,
            },
            Point {
                x: self.x,
                y: self.y + 1,
            },
            Point {
                x: self.x,
                y: self.y - 1,
            },
        ]
    }

    // Whether this point falls within a 0..width by 0..height grid.
    pub fn in_bounds(&self, width: u32, height: u32) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < width as i32 && self.y < height as i32
    }
}
