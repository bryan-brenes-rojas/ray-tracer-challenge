use crate::{point::Point, vector::Vector};

pub trait Object { 
    fn normal_at(&self, point: &Point) -> Vector;
}
