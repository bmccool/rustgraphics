use super::point::Point;
use super::line::Line;
use super::line::Bresenhams;
use super::rotation::{Rotation, RotationMatrixX, RotationMatrixY, RotationMatrixZ, self};
use super::triangle::Triangle;
use itertools::Itertools;



#[derive(Default)]
pub struct Sprite{
    // I'm calling Sprite a collection of things in 3D that should move/rotate/draw together
    // Points are RELATIVE TO ORIGIN
    // RENDER will cast them back to absolute coordinates
    pub points: Vec<Point>,
    pub lines: Vec<Line>,
    pub tris: Vec<Triangle>,
    //pub triangles: Vec<Triangle>,
    //pub quads: Vec<Quad>,
    pub origin: Point,
    pub angle_x: f64,
    pub angle_y: f64,
    pub angle_z: f64,
}

pub fn from_points_faces(points: Vec<f64>, faces: Vec<u64>, scale: f64) -> Result<Sprite, String> {
    // Returns a Sprite from a list of points and faces
    // points is a vector of floats, with every group of three corresponding to a point
    // faces is a vector of ints, with a pattern like
    //     <number of points in face> <point index> <point index> <point index> <number of points in face> <point index> <point index> <point index> ...
    //    where the number of points in face could be 3 (tri), 4 (quad), or more (poly?)
    //   and the point index is the index of the point in the points vector
    // The origin will default to 0,0,0
    let mut sprite = Sprite{ origin: Point{x: 0.0, y: 0.0, z: 0.0 }, ..Default::default()};

    //for chunk in points.into_iter().chunks(3).into_iter() {
    //    // TODO I think the unwrap should be nicer.  Either pattern match, or unwarp_or_else thing
    //    // Probably fine unless it gets bad data
    //    sprite.points.push(Point{x: chunk.next().unwrap(), y: chunk.next().unwrap(), z: chunk.next().unwrap()});
    //}
    for i in 0..points.len() {
        if i % 3 == 0 {
            sprite.points.push(Point{x: points[i] * scale, y: points[i+1] * scale, z: points[i+2] * scale});
        }
    }

    let mut face_iter = faces.into_iter();
    while let Some(i) = face_iter.next() {
        match i {
            // TODO
            // The format here looks AWFULLY similar.  Could this be generic?
            3 => {
                let index_a = face_iter.next().unwrap() as usize;
                let index_b = face_iter.next().unwrap() as usize;
                let index_c = face_iter.next().unwrap() as usize;

                sprite.lines.push(Line{start: sprite.points[index_a], end: sprite.points[index_b], current: sprite.points[index_a]});
                sprite.lines.push(Line{start: sprite.points[index_b], end: sprite.points[index_c], current: sprite.points[index_b]});
                sprite.lines.push(Line{start: sprite.points[index_c], end: sprite.points[index_a], current: sprite.points[index_c]});
            },
            4 => {
                let index_a = face_iter.next().unwrap() as usize;
                let index_b = face_iter.next().unwrap() as usize;
                let index_c = face_iter.next().unwrap() as usize;
                let index_d = face_iter.next().unwrap() as usize;

                sprite.lines.push(Line{start: sprite.points[index_a], end: sprite.points[index_b], current: sprite.points[index_a]});
                sprite.lines.push(Line{start: sprite.points[index_b], end: sprite.points[index_c], current: sprite.points[index_b]});
                sprite.lines.push(Line{start: sprite.points[index_c], end: sprite.points[index_d], current: sprite.points[index_c]});
                sprite.lines.push(Line{start: sprite.points[index_d], end: sprite.points[index_a], current: sprite.points[index_d]});
            },
            5 => {
                let index_a = face_iter.next().unwrap() as usize;
                let index_b = face_iter.next().unwrap() as usize;
                let index_c = face_iter.next().unwrap() as usize;
                let index_d = face_iter.next().unwrap() as usize;
                let index_e = face_iter.next().unwrap() as usize;

                sprite.lines.push(Line{start: sprite.points[index_a], end: sprite.points[index_b], current: sprite.points[index_a]});
                sprite.lines.push(Line{start: sprite.points[index_b], end: sprite.points[index_c], current: sprite.points[index_b]});
                sprite.lines.push(Line{start: sprite.points[index_c], end: sprite.points[index_d], current: sprite.points[index_c]});
                sprite.lines.push(Line{start: sprite.points[index_d], end: sprite.points[index_e], current: sprite.points[index_d]});
                sprite.lines.push(Line{start: sprite.points[index_e], end: sprite.points[index_a], current: sprite.points[index_e]});
            }
            _ => return Err("Can't handle faces with that number of verticies".to_string()),
        }
    }
    return Ok(sprite);
}

