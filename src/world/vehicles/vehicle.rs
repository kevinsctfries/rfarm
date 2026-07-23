use crate::world::geometry::point::Point;

#[derive(Clone)]
pub struct Vehicle {
    pub position: Point,
    pub previous: Option<Point>,
    pub direction: Direction,
}

#[derive(Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Vehicle {
    pub fn new(position: Point, direction: Direction) -> Self {
        Self {
            position,
            previous: None,
            direction,
        }
    }

    pub fn symbol(&self) -> char {
        match self.direction {
            Direction::North => '^',
            Direction::South => 'v',
            Direction::East => '>',
            Direction::West => '<',
        }
    }

    pub fn move_to(&mut self, point: Point) {
        self.previous = Some(self.position);
        self.position = point;
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }
}
