use rand::RngExt;

use crate::world::geometry::point::Point;

use super::road::Road;

pub struct RoadGenerator;

impl RoadGenerator {
    pub fn generate(width: u32, height: u32, rng: &mut impl RngExt) -> Road {
        let mut road = Road::new();

        // horizontal arterial
        let horizontal_y = rng.random_range(5..height - 5) as i32;

        for x in 0..width {
            road.add_segment(Point {
                x: x as i32,
                y: horizontal_y,
            });
        }

        // vertical arterial
        let vertical_x = rng.random_range(5..width - 5) as i32;

        for y in 0..height {
            road.add_segment(Point {
                x: vertical_x,
                y: y as i32,
            });
        }

        road
    }
}
