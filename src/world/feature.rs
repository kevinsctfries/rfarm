use crate::world::geometry::point::Point;

pub trait Feature {
    fn contains(&self, point: Point) -> bool;

    fn is_border(&self, point: Point) -> bool;

    fn symbol_at(&self, point: Point) -> Option<char>;

    fn priority(&self) -> u32;

    fn as_any(&self) -> &dyn std::any::Any;
}