pub trait Render { fn render(&self, f: impl FnOnce(i16, i16, u32) -> Result<(), String> + std::marker::Copy) -> Result<String, String>; }

impl Render for Sprite {
    fn render(&self, f: impl FnOnce(i16, i16, u32) -> Result<(), String> + std::marker::Copy) -> Result<String, String> {
        let rotation_matrix_x = RotationMatrixX { angle: self.angle_x };
        let rotation_matrix_y = RotationMatrixY { angle: self.angle_y };
        let rotation_matrix_z = RotationMatrixZ { angle: self.angle_z };

        // Render Points
        for p in self.points.iter() {
            let mut rotated_point = *p;
            rotated_point = rotation_matrix_x.rotate(self.angle_x, rotated_point);
            rotated_point = rotation_matrix_y.rotate(self.angle_y, rotated_point);
            rotated_point = rotation_matrix_z.rotate(self.angle_z, rotated_point);
            rotated_point = rotated_point + self.origin;
            let _ = f(rotated_point.x as i16, rotated_point.y as i16, 0xFFFFFFFFu32);
        }

        // Render Lines
        for l in self.lines.iter() {
            let mut rotated_line = *l;
            rotated_line.start = rotation_matrix_x.rotate(self.angle_x, rotated_line.start);
            rotated_line.start = rotation_matrix_y.rotate(self.angle_y, rotated_line.start);
            rotated_line.start = rotation_matrix_z.rotate(self.angle_z, rotated_line.start);
            rotated_line.end = rotation_matrix_x.rotate(self.angle_x, rotated_line.end);
            rotated_line.end = rotation_matrix_y.rotate(self.angle_y, rotated_line.end);
            rotated_line.end = rotation_matrix_z.rotate(self.angle_z, rotated_line.end);
            rotated_line.start = rotated_line.start + self.origin;
            rotated_line.end = rotated_line.end + self.origin;
            for p in rotated_line.bresenhams() {
                let _ = f(p.x as i16, p.y as i16, 0xFFFFFFFFu32);
            }
        }

        // Render Triangles
        for t in self.tris.iter() {
            let mut rotated_tri = *t;
            rotated_tri.points[0] = rotation_matrix_x.rotate(self.angle_x, rotated_tri.points[0]);
            rotated_tri.points[0] = rotation_matrix_y.rotate(self.angle_y, rotated_tri.points[0]);
            rotated_tri.points[0] = rotation_matrix_z.rotate(self.angle_z, rotated_tri.points[0]);
            rotated_tri.points[1] = rotation_matrix_x.rotate(self.angle_x, rotated_tri.points[1]);
            rotated_tri.points[1] = rotation_matrix_y.rotate(self.angle_y, rotated_tri.points[1]);
            rotated_tri.points[1] = rotation_matrix_z.rotate(self.angle_z, rotated_tri.points[1]);
            rotated_tri.points[2] = rotation_matrix_x.rotate(self.angle_x, rotated_tri.points[2]);
            rotated_tri.points[2] = rotation_matrix_y.rotate(self.angle_y, rotated_tri.points[2]);
            rotated_tri.points[2] = rotation_matrix_z.rotate(self.angle_z, rotated_tri.points[2]);
            rotated_tri.points[0] = rotated_tri.points[0] + self.origin;
            rotated_tri.points[1] = rotated_tri.points[1] + self.origin;
            rotated_tri.points[2] = rotated_tri.points[2] + self.origin;

            // Draw A-B
            let line = Line { start: rotated_tri.points[0], end: rotated_tri.points[1], current: rotated_tri.points[0] };
            for p in line.bresenhams() {
                let _ = f(p.x as i16, p.y as i16, 0xFFFFFFFFu32);
            }

            // Draw B-C
            let line = Line { start: rotated_tri.points[1], end: rotated_tri.points[2], current: rotated_tri.points[1] };
            for p in line.bresenhams() {
                let _ = f(p.x as i16, p.y as i16, 0xFFFFFFFFu32);
            }

            // Draw C-A
            let line = Line { start: rotated_tri.points[2], end: rotated_tri.points[0], current: rotated_tri.points[2] };
            for p in line.bresenhams() {
                let _ = f(p.x as i16, p.y as i16, 0xFFFFFFFFu32);
            }
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
