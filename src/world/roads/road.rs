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

    fn neighbors(&self, point: Point) -> Vec<Point> {
        let mut result = Vec::new();

        if self.north {
            result.push(Point {
                x: point.x,
                y: point.y - 1,
            });
        }

        if self.south {
            result.push(Point {
                x: point.x,
                y: point.y + 1,
            });
        }

        if self.east {
            result.push(Point {
                x: point.x + 1,
                y: point.y,
            });
        }

        if self.west {
            result.push(Point {
                x: point.x - 1,
                y: point.y,
            });
        }

        result
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

            _ => ' ',
        }
    }
}

pub struct Road {
    tiles: HashMap<Point, RoadConnection>,

    pub width: i32,
    pub height: i32,

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

    pub fn contains(&self, point: Point) -> bool {
        self.tiles.contains_key(&point)
    }

    pub fn points(&self) -> Vec<Point> {
        self.tiles.keys().copied().collect()
    }

    pub fn neighbors(&self, point: Point) -> Vec<Point> {
        match self.tiles.get(&point) {
            Some(connection) => connection.neighbors(point),
            None => Vec::new(),
        }
    }

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

    pub fn border_exits(&self) -> Vec<Point> {
        self.border_exits.iter().copied().collect()
    }

    pub fn connections(&self) -> Vec<(Point, Vec<Point>)> {
        self.tiles
            .iter()
            .map(|(point, connection)| (*point, connection.neighbors(*point)))
            .collect()
    }
}

impl Feature for Road {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn contains(&self, point: Point) -> bool {
        self.tiles.contains_key(&point)
    }

    fn is_border(&self, _point: Point) -> bool {
        false
    }

    fn symbol_at(&self, point: Point) -> Option<char> {
        let mut connection = *self.tiles.get(&point)?;

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
