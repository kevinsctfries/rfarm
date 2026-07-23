use std::io::{self, Write};

use super::geometry::point::Point;
use super::map::Map;

pub struct Renderer;

impl Renderer {
    pub fn render(map: &Map) {
        let mut output = String::new();

        // Clear terminal, move cursor home, hide cursor.
        output.push_str("\x1b[2J");
        output.push_str("\x1b[H");
        output.push_str("\x1b[?25l");

        for y in 0..map.height {
            for x in 0..map.width {
                let point = Point {
                    x: x as i32,
                    y: y as i32,
                };

                output.push(Self::symbol_at(map, point));
            }

            output.push('\n');
        }

        print!("{}", output);

        io::stdout().flush().unwrap();
    }

    fn symbol_at(map: &Map, point: Point) -> char {
        // Vehicles
        if let Some(vehicle) = map
            .vehicles
            .iter()
            .find(|vehicle| vehicle.position == point)
        {
            return vehicle.symbol();
        }

        // Labels
        if let Some(character) = Self::label_character(map, point) {
            return character;
        }

        let mut symbol = '.';
        let mut highest_priority = 0;

        // Features
        for feature in &map.features {
            if let Some(feature_symbol) = feature.symbol_at(point) {
                if feature.priority() >= highest_priority {
                    symbol = feature_symbol;
                    highest_priority = feature.priority();
                }
            }
        }

        // Parcels
        if symbol == '.' {
            if let Some(parcel_id) = map.parcel_at(point) {
                let neighbors = [
                    Point {
                        x: point.x + 1,
                        y: point.y,
                    },
                    Point {
                        x: point.x,
                        y: point.y + 1,
                    },
                ];

                let mut hedge = false;

                for neighbor in neighbors {
                    if let Some(other_id) = map.parcel_at(neighbor) {
                        if other_id != parcel_id {
                            hedge = true;
                            break;
                        }
                    }
                }

                if hedge {
                    symbol = '#';
                } else {
                    let shades = ['░', '▒', '▓'];

                    symbol = shades[(parcel_id % shades.len() as u64) as usize];
                }
            }
        }

        symbol
    }

    fn label_character(map: &Map, point: Point) -> Option<char> {
        for parcel in &map.parcels {
            let Some(position) = parcel.label_position else {
                continue;
            };

            let name = &parcel.farm_name;

            let start_x = position.x - (name.len() as i32 / 2);

            let offset = point.x - start_x;

            if point.y == position.y && offset >= 0 && offset < name.len() as i32 {
                return name.chars().nth(offset as usize);
            }
        }

        None
    }
}
