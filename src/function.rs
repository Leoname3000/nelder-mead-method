use crate::point::Point;

pub trait Function {
    fn calc(p: Point) -> f64;
}