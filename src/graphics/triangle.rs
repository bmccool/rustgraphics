use super::line::Bresenhams;
use super::line::Line;
use super::line::Fill;
use super::line::Slope;
use std::collections::HashMap;

use super::point::Point;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub points: [Point; 3],
}

pub trait Shade { fn shade(&self, shade: u8, f: impl FnOnce(i16, i16, u32) -> Result<(), String> + std::marker::Copy) -> Result<String, String>; }

impl Shade for Triangle {
    fn shade(&self, _shade: u8, f: impl FnOnce(i16, i16, u32) -> Result<(), String> + std::marker::Copy) -> Result<String, String> {
        let a = self.points[0];
        let b = self.points[1];
        let c = self.points[2];
        let ab = Line { start: a, end: b, current: a };
        let bc = Line { start: b, end: c, current: b };
        let ca = Line { start: c, end: a, current: c };
        let lines = vec![ab, bc, ca];
        fill_lines(lines, f)?;
        Ok("SUCCESS".to_string())
    }
}

fn fill_lines(lines: Vec<Line>, f: impl FnOnce(i16, i16, u32) -> Result<(), String> + std::marker::Copy) -> Result<String, String> {
    /*
    ASSUMPTION: Points are given in clockwise order
    If line Y is increasing, we are on the left buffer (clockwise)
    If line Y is decreasing, we are on the right buffer (clockwise)
    if line Y is neither increasing nor decreasing
        min x of line to left buffer, max x of line to right buffer    
    */
    /*
    ASSUMPTION: Filling relative to Z axis.
     */
    let mut l_buf = HashMap::<i16, i16>::new();
    let mut r_buf = HashMap::<i16, i16>::new();
    for line in lines {
        //line.fill(f)?;
        if line.slope().y > 0.0 {
            // left buffer
            let points = line.bresenhams();
            for point in points {
                l_buf.insert(point.y as i16, point.x as i16);
            }
        } else if line.slope().y < 0.0 {
            // right buffer
            let points = line.bresenhams();
            for point in points {
                r_buf.insert(point.y as i16, point.x as i16);
            }
        } else {
            // neither
            if line.start.x < line.end.x {
                l_buf.insert(line.start.y as i16, line.start.x as i16);
                r_buf.insert(line.end.y as i16, line.end.x as i16);
            } else {
                l_buf.insert(line.end.y as i16, line.end.x as i16);
                r_buf.insert(line.start.y as i16, line.start.x as i16);

            }

        }
    }
    for (key, value) in l_buf.into_iter() {
        let x = value;
        let y = key;
        let z = 0;
        let p1 = Point{x: x as f64, y: y as f64, z: z as f64};
        let x2 = r_buf.get(&key).unwrap();
        let p2 = Point{x: *x2 as f64, y: y as f64, z: z as f64};
        let line = Line{start: p1, end: p2, current: p1};
        line.fill(f)?;
    }
    Ok("SUCCESS".to_string())
}