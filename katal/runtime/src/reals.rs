use parity_codec::{Decode, Encode};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Encode, Decode, Default, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Real(i64);

impl Add for Real {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        FixedPoint(self.0 + rhs.0)
    }
}
