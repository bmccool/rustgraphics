use super::point::Point;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub points: [Point; 3],
}
