use super::point::Point;

#[derive(Debug, Clone)]
pub struct Path {
    pub points: Vec<Point>,
}

impl Path {
    pub fn new(points: Vec<Point>) -> Path {
        Path { points }
    }

    pub fn distance_to(&self, point: Point) -> f32 {
        let mut closest = f32::MAX;

        for segment in self.points.windows(2) {
            let start = segment[0];
            let end = segment[1];

            let distance = distance_to_segment(point, start, end);

            if distance < closest {
                closest = distance;
            }
        }

        closest
    }
}

fn distance_to_segment(point: Point, start: Point, end: Point) -> f32 {
    let px = point.x as f32;
    let py = point.y as f32;

    let x1 = start.x as f32;
    let y1 = start.y as f32;

    let x2 = end.x as f32;
    let y2 = end.y as f32;

    let dx = x2 - x1;
    let dy = y2 - y1;

    if dx == 0.0 && dy == 0.0 {
        return point.distance_to(start);
    }

    let t = ((px - x1) * dx + (py - y1) * dy) / (dx * dx + dy * dy);

    let t = t.clamp(0.0, 1.0);

    let closest_x = x1 + t * dx;
    let closest_y = y1 + t * dy;

    let dx = px - closest_x;
    let dy = py - closest_y;

    (dx * dx + dy * dy).sqrt()
}
