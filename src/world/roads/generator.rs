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
        let y = loop {
            let candidate = rng.random_range(3..height - 3) as i32;

            let west_exit = Point { x: 0, y: candidate };

            let east_exit = Point {
                x: width - 1,
                y: candidate,
            };

            if !too_close_to_river_endpoint(west_exit, river)
                && !too_close_to_river_endpoint(east_exit, river)
            {
                break candidate;
            }
        };

        let west_exit = Point { x: 0, y };
        let west_inside = Point { x: 1, y };

        let east_inside = Point { x: width - 2, y };

        let east_exit = Point { x: width - 1, y };

        // Guaranteed straight map entrance
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

        // Guaranteed straight map exit
        road.add_segment(east_inside);
        road.add_segment(east_exit);

        road.mark_border_exit(west_exit);
        road.mark_border_exit(east_exit);

        // Vertical arterial
        let x = loop {
            let candidate = rng.random_range(3..width - 3) as i32;

            let north_exit = Point { x: candidate, y: 0 };

            let south_exit = Point {
                x: candidate,
                y: height - 1,
            };

            if !too_close_to_river_endpoint(north_exit, river)
                && !too_close_to_river_endpoint(south_exit, river)
            {
                break candidate;
            }
        };

        let north_exit = Point { x, y: 0 };
        let north_inside = Point { x, y: 1 };

        let south_inside = Point { x, y: height - 2 };

        let south_exit = Point { x, y: height - 1 };

        // Guaranteed straight map entrance
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

        // Guaranteed straight map exit
        road.add_segment(south_inside);
        road.add_segment(south_exit);

        road.mark_border_exit(north_exit);
        road.mark_border_exit(south_exit);

        road
    }
}

fn too_close_to_river_endpoint(point: Point, river: &River) -> bool {
    const MIN_DISTANCE: f32 = 6.0;

    point.distance_to(river.source) < MIN_DISTANCE || point.distance_to(river.mouth) < MIN_DISTANCE
}
