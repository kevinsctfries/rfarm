use std::collections::HashMap;

use crate::world::feature::Feature;
use crate::world::geometry::point::Point;

#[derive(Clone, Copy, Debug)]
pub struct RoadConnection {
    pub north: bool,
    pub south: bool,
    pub east: bool,
    pub west: bool,
}

impl RoadConnection {
    pub fn empty() -> Self {
        Self {
            north: false,
            south: false,
            east: false,
            west: false,
        }
    }

    pub fn symbol(&self) -> char {
        match (self.north, self.south, self.east, self.west) {
            // straight roads
            (true, true, false, false) => '┃',
            (false, false, true, true) => '━',

            // endpoints
            (true, false, false, false) => '╹',
            (false, true, false, false) => '╻',
            (false, false, true, false) => '╺',
            (false, false, false, true) => '╸',

            // intersections
            (true, true, true, true) => '╋',

            // three-way intersections
            (false, true, true, true) => '┳',
            (true, false, true, true) => '┻',
            (true, true, false, true) => '┫',
            (true, true, true, false) => '┣',

            // corners
            (false, true, true, false) => '┏',
            (false, true, false, true) => '┓',
            (true, false, true, false) => '┗',
            (true, false, false, true) => '┛',

            _ => ' ',
        }
    }
}

pub struct Road {
    pub tiles: HashMap<Point, RoadConnection>,
}

impl Road {
    pub fn new() -> Self {
        Self {
            tiles: HashMap::new(),
        }
    }

    pub fn add_segment(&mut self, point: Point) {
        println!("ADDING ROAD {:?}", point);

        self.tiles.entry(point).or_insert(RoadConnection::empty());

        self.update_neighbors(point);
    }

    fn update_neighbors(&mut self, point: Point) {
        let neighbors = [
            (
                Point {
                    x: point.x,
                    y: point.y - 1,
                },
                0,
            ),
            (
                Point {
                    x: point.x,
                    y: point.y + 1,
                },
                1,
            ),
            (
                Point {
                    x: point.x - 1,
                    y: point.y,
                },
                2,
            ),
            (
                Point {
                    x: point.x + 1,
                    y: point.y,
                },
                3,
            ),
        ];

        for (neighbor, direction) in neighbors {
            if !self.tiles.contains_key(&neighbor) {
                continue;
            }

            match direction {
                // neighbor is north
                0 => {
                    self.tiles.get_mut(&point).unwrap().north = true;
                    self.tiles.get_mut(&neighbor).unwrap().south = true;
                }

                // neighbor is south
                1 => {
                    self.tiles.get_mut(&point).unwrap().south = true;
                    self.tiles.get_mut(&neighbor).unwrap().north = true;
                }

                // neighbor is west
                2 => {
                    self.tiles.get_mut(&point).unwrap().west = true;
                    self.tiles.get_mut(&neighbor).unwrap().east = true;
                }

                // neighbor is east
                3 => {
                    self.tiles.get_mut(&point).unwrap().east = true;
                    self.tiles.get_mut(&neighbor).unwrap().west = true;
                }

                _ => {}
            }
        }
    }

    pub fn symbol_at(&self, point: Point) -> Option<char> {
        self.tiles.get(&point).map(|road| {
            let mut connection = *road;

            if point.x == 0 {
                connection.west = true;
            }

            if point.x == 79 {
                connection.east = true;
            }

            if point.y == 0 {
                connection.north = true;
            }

            if point.y == 39 {
                connection.south = true;
            }

            connection.symbol()
        })
    }
}

impl Feature for Road {
    fn contains(&self, point: Point) -> bool {
        self.tiles.contains_key(&point)
    }

    fn is_border(&self, _point: Point) -> bool {
        false
    }

    fn symbol_at(&self, point: Point) -> Option<char> {
        self.symbol_at(point)
    }

    fn priority(&self) -> u32 {
        200
    }
}
