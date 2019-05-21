// The Reals library implements a new data type for safe fixed-point arithmetic. It works by creating
// a struct containing only an option of an i64 and then doing operator overloading for the most
// common arithmetic operations (addition, subtraction, multiplication, division). It also implements
// some convenience functions like negation, absolute value and creating from an i64. It also allows
// comparisons by deriving the Eq and Ord traits.
//
// Fixed point arithmetic works similarly to normal integer arithmetic but every number is scaled
// by the same number, which we call the scale factor. This library allows to change the scale factor
// by simply changing the constant SF. By default it is set to 1 billion, which gives reals with 9
// decimal points.
// Almost all operations works equally to integer arithmetic, except for multiplication and division.
// In multiplication and division the result of the operation needs to be rescaled and rounded. The
// range allowed by a real (for the default SF) is [-9223372036.854775808, 9223372036.854775807],
// which is simply the range of an i64 but rescaled.
//
// This library also implements safe math. All reals are an option of an i64, so a real can have the
// value 'None'. And all operations check for over/underflow and will return a 'None' as a result when
// that happens. A quirk is that, when comparing two reals, 'None' is considered smaller than any
// number.

// These are necessary to work with Substrate.
use parity_codec::{Decode, Encode};
// These are necessary to do operator overloading.
use std::ops::{Add, Div, Mul, Neg, Sub};

// The scale factor.
const SF: i128 = 1000000000;

// The maximum and minimum values supported by i64, as a i128. They are used for over/underflow
// checks in multiplication and division.
const MAX: i128 = i64::max_value() as i128;
const MIN: i128 = i64::min_value() as i128;

// This creates the Real data type and derives several traits for it.
#[derive(Decode, Encode, Default, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Real(pub Option<i64>);

impl Real {
    // Transforms an i64 into a real. It scales the input by the scale factor.
    pub fn from(x: i64) -> Real {
        Real(x.checked_mul(SF as i64))
    }

    // Returns the absolute value of a real. If input is 'None', it returns 'None'.
    pub fn abs(self) -> Real {
        if self.0.is_some() {
            Real(self.0.unwrap().checked_abs())
        } else {
            Real(None)
        }
    }
}

// Calculates the sum of two reals. If any of the inputs is 'None' (or the result over/underflows),
// it returns 'None'. It does operator overloading for the symbol '+'.
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

// Calculates the division of two reals. If any of the inputs is 'None' (or the result
// over/underflows), it returns 'None'. It does operator overloading for the symbol '/'.
impl Div for Real {
    type Output = Real;

    fn div(self, rhs: Real) -> Real {
        if self.0.is_some() && rhs.0.is_some() {
            // Casting onto larger type to prevent overflow in the intermediate calculations.
            let a: i128 = self.0.unwrap() as i128;
            let b: i128 = rhs.0.unwrap() as i128;

            // Checking for division by zero.
            if b == 0 {
                return Real(None);
            }

            // Multiplying the dividend by the scale factor.
            let mut c = a * SF;

            // Calculating the remainder.
            let r = (c % b);

            // Dividing by the divisor.
            c /= b;

            // Rounding depending on the remainder. It uses the 'round half away from zero' method.
            if 2 * r >= SF {
                c += 1;
            } else if 2 * r <= -SF {
                c -= 1;
            }

            // Verifying if it over/underflows and then returning the appropriate answer.
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

// Calculates the multiplication of two reals. If any of the inputs is 'None' (or the result
// over/underflows), it returns 'None'. It does operator overloading for the symbol '*'.
impl Mul for Real {
    type Output = Real;

    fn mul(self, rhs: Real) -> Real {
        if self.0.is_some() && rhs.0.is_some() {
            // Casting onto larger type to prevent overflow in the intermediate calculations.
            let a: i128 = self.0.unwrap() as i128;
            let b: i128 = rhs.0.unwrap() as i128;

            // Multiplying both numbers.
            let mut c = a * b;

            // Calculating the remainder.
            let r = (c % SF);

            // Dividing by the scale factor.
            c /= SF;

            // Rounding depending on the remainder. It uses the 'round half away from zero' method.
            if 2 * r >= SF {
                c += 1;
            } else if 2 * r <= -SF {
                c -= 1;
            }

            // Verifying if it over/underflows and then returning the appropriate answer.
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

// Calculates the negation of a real. If the input is 'None' (or the result
// overflows which is possible if the input is -2^63/SF), it returns 'None'.
// It does operator overloading for the symbol '-'.
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

// Calculates the subtraction of two reals. If any of the inputs is 'None' (or the result
// over/underflows), it returns 'None'. It does operator overloading for the symbol '-'.
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
