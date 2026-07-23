use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::world::feature::Feature;
use crate::world::geometry::point::Point;
use crate::world::terrain::river::River;

#[derive(Clone, Copy, Eq, PartialEq)]
struct Node {
    point: Point,
    cost: i32,
    priority: i32,
    direction: Direction,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum Direction {
    None,
    North,
    South,
    East,
    West,
}

impl Direction {
    fn between(a: Point, b: Point) -> Self {
        if b.x > a.x {
            Self::East
        } else if b.x < a.x {
            Self::West
        } else if b.y > a.y {
            Self::South
        } else if b.y < a.y {
            Self::North
        } else {
            Self::None
        }
    }

    fn turn_cost(self, next: Direction) -> i32 {
        match (self, next) {
            (Direction::None, _) => 0,
            (a, b) if a == b => 0,
            _ => 15,
        }
    }
}

pub struct Pathfinder;

impl Pathfinder {
    pub fn find_path(
        width: u32,
        height: u32,
        start: Point,
        goal: Point,
        river: &River,
        preferred_direction: Direction,
    ) -> Vec<Point> {
        let mut open = BinaryHeap::new();

        let mut came_from: HashMap<(Point, Direction), (Point, Direction)> = HashMap::new();

        let mut cost_so_far: HashMap<(Point, Direction), i32> = HashMap::new();

        open.push(Node {
            point: start,
            cost: 0,
            priority: 0,
            direction: preferred_direction,
        });

        cost_so_far.insert((start, preferred_direction), 0);

        let mut visited: HashSet<(Point, Direction)> = HashSet::new();

        while let Some(current) = open.pop() {
            let state = (current.point, current.direction);

            if visited.contains(&state) {
                continue;
            }

            visited.insert(state);

            if current.point == goal {
                return reconstruct(came_from, state, start);
            }

            for next in current.point.orthogonal_neighbors() {
                if !next.in_bounds(width, height) {
                    continue;
                }

                let direction = Direction::between(current.point, next);

                let mut new_cost = current.cost + 1;

                // Rivers are expensive, not forbidden
                if river.contains(next) {
                    new_cost += 250;
                }

                // Avoid zig-zag roads
                new_cost += current.direction.turn_cost(direction);

                // Keep arterials straight
                if direction != preferred_direction {
                    new_cost += 5;
                }

                let key = (next, direction);

                if !cost_so_far.contains_key(&key) || new_cost < cost_so_far[&key] {
                    cost_so_far.insert(key, new_cost);

                    came_from.insert(key, (current.point, current.direction));

                    let heuristic = (goal.x - next.x).abs() + (goal.y - next.y).abs();

                    open.push(Node {
                        point: next,

                        cost: new_cost,

                        priority: new_cost + heuristic,

                        direction,
                    });
                }
            }
        }

        Vec::new()
    }
}

fn reconstruct(
    came_from: HashMap<(Point, Direction), (Point, Direction)>,

    mut current: (Point, Direction),

    start: Point,
) -> Vec<Point> {
    let mut path = vec![current.0];

    while current.0 != start {
        if let Some(previous) = came_from.get(&current) {
            current = *previous;

            path.push(current.0);
        } else {
            break;
        }
    }

    path.reverse();

    path
}
