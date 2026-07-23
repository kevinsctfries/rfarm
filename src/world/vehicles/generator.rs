use rand::RngExt;

use crate::world::geometry::point::Point;
use crate::world::roads::road::Road;

use super::vehicle::{Direction, Vehicle};

pub struct VehicleGenerator;

impl VehicleGenerator {
    pub fn generate(road: &Road, count: usize, rng: &mut impl RngExt) -> Vec<Vehicle> {
        let road_tiles = road.points();

        let mut vehicles = Vec::new();

        if road_tiles.is_empty() {
            return vehicles;
        }

        for _ in 0..count {
            let position = road_tiles[rng.random_range(0..road_tiles.len())];

            let direction = Self::choose_initial_direction(road, position, rng);

            vehicles.push(Vehicle::new(position, direction));
        }

        vehicles
    }

    pub fn spawn_at_exit(point: Point, direction: Direction) -> Vehicle {
        Vehicle::new(point, direction)
    }

    pub fn choose_initial_direction(road: &Road, point: Point, rng: &mut impl RngExt) -> Direction {
        let mut directions = Vec::new();

        for neighbor in point.orthogonal_neighbors() {
            if !road.contains(neighbor) {
                continue;
            }

            if neighbor.x > point.x {
                directions.push(Direction::East);
            }

            if neighbor.x < point.x {
                directions.push(Direction::West);
            }

            if neighbor.y > point.y {
                directions.push(Direction::South);
            }

            if neighbor.y < point.y {
                directions.push(Direction::North);
            }
        }

        if directions.is_empty() {
            Direction::East
        } else {
            directions[rng.random_range(0..directions.len())]
        }
    }
}
