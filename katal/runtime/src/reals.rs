// 'None' is smaller than any number!
use parity_codec::{Decode, Encode};
use std::ops::{Add, Div, Mul, Neg, Sub};

// The scale factor
const SF: i128 = 1000000000;

// The maximum and minimum values supported by i64
const MAX: i128 = i64::max_value() as i128;
const MIN: i128 = i64::min_value() as i128;

#[derive(Decode, Encode, Default, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Real(pub Option<i64>);

impl Real {
    pub fn from(x: i64) -> Real {
        // TODO: Multiply with SF and check for overflow
        Real(Some(x))
    }

    pub fn abs(self) -> Real {
        if self.0.is_some() {
            Real(self.0.unwrap().checked_abs())
        } else {
            Real(None)
        }
    }
}

impl Add for Real {
    type Output = Real;

    fn add(self, rhs: Real) -> Real {
        if self.0.is_some() && rhs.0.is_some() {
            Real(self.0.unwrap().checked_add(rhs.0.unwrap()))
        } else {
            Real(None)
        }
    }
}

impl Div for Real {
    type Output = Real;

    fn div(self, rhs: Real) -> Real {
        if self.0.is_some() && rhs.0.is_some() {
            // Casting onto larger type
            let a: i128 = self.0.unwrap() as i128;
            let b: i128 = rhs.0.unwrap() as i128;

            // Checking for division by zero
            if b == 0 {
                return Real(None);
            }

            // Multiplying the dividend by the scale factor
            let mut c = a * SF;

            // Calculating the remainder
            let r = (c % b);

            // Dividing by the divisor
            c /= b;

            // Rounding away from zero
            if 2 * r >= SF {
                c += 1;
            } else if 2 * r <= -SF {
                c -= 1;
            }

            // Verifying if it over/underflows and then returning the appropriate answer
            if c < MIN || c > MAX {
                Real(None)
            } else {
                Real(Some(c as i64))
            }
        } else {
            Real(None)
        }
    }
}

impl Mul for Real {
    type Output = Real;

    fn mul(self, rhs: Real) -> Real {
        if self.0.is_some() && rhs.0.is_some() {
            // Casting onto larger type
            let a: i128 = self.0.unwrap() as i128;
            let b: i128 = rhs.0.unwrap() as i128;

            // Multiplying both numbers
            let mut c = a * b;

            // Calculating the remainder
            let r = (c % SF);

            // Dividing by the scale factor
            c /= SF;

            // Rounding away from zero
            if 2 * r >= SF {
                c += 1;
            } else if 2 * r <= -SF {
                c -= 1;
            }

            // Verifying if it over/underflows and then returning the appropriate answer
            if c < MIN || c > MAX {
                Real(None)
            } else {
                Real(Some(c as i64))
            }
        } else {
            Real(None)
        }
    }
}

impl Neg for Real {
    type Output = Real;

    fn neg(self) -> Real {
        if self.0.is_some() {
            Real(self.0.unwrap().checked_neg())
        } else {
            Real(None)
        }
    }
}

impl Sub for Real {
    type Output = Real;

    fn sub(self, rhs: Real) -> Real {
        if self.0.is_some() && rhs.0.is_some() {
            Real(self.0.unwrap().checked_sub(rhs.0.unwrap()))
        } else {
            Real(None)
        }
    }
}
