use crate::world::geometry::point::Point;
use crate::world::terrain::river::River;

use super::path::{Direction, Pathfinder};
use super::road::Road;

pub struct RoadGenerator;

impl RoadGenerator {
    pub fn generate(width: u32, height: u32, river: &River, rng: &mut impl rand::RngExt) -> Road {
        let mut road = Road::new(width, height);

        // Horizontal arterial
        let y = rng.random_range(3..height - 3) as i32;

        let path = Pathfinder::find_path(
            width,
            height,
            Point { x: 0, y },
            Point {
                x: width as i32 - 1,
                y,
            },
            river,
            Direction::East,
        );

        for point in path {
            road.add_segment(point);
        }

        // Vertical arterial
        let x = rng.random_range(3..width - 3) as i32;

        let path = Pathfinder::find_path(
            width,
            height,
            Point { x, y: 0 },
            Point {
                x,
                y: height as i32 - 1,
            },
            river,
            Direction::South,
        );

        for point in path {
            road.add_segment(point);
        }

        road
    }
}
