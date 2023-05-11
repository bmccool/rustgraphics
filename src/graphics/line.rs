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