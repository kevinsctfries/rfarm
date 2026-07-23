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

            for neighbor in point.orthogonal_neighbors() {
                if !neighbor.in_bounds(map.width, map.height) {
                    continue;
                }

                if visited.contains(&neighbor) {
                    continue;
                }

                if map.is_occupied(neighbor) {
                    continue;
                }

                visited.insert(neighbor);
                queue.push_back(neighbor);
            }
        }

        LandRegion { tiles }
    }
}
