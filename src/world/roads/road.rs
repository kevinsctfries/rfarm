use std::collections::{HashMap, HashSet};

use crate::world::feature::Feature;
use crate::world::geometry::point::Point;

#[derive(Clone, Copy)]
struct RoadConnection {
    north: bool,
    south: bool,
    east: bool,
    west: bool,
}

impl RoadConnection {
    fn new() -> Self {
        Self {
            north: false,
            south: false,
            east: false,
            west: false,
        }
    }

    fn symbol(&self) -> char {
        match (self.north, self.south, self.east, self.west) {
            (true, true, false, false) => '┃',
            (false, false, true, true) => '━',

            (false, true, true, false) => '┏',
            (false, true, false, true) => '┓',
            (true, false, true, false) => '┗',
            (true, false, false, true) => '┛',

            (true, true, true, false) => '┣',
            (true, true, false, true) => '┫',
            (true, false, true, true) => '┻',
            (false, true, true, true) => '┳',

            (true, true, true, true) => '╋',

            // These are only true dead ends inside the map.
            // Border exits are handled in symbol_at().
            _ => ' ',
        }
    }
}

pub struct Road {
    pub tiles: HashMap<Point, RoadConnection>,

    pub width: i32,
    pub height: i32,

    // Points where a road is meant to visibly continue past the map edge.
    // Only these points get a faked "outside the map" connection when
    // rendered - see symbol_at(). Without this, any tile that merely
    // happens to touch the border (e.g. a path briefly running alongside
    // an edge) would incorrectly render as if it exited the map there too.
    border_exits: HashSet<Point>,
}

impl Road {
    pub fn new(width: u32, height: u32) -> Road {
        Road {
            tiles: HashMap::new(),
            width: width as i32,
            height: height as i32,
            border_exits: HashSet::new(),
        }
    }

    // Mark a point as a genuine border exit for this road (e.g. the start
    // or end of an arterial), rather than a tile that just happens to sit
    // on the map's edge while passing through.
    pub fn mark_border_exit(&mut self, point: Point) {
        self.border_exits.insert(point);
    }

    pub fn add_segment(&mut self, point: Point) {
        self.tiles.entry(point).or_insert(RoadConnection::new());

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
                    x: point.x + 1,
                    y: point.y,
                },
                2,
            ),
            (
                Point {
                    x: point.x - 1,
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
                0 => {
                    self.tiles.get_mut(&point).unwrap().north = true;
                    self.tiles.get_mut(&neighbor).unwrap().south = true;
                }

                1 => {
                    self.tiles.get_mut(&point).unwrap().south = true;
                    self.tiles.get_mut(&neighbor).unwrap().north = true;
                }

                2 => {
                    self.tiles.get_mut(&point).unwrap().east = true;
                    self.tiles.get_mut(&neighbor).unwrap().west = true;
                }

                3 => {
                    self.tiles.get_mut(&point).unwrap().west = true;
                    self.tiles.get_mut(&neighbor).unwrap().east = true;
                }

                _ => {}
            }
        }
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
        let mut connection = *self.tiles.get(&point)?;

        // Fake outside-world connections, but only at genuine border exits -
        // not every tile that happens to touch the map's edge.
        if self.border_exits.contains(&point) {
            if point.x == 0 {
                connection.west = true;
            }

            if point.x == self.width - 1 {
                connection.east = true;
            }

            if point.y == 0 {
                connection.north = true;
            }

            if point.y == self.height - 1 {
                connection.south = true;
            }
        }

        Some(connection.symbol())
    }

    fn priority(&self) -> u32 {
        200
    }
}
