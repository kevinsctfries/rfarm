use super::geometry::point::Point;
use super::geometry::polygon::Polygon;

use std::collections::HashSet;

pub struct LandParcel {
    pub id: u64,

    pub tiles: HashSet<Point>,

    pub boundary: Option<Polygon>,

    pub seed: u64,

    pub farm_name: String,

    pub label_position: Option<Point>,
}

impl LandParcel {
    pub fn new(id: u64, tiles: HashSet<Point>, farm_name: String, seed: u64) -> Self {
        let label_position = Self::find_label_position(&tiles, &farm_name);

        Self {
            id,
            tiles,
            boundary: None,
            seed,
            farm_name,
            label_position,
        }
    }

    pub fn contains(&self, point: Point) -> bool {
        self.tiles.contains(&point)
    }

    // Find a place where the farm name can fit.
    fn find_label_position(tiles: &HashSet<Point>, name: &str) -> Option<Point> {
        if tiles.is_empty() {
            return None;
        }

        let center = Self::calculate_centroid(tiles);

        if Self::can_fit_name(tiles, center, name) {
            return Some(center);
        }

        // Search outward from centroid.
        for radius in 1..20 {
            for y in -radius..=radius {
                for x in -radius..=radius {
                    let point = Point {
                        x: center.x + x,
                        y: center.y + y,
                    };

                    if Self::can_fit_name(tiles, point, name) {
                        return Some(point);
                    }
                }
            }
        }

        None
    }

    // Calculate geometric center of parcel.
    fn calculate_centroid(tiles: &HashSet<Point>) -> Point {
        let total_x: i32 = tiles.iter().map(|p| p.x).sum();

        let total_y: i32 = tiles.iter().map(|p| p.y).sum();

        Point {
            x: total_x / tiles.len() as i32,
            y: total_y / tiles.len() as i32,
        }
    }

    // Checks if the whole text string fits horizontally.
    fn can_fit_name(tiles: &HashSet<Point>, position: Point, name: &str) -> bool {
        let half_width = name.len() as i32 / 2;

        for offset in -half_width..=half_width {
            let point = Point {
                x: position.x + offset,
                y: position.y,
            };

            if !tiles.contains(&point) {
                return false;
            }
        }

        true
    }
}
