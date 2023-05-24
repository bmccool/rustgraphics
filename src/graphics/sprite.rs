use super::point::Point;
use super::line::Line;
use super::rotation::{Rotation, RotationMatrixX, RotationMatrixY, RotationMatrixZ, self};


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


pub trait Render { fn render(&self, f: impl FnOnce(i16, i16, u32) -> Result<(), String> + std::marker::Copy) -> Result<String, String>; }

impl Render for Sprite {
    fn render(&self, f: impl FnOnce(i16, i16, u32) -> Result<(), String> + std::marker::Copy) -> Result<String, String> {
        let rotation_matrix_x = RotationMatrixX { angle: self.angle_x };
        //let rotation_matrix_y = RotationMatrixY { angle: self.angle_y };
        //let rotation_matrix_z = RotationMatrixZ { angle: self.angle_z };
        for p in self.points.iter() {
            let mut rotated_point = *p;
            rotated_point = rotation_matrix_x.rotate(self.angle_x, rotated_point);
            //rotated_point = rotation_matrix_y.rotate(self.angle_y, rotated_point);
            let _ = f(rotated_point.x as i16, rotated_point.y as i16, 0xFFFFFFFFu32);
        }
        return Ok("TODO".to_string());
    }

    /*
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
    */
}
