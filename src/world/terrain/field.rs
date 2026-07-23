use crate::world::feature::Feature;
use crate::world::geometry::point::Point;
use crate::world::geometry::polygon::Polygon;

pub struct Field {
    pub shape: Polygon,
}

impl Field {
    pub fn test() -> Field {
        Field {
            shape: Polygon::new(vec![
                Point { x: 10, y: 10 },
                Point { x: 30, y: 10 },
                Point { x: 35, y: 25 },
                Point { x: 15, y: 30 },
            ]),
        }
    }

    pub fn can_exist(&self, features: &[Box<dyn Feature>]) -> bool {
        for feature in features {
            if self.shape.overlaps_feature(feature.as_ref()) {
                return false;
            }
        }

        true
    }
}

impl Feature for Field {
    fn contains(&self, point: Point) -> bool {
        self.shape.contains(point)
    }

    fn is_border(&self, _point: Point) -> bool {
        false
    }

    fn symbol(&self) -> char {
        '"'
    }

    fn priority(&self) -> u32 {
        20
    }
}
