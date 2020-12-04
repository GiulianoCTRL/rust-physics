use std::ops::{Add, Sub, Mul, Div};
use std::fmt;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Number {
    Float(f64),
    Int(i64),
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::Float(float) => float.fmt(f),
            Number::Int(int) => int.fmt(f),
        }
    }
}

impl Number {
    /// Determine if Number is of float type
    pub fn is_float(&self) -> bool {
        match self {
            Number::Float(_) => true,
            Number::Int(_) => false,
        }
    }

    /// Determine if Number is of integer type
    pub fn is_int(&self) -> bool {
        !self.is_float()
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Number::Float(f0), Number::Float(f1)) => Number::Float(f0 + f1),
            (Number::Float(f), Number::Int(i)) => Number::Float(f + i as f64),
            (Number::Int(i), Number::Float(f)) => Number::Float(i as f64 + f),
            (Number::Int(i0), Number::Int(i1)) => Number::Int(i0 + i1),
        }
    }

}

pub struct Velocity;

// TODO: Everything
// Also, figure out a way to use any number type or even create number type for different measurement units
// e.g. distance, time, weight and so on


impl Velocity {
    /// Get avg velocity
    pub fn from_time_distance(s: i64, t: i64) -> i64 {
        s / t
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
