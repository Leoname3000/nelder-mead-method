use std::f64;
use crate::function::Function;
use crate::point::Point;

pub struct RosenbrockFunction;

impl Function for RosenbrockFunction {
    fn calc(p: Point) -> f64 {
        return f64::powi(1. - p.x, 2) + 100. * f64::powi(p.y - f64::powi(p.x, 2), 2);
    }
}