use fixed::traits::ToFixed;
use fixed::types::*;
use fixed_sqrt::FixedSqrt;
use std::ops::{Add, Div, Mul, Sub};

pub type FixF = I28F4; //fixed float

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Copy)]
pub struct Pos {
    pub x: FixF, // FixedI32 with 24 integer bits and eight fractional bits.
    pub y: FixF,
}

impl Pos {
    pub fn new(x: FixF, y: FixF) -> Pos {
        Pos { x: x, y: y }
    }

    pub fn from_num<T: ToFixed>(x: T, y: T) -> Self {
        Pos {
            x: FixF::from_num(x),
            y: FixF::from_num(y),
        }
    }

    pub fn square(pos: Pos) -> Pos {
        return Pos::new(pos.x * pos.x, pos.y * pos.y);
    }

    pub fn vec_length(pos: Pos) -> FixF {
        // returns distance from the origin
        let l = Pos::square(pos);
        return (l.x + l.y).sqrt();
    }
    
    pub fn dist(&self, xy: &Pos) -> FixF {
        Pos::vec_length(self.clone() - xy.clone())
    }

    pub fn round(&self) -> Pos {
        Pos {
            x: self.x.round(),
            y: self.y.round(),
        }
    }

    /// Get fractional part that was lost when rounding
    pub fn fractional_part(&self) -> Pos {
        *self - self.round()
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Pos {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<FixF> for Pos {
    type Output = Self;

    fn mul(self, rhs: FixF) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<FixF> for Pos {
    type Output = Self;

    fn div(self, rhs: FixF) -> Self::Output {
        if rhs == 0 {
            panic!("Cannot divide by zero-valued `Rational`!");
        }
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
