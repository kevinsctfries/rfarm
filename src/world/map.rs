use super::feature::Feature;
use super::geometry::point::Point;
use super::land_parcel::LandParcel;
use super::roads::road::Road;
use super::vehicles::vehicle::{Direction, Vehicle};

pub struct Map {
    pub width: u32,
    pub height: u32,

    pub features: Vec<Box<dyn Feature>>,

    pub parcels: Vec<LandParcel>,

    pub vehicles: Vec<Vehicle>,
}

impl Map {
    pub fn new(width: u32, height: u32) -> Map {
        Map {
            width,
            height,
            features: Vec::new(),
            parcels: Vec::new(),
            vehicles: Vec::new(),
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
        let road_tiles = match self.road() {
            Some(road) => road.points(),
            None => return,
        };

        for vehicle in &mut self.vehicles {
            let mut options = Vec::new();

            for neighbor in vehicle.position.orthogonal_neighbors() {
                if !road_tiles.contains(&neighbor) {
                    continue;
                }

                // Prevent immediately reversing direction.
                if let Some(previous) = vehicle.previous {
                    if neighbor == previous {
                        continue;
                    }
                }

                options.push(neighbor);
            }

            if options.is_empty() {
                continue;
            }

            let current = vehicle.position;

            let mut next = options[0];

            // Prefer continuing straight.
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

            // Update direction based on actual movement.
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
        }
    }

    fn next_vehicle_position(&self, vehicle: &Vehicle, road: &Road) -> Option<Point> {
        let mut options = Vec::new();

        for neighbor in vehicle.position.orthogonal_neighbors() {
            if !road.contains(neighbor) {
                continue;
            }

            // Prevent immediately reversing direction.
            if let Some(previous) = vehicle.previous {
                if neighbor == previous {
                    continue;
                }
            }

            options.push(neighbor);
        }

        if options.is_empty() {
            return None;
        }

        // Prefer continuing straight.
        for option in &options {
            match vehicle.direction {
                Direction::North if option.y < vehicle.position.y => return Some(*option),

                Direction::South if option.y > vehicle.position.y => return Some(*option),

                Direction::East if option.x > vehicle.position.x => return Some(*option),

                Direction::West if option.x < vehicle.position.x => return Some(*option),

                _ => {}
            }
        }

        // Otherwise choose a turn.
        Some(options[0])
    }
}
