use super::point::Point;

#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub start: Point,
    pub end: Point,
    pub current: Point,
}


pub trait Magnitude { fn magnitude(&self) -> f64; }

impl Magnitude for Line {
    fn magnitude(&self) -> f64 {
        let x = self.end.x - self.start.x;
        let y = self.end.y - self.start.y;
        (x * x + y * y).sqrt()
    }
}

pub trait Bresenhams {
    fn bresenhams(&self) -> Vec<Point>;
}

pub trait Fill { fn fill(&self, f: impl FnOnce(i16, i16, u32) -> Result<(), String> + std::marker::Copy) -> Result<String, String>; }

impl Fill for Line {
    fn fill(&self, f: impl FnOnce(i16, i16, u32) -> Result<(), String> + std::marker::Copy) -> Result<String, String> {
        let points = self.bresenhams();
        for point in points {
            f(point.x as i16, point.y as i16, 0xFFFFFFFFu32)?;
        }
        Ok("SUCCESS".to_string())
    }
}

pub trait Slope { fn slope(&self) -> Point; }

impl Slope for Line {
    fn slope(&self) -> Point {
        let x = self.end.x - self.start.x;
        let y = self.end.y - self.start.y;
        let z = self.end.z - self.start.z;
        Point { x: x, y: y, z: z }
    }
}

impl Bresenhams for Line {
    fn bresenhams(&self) -> Vec<Point> {
        let x0 = self.start.x as i16;
        let y0 = self.start.y as i16;
        let x1 = self.end.x as i16;
        let y1 = self.end.y as i16;
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        if dy <= dx {
            if x0 > x1 {
                return bresenhams_low(x1, y1, x0, y0);
            } else {
                return bresenhams_low(x0, y0, x1, y1);
            }
        } else {
            if y0 > y1 {
                return bresenhams_high(x1, y1, x0, y0);
            } else {
                return bresenhams_high(x0, y0, x1, y1);
            }
        }
    }
}

fn bresenhams_low(x0: i16, y0: i16, x1: i16, y1: i16) -> Vec<Point> {
    let mut points = Vec::new();
    let dx = x1 - x0;
    let mut dy = y1 - y0;
    let mut yi = 1;
    if dy < 0 {
        yi = -1;
        dy = -dy;
    }
    let mut d = 2 * dy - dx;
    let mut y = y0;
    for x in x0..x1 {
        points.push(Point{x: x as f64, y: y as f64, z: 0.0});
        if d > 0 {
            y = y + yi;
            d = d - 2 * dx;
        }
        d = d + 2 * dy;
    }
    return points;
}

fn bresenhams_high(x0: i16, y0: i16, x1: i16, y1: i16) -> Vec<Point> {
    let mut points = Vec::new();
    let mut dx = x1 - x0;
    let dy = y1 - y0;
    let mut xi = 1;
    if dx < 0 {
        xi = -1;
        dx = -dx;
    }
    let mut d = 2 * dx - dy;
    let mut x = x0;
    for y in y0..y1 {
        points.push(Point{x: x as f64, y: y as f64, z: 0.0});
        if d > 0 {
            x = x + xi;
            d = d - 2 * dy;
        }
        d = d + 2 * dx;
    }
    return points;
}

// Implement `Iterator` for `Line`.
// The `Iterator` trait only requires a method to be defined for the `next` element.
impl Iterator for Line {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        //print!("current: {:?}, end: {:?}, ", self.current, self.end);
        if (self.current.x.floor() == self.end.x.floor()) && (self.current.y.floor() == self.end.y.floor()) {
            //println!("DONE!");
            return None;
        }
        let centi_x = (self.end.x - self.start.x) / self.magnitude();
        let centi_y = (self.end.y - self.start.y) / self.magnitude();
        let mut p = self.current;
        while (self.current.x.floor() == p.x.floor()) && (self.current.y.floor() == p.y.floor()) {
            p.x += centi_x;
            p.y += centi_y;
        }
        self.current = p;
        return Some(p);
    }
}

