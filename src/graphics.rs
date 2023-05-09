mod graphics {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

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
        /*
        let slope = (self.start.y - self.end.y) /  (self.start.x - self.end.x);
        print!("slope: {}", slope);
        if slope < 1.0 {
            if self.current.x > self.end.x {
                self.current.x -= 1.0;
                self.current.y -= slope;
            }
            else {
                self.current.x += 1.0;
                self.current.y += slope;
            }

        } else {
            if self.current.y > self.end.y {
                self.current.x -= 1.0 / slope;
                self.current.y -= 1.0;
            }
            else {
                self.current.x += 1.0 / slope;
                self.current.y += 1.0;
            }
        }
        Some(self.current)
        */

    }
}