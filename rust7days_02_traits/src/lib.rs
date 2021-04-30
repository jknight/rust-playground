extern crate rand;
// General purpose import:
//use rand::prelude::*;
// explict import:
use rand::Rng;
use std::ops::Add;

#[derive(Debug)]  // needed for {:?} formatting
pub struct Point {
    pub x: i32,
    pub y: i32,
}

// Implement a random function on the Point struct
impl Point {
    pub fn random() -> Self {
        let mut tr = rand::thread_rng();
        Point { x: tr.gen(), y: tr.gen(), }
    }
}

// Implement 'std::ops::Add' for Point
impl Add for Point {
    type Output = Point; // specify the output type for this trait
    fn add(self, other: Point) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}


