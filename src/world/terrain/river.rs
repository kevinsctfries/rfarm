use rand::RngExt;

use crate::world::feature::Feature;
use crate::world::geometry::path::Path;
use crate::world::geometry::point::Point;

pub struct River {
    pub path: Path,
    pub width: f32,

    // The point where the river begins.
    pub source: Point,

    // The point where the river leaves the map.
    pub mouth: Point,
}

impl River {
    pub fn generate(width: u32, height: u32, rng: &mut impl RngExt) -> River {
        let mut points = Vec::new();

        // Rivers currently flow west -> east.
        // The first generated point is the source.
        let start_y = rng.random_range(0..height) as i32;

        let mut y = start_y;

        let mut direction = 0;

        for x in 0..width {
            points.push(Point { x: x as i32, y });

            let change = rng.random_range(-1..=1);

            direction += change;

            direction = direction.clamp(-1, 1);

            y += direction;

            y = y.clamp(0, height as i32 - 1);
        }

        let source = points.first().copied().unwrap();

        let mouth = points.last().copied().unwrap();

        River {
            path: Path::new(points),
            width: 1.5,
            source,
            mouth,
        }
    }

    pub fn contains_point(&self, point: Point) -> bool {
        self.path.distance_to(point) <= self.width
    }
}

impl Feature for River {
    fn contains(&self, point: Point) -> bool {
        self.contains_point(point)
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
