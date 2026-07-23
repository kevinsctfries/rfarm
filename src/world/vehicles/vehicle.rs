use crate::world::geometry::point::Point;

#[derive(Clone)]
pub struct Vehicle {
    pub position: Point,

    // Where we came from last tick.
    pub previous: Option<Point>,

    // Current movement direction.
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

    pub fn next_position(&self) -> Point {
        match self.direction {
            Direction::North => Point {
                x: self.position.x,
                y: self.position.y - 1,
            },

            Direction::South => Point {
                x: self.position.x,
                y: self.position.y + 1,
            },

            Direction::East => Point {
                x: self.position.x + 1,
                y: self.position.y,
            },

            Direction::West => Point {
                x: self.position.x - 1,
                y: self.position.y,
            },
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
