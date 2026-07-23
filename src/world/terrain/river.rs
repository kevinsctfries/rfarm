use rand::RngExt;

use crate::world::feature::Feature;
use crate::world::geometry::path::Path;
use crate::world::geometry::point::Point;

pub struct River {
    pub path: Path,
    pub width: f32,
}

impl River {
    pub fn generate(width: u32, height: u32, rng: &mut impl RngExt) -> River {
        let mut points = Vec::new();

        let mut y = rng.random_range(0..height) as i32;

        let mut direction = 0;

        for x in 0..width {
            points.push(Point { x: x as i32, y });

            let change = rng.random_range(-1..=1);

            direction += change;

            direction = direction.clamp(-1, 1);

            y += direction;

            y = y.clamp(0, height as i32 - 1);
        }

        River {
            path: Path::new(points),
            width: 1.5,
        }
    }
}

impl Feature for River {
    fn contains(&self, point: Point) -> bool {
        self.path.distance_to(point) <= self.width
    }

    fn is_border(&self, point: Point) -> bool {
        let distance = self.path.distance_to(point);

        distance > self.width && distance <= self.width + 1.0
    }

    fn symbol_at(&self, point: Point) -> Option<char> {
        if self.contains(point) {
            Some('~')
        } else {
            None
        }
    }

    fn priority(&self) -> u32 {
        100
    }
}
