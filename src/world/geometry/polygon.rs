use super::point::Point;
use crate::world::feature::Feature;

#[derive(Clone)]
pub struct Polygon {
    pub tiles: Vec<Point>,
}

impl Polygon {
    pub fn new(points: Vec<Point>) -> Polygon {
        Polygon { tiles: points }
    }

    pub fn from_points(points: Vec<Point>) -> Polygon {
        Self::new(points)
    }

    pub fn contains(&self, point: Point) -> bool {
        self.tiles.contains(&point)
    }

    pub fn overlaps_feature(&self, feature: &dyn Feature) -> bool {
        for tile in &self.tiles {
            if feature.contains(*tile) {
                return true;
            }
        }

        false
    }
}
