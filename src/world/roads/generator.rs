use crate::world::geometry::point::Point;
use crate::world::terrain::river::River;

use super::path::{Direction, Pathfinder};
use super::road::Road;

pub struct RoadGenerator;

impl RoadGenerator {
    pub fn generate(width: u32, height: u32, river: &River, rng: &mut impl rand::RngExt) -> Road {
        let mut road = Road::new(width, height);

        let width = width as i32;
        let height = height as i32;

        // Horizontal arterial
        let y = rng.random_range(3..height - 3);

        // Force border connections
        let west_exit = Point { x: 0, y };
        let west_inside = Point { x: 1, y };

        let east_inside = Point { x: width - 2, y };

        let east_exit = Point { x: width - 1, y };

        // Guaranteed straight entrances
        road.add_segment(west_exit);
        road.add_segment(west_inside);

        let path = Pathfinder::find_path(
            width as u32,
            height as u32,
            west_inside,
            east_inside,
            river,
            Direction::East,
        );

        for point in path {
            road.add_segment(point);
        }

        road.add_segment(east_inside);
        road.add_segment(east_exit);

        road.mark_border_exit(west_exit);
        road.mark_border_exit(east_exit);

        // Vertical arterial
        let x = rng.random_range(3..width - 3);

        let north_exit = Point { x, y: 0 };
        let north_inside = Point { x, y: 1 };

        let south_inside = Point { x, y: height - 2 };

        let south_exit = Point { x, y: height - 1 };

        road.add_segment(north_exit);
        road.add_segment(north_inside);

        let path = Pathfinder::find_path(
            width as u32,
            height as u32,
            north_inside,
            south_inside,
            river,
            Direction::South,
        );

        for point in path {
            road.add_segment(point);
        }

        road.add_segment(south_inside);
        road.add_segment(south_exit);

        road.mark_border_exit(north_exit);
        road.mark_border_exit(south_exit);

        road
    }
}
