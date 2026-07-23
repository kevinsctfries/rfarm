use rand::RngExt;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use super::feature::Feature;
use super::geometry::point::Point;
use super::land_parcel::LandParcel;
use super::roads::road::Road;
use super::vehicles::generator::VehicleGenerator;
use super::vehicles::vehicle::{Direction, Vehicle};

pub struct Map {
    pub width: u32,
    pub height: u32,

    pub features: Vec<Box<dyn Feature>>,

    pub parcels: Vec<LandParcel>,

    pub vehicles: Vec<Vehicle>,

    rng: ChaCha8Rng,
}

impl Map {
    pub fn new(width: u32, height: u32, seed: u64) -> Map {
        Map {
            width,
            height,
            features: Vec::new(),
            parcels: Vec::new(),
            vehicles: Vec::new(),
            rng: ChaCha8Rng::seed_from_u64(seed),
        }
    }

    pub fn add_feature(&mut self, feature: Box<dyn Feature>) {
        self.features.push(feature);
    }

    pub fn add_parcel(&mut self, parcel: LandParcel) {
        self.parcels.push(parcel);
    }

    pub fn is_occupied(&self, point: Point) -> bool {
        for feature in &self.features {
            if feature.contains(point) {
                return true;
            }
        }

        false
    }

    pub fn parcel_at(&self, point: Point) -> Option<u64> {
        for parcel in &self.parcels {
            if parcel.contains(point) {
                return Some(parcel.id);
            }
        }

        None
    }

    pub fn road(&self) -> Option<&Road> {
        for feature in &self.features {
            if let Some(road) = feature.as_any().downcast_ref::<Road>() {
                return Some(road);
            }
        }

        None
    }

    pub fn update_vehicles(&mut self) {
        let road = match self.road() {
            Some(road) => road,
            None => return,
        };

        let road_tiles = road.points();

        let mut remove = Vec::new();

        for (index, vehicle) in self.vehicles.iter_mut().enumerate() {
            let mut options = Vec::new();

            for neighbor in vehicle.position.orthogonal_neighbors() {
                if !road_tiles.contains(&neighbor) {
                    continue;
                }

                if let Some(previous) = vehicle.previous {
                    if neighbor == previous {
                        continue;
                    }
                }

                options.push(neighbor);
            }

            if options.is_empty() {
                remove.push(index);
                continue;
            }

            let current = vehicle.position;

            let mut next = options[0];

            for option in &options {
                match vehicle.direction {
                    Direction::North if option.y < current.y => {
                        next = *option;
                        break;
                    }

                    Direction::South if option.y > current.y => {
                        next = *option;
                        break;
                    }

                    Direction::East if option.x > current.x => {
                        next = *option;
                        break;
                    }

                    Direction::West if option.x < current.x => {
                        next = *option;
                        break;
                    }

                    _ => {}
                }
            }

            let new_direction = if next.x > current.x {
                Direction::East
            } else if next.x < current.x {
                Direction::West
            } else if next.y > current.y {
                Direction::South
            } else {
                Direction::North
            };

            vehicle.set_direction(new_direction);
            vehicle.move_to(next);

            if next.x < 0
                || next.y < 0
                || next.x >= self.width as i32
                || next.y >= self.height as i32
            {
                remove.push(index);
            }
        }

        for index in remove.into_iter().rev() {
            self.vehicles.remove(index);
        }

        if self.rng.random_range(0..100) < 10 {
            let exits = match self.road() {
                Some(road) => road.border_exits(),
                None => Vec::new(),
            };

            if !exits.is_empty() {
                let exit = exits[self.rng.random_range(0..exits.len())];

                let direction = match (exit.x, exit.y, self.width as i32, self.height as i32) {
                    (0, _, _, _) => Direction::East,
                    (x, _, width, _) if x == width - 1 => Direction::West,
                    (_, 0, _, _) => Direction::South,
                    (_, y, _, height) if y == height - 1 => Direction::North,
                    _ => Direction::East,
                };

                self.vehicles.push(Vehicle::new(exit, direction));
            }
        }
    }
}
