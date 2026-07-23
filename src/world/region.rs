use std::collections::{HashSet, VecDeque};

use crate::world::geometry::point::Point;
use crate::world::map::Map;

#[derive(Debug)]
pub struct LandRegion {
    pub tiles: Vec<Point>,
}

impl LandRegion {
    pub fn generate(map: &Map) -> Vec<LandRegion> {
        let mut regions = Vec::new();
        let mut visited = HashSet::<Point>::new();

        for y in 0..map.height {
            for x in 0..map.width {
                let start = Point {
                    x: x as i32,
                    y: y as i32,
                };

                if visited.contains(&start) {
                    continue;
                }

                if map.is_occupied(start) {
                    continue;
                }

                let region = Self::flood_fill(map, start, &mut visited);

                regions.push(region);
            }
        }

        regions
    }

    fn flood_fill(map: &Map, start: Point, visited: &mut HashSet<Point>) -> LandRegion {
        let mut queue = VecDeque::new();
        let mut tiles = Vec::new();

        queue.push_back(start);
        visited.insert(start);

        while let Some(point) = queue.pop_front() {
            tiles.push(point);

            let neighbours = [
                Point {
                    x: point.x + 1,
                    y: point.y,
                },
                Point {
                    x: point.x - 1,
                    y: point.y,
                },
                Point {
                    x: point.x,
                    y: point.y + 1,
                },
                Point {
                    x: point.x,
                    y: point.y - 1,
                },
            ];

            for neighbour in neighbours {
                if neighbour.x < 0
                    || neighbour.y < 0
                    || neighbour.x >= map.width as i32
                    || neighbour.y >= map.height as i32
                {
                    continue;
                }

                if visited.contains(&neighbour) {
                    continue;
                }

                if map.is_occupied(neighbour) {
                    continue;
                }

                visited.insert(neighbour);
                queue.push_back(neighbour);
            }
        }

        LandRegion { tiles }
    }
}
