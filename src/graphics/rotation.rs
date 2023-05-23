use super::point::Point;

pub struct RotationMatrix{
    pub rows: [[f64; 3]; 3],
}

#[derive(Debug, Clone, Copy)]
pub struct RotationMatrixX {
    pub angle: f64,
}

pub struct RotationMatrixY {
    pub angle: f64,
}

pub struct RotationMatrixZ {
    pub angle: f64,
}

pub trait Rotation {
    fn rotate(&self, angle: f64, point: Point) -> Point;
}

fn multiply(matrix: [[f64; 3]; 3], point: Point) -> Point {
    let x = matrix[0][0] * point.x + matrix[0][1] * point.y + matrix[0][2] * point.z;
    let y = matrix[1][0] * point.x + matrix[1][1] * point.y + matrix[1][2] * point.z;
    let z = matrix[2][0] * point.x + matrix[2][1] * point.y + matrix[2][2] * point.z;
    Point { x, y, z }
}

impl Rotation for RotationMatrixX {
    fn rotate(&self, angle: f64, point: Point) -> Point {
        let rows = [
            [angle.cos(), -angle.sin(), 0.0],
            [angle.sin(), angle.cos(), 0.0],
            [0.0, 0.0, 1.0],
        ];
        return multiply(rows, point);
    }
}

impl Rotation for RotationMatrixY {
    fn rotate(&self, angle: f64, point: Point) -> Point {
        let rows = [
            [angle.cos(), 0.0, angle.sin()],
            [0.0, 1.0, 0.0],
            [-angle.sin(), 0.0, angle.cos()],
        ];
        return multiply(rows, point);
    }
}

impl Rotation for RotationMatrixZ {
    fn rotate(&self, angle: f64, point: Point) -> Point {
        let rows = [
            [1.0, 0.0, 0.0],
            [0.0, angle.cos(), -angle.sin()],
            [0.0, angle.sin(), angle.cos()],
        ];
        return multiply(rows, point);
    }
}