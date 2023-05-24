use super::point::Point;
use super::line::Line;


#[derive(Default)]
pub struct Sprite{
    // I'm calling Sprite a collection of things in 3D that should move/rotate/draw together
    // Points are RELATIVE TO ORIGIN
    // RENDER will cast them back to absolute coordinates
    pub points: Vec<Point>,
    pub lines: Vec<Line>,
    //pub triangles: Vec<Triangle>,
    //pub quads: Vec<Quad>,
    pub origin: Point,
    pub angle_x: f64,
    pub angle_y: f64,
    pub angle_z: f64,
}

/*
pub trait Render { fn render(&self, f: fn(i16, i16, u32) -> Result<(), String>) -> Result<String, String>; }

impl Render for Sprite {
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
*/