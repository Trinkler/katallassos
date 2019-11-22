// Copyright 2019 by Trinkler Software AG (Switzerland).
// This file is part of the Katal Chain.
//
// Katal Chain is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version <http://www.gnu.org/licenses/>.
//
// Katal Chain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

//! # Reals
//!
//! ## Overview
//! The Reals library implements a new data type for safe fixed-point arithmetic. It works by creating
//! a struct containing only an option of an i64 and then doing operator overloading for the most
//! common arithmetic operations (addition, subtraction, multiplication, division). It also implements
//! some convenience functions like negation, absolute value and creating from an i64. It also allows
//! comparisons by deriving the Eq and Ord traits.
//!
//! ## Fixed-point arithmetic
//! Fixed point arithmetic works similarly to normal integer arithmetic but every number is scaled
//! by the same number, which we call the scale factor. This library allows to change the scale factor
//! by simply changing the constant SF. By default it is set to 1 billion, which gives reals with 9
//! decimal points.
//! Almost all operations works equally to integer arithmetic, except for multiplication and division.
//! In multiplication and division the result of the operation needs to be rescaled and rounded. The
//! range allowed by a real (for the default SF) is [-9223372036.854775808, 9223372036.854775807],
//! which is simply the range of an i64 but rescaled.
//!
//! ## Safe arithmetic
//! This library also implements safe math. All reals are an option of an i64, so a real can have the
//! value 'None'. And all operations check for over/underflow and will return a 'None' as a result when
//! that happens. A quirk is that, when comparing two reals, 'None' is considered smaller than any
//! number.

use super::*;
use substrate_debug_derive::RuntimeDebug;

/// The scale factor (must be positive).
const SF: i128 = 1000000000;

// The maximum and minimum values supported by i64, as a i128. They are used for over/underflow
// checks in multiplication and division.
const MAX: i128 = i64::max_value() as i128;
const MIN: i128 = i64::min_value() as i128;

/// This struct implements the real data type. It is a tuple containing a single Option of
/// an i64.
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, RuntimeDebug)]
pub struct Real(pub Option<i64>);

impl Real {
    /// Transforms an i64 into a real. It scales the input by the scale factor.
    pub fn from(x: i64) -> Real {
        Real(x.checked_mul(SF as i64))
    }

    /// Transforms a real into an i64. It divides the input by the scale factor.
    /// This function does not apply safe arithmetic. Care must be had to not feed Reals
    /// that are None, otherwise this function will just return zero.
    pub fn to(x: Real) -> i64 {
        x.0.unwrap_or(0) / (SF as i64)
    }

    /// Returns the absolute value of a real. If input is 'None' (or the result
    /// overflows which is possible if the input is -2^63/SF), it returns 'None'.
    pub fn abs(self) -> Real {
        if self.0.is_some() {
            Real(self.0.unwrap().checked_abs())
        } else {
            Real(None)
        }
    }

    /// Returns the maximum of two reals. If both are equal, it will return the first real.
    /// If any of the inputs is 'None' it will return 'None'.
    pub fn max(x: Real, y: Real) -> Real {
        if x.0.is_some() && y.0.is_some() {
            if x < y {
                y
            } else {
                x
            }
        } else {
            Real(None)
        }
    }

    /// Returns the minimum of two reals. If both are equal, it will return the first real.
    /// If any of the inputs is 'None' it will return 'None'.
    pub fn min(x: Real, y: Real) -> Real {
        if x.0.is_some() && y.0.is_some() {
            if x > y {
                y
            } else {
                x
            }
        } else {
            Real(None)
        }
    }
}

/// Calculates the sum of two reals. If any of the inputs is 'None' (or the result over/underflows),
/// it returns 'None'. It does operator overloading for the symbol '+'.
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

/// Implements the addition assignment operator +=. Follows the same rules as the
/// addition operator.
impl AddAssign for Real {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

/// Calculates the division of two reals. If any of the inputs is 'None' (or the result
/// over/underflows), it returns 'None'. It does operator overloading for the symbol '/'.
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
            let r = c % b;

            // Dividing by the divisor.
            c /= b;

            // Rounding depending on the remainder. It uses the 'round half away from zero' method.
            if 2 * r.abs() >= b.abs() {
                //We can't use c.signum because c may be zero.
                c += a.signum() * b.signum();
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

/// Implements the division assignment operator /=. Follows the same rules as the
/// division operator.
impl DivAssign for Real {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

/// Calculates the multiplication of two reals. If any of the inputs is 'None' (or the result
/// over/underflows), it returns 'None'. It does operator overloading for the symbol '*'.
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
            let r = c % SF;

            // Dividing by the scale factor.
            c /= SF;

            // Rounding depending on the remainder. It uses the 'round half away from zero' method.
            if 2 * r.abs() >= SF {
                //We can't use c.signum because c may be zero.
                c += a.signum() * b.signum();
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

/// Implements the multiplication assignment operator *=. Follows the same rules as the
/// multiplication operator.
impl MulAssign for Real {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

/// Calculates the negation of a real. If the input is 'None' (or the result
/// overflows which is possible if the input is -2^63/SF), it returns 'None'.
/// It does operator overloading for the symbol '-'.
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

/// Calculates the subtraction of two reals. If any of the inputs is 'None' (or the result
/// over/underflows), it returns 'None'. It does operator overloading for the symbol '-'.
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

/// Implements the subtraction assignment operator -=. Follows the same rules as the
/// subtraction operator.
impl SubAssign for Real {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_works() {
        let x: i64 = 1;
        let s = SF as i64;
        // Checking basic case.
        assert_eq!(Real(Some(x * s)), Real::from(x));
        // Checking overflow.
        assert_eq!(Real(None), Real::from(i64::max_value()));
        // Checking underflow.
        assert_eq!(Real(None), Real::from(i64::min_value()));
    }

    #[test]
    fn to_works() {
        let x: i64 = 2;
        let s = SF as i64;
        let r = Real(Some(x * s));
        // Checking basic case.
        assert_eq!(x, Real::to(r));
    }

    #[test]
    fn abs_works() {
        // Check positive case.
        let x = Real(Some(1));
        assert_eq!(Real(Some(1)), x.abs());
        // Check negative case.
        let x = Real(Some(-1));
        assert_eq!(Real(Some(1)), x.abs());
        // Check zero case.
        let x = Real(Some(0));
        assert_eq!(Real(Some(0)), x.abs());
        // Check None case.
        let x = Real(None);
        assert_eq!(Real(None), x.abs());
        // Check overflow.
        let x = Real(Some(i64::min_value()));
        assert_eq!(Real(None), x.abs());
    }

    #[test]
    fn max_works() {
        // Check case where both are None.
        let x = Real(None);
        assert_eq!(Real(None), Real::max(x, x));
        // Check case where one is None.
        let y = Real(Some(1));
        assert_eq!(Real(None), Real::max(x, y));
        assert_eq!(Real(None), Real::max(y, x));
        // Check regular case.
        let x = Real(Some(4));
        let y = Real(Some(2));
        assert_eq!(Real(Some(4)), Real::max(x, y));
        assert_eq!(Real(Some(4)), Real::max(y, x));
        assert_eq!(Real(Some(4)), Real::max(x, x));
    }

    #[test]
    fn min_works() {
        // Check case where both are None.
        let x = Real(None);
        assert_eq!(Real(None), Real::min(x, x));
        // Check case where one is None.
        let y = Real(Some(1));
        assert_eq!(Real(None), Real::min(x, y));
        assert_eq!(Real(None), Real::min(y, x));
        // Check regular case.
        let x = Real(Some(4));
        let y = Real(Some(2));
        assert_eq!(Real(Some(2)), Real::min(x, y));
        assert_eq!(Real(Some(2)), Real::min(y, x));
        assert_eq!(Real(Some(2)), Real::min(y, y));
    }

    #[test]
    fn neg_works() {
        // Check positive case.
        let x = Real(Some(1));
        assert_eq!(Real(Some(-1)), -x);
        // Check negative case.
        let x = Real(Some(-1));
        assert_eq!(Real(Some(1)), -x);
        // Check zero case.
        let x = Real(Some(0));
        assert_eq!(Real(Some(0)), -x);
        // Check None case.
        let x = Real(None);
        assert_eq!(Real(None), -x);
        // Check overflow.
        let x = Real(Some(i64::min_value()));
        assert_eq!(Real(None), -x);
    }

    #[test]
    fn add_works() {
        // Check case where both are None.
        let x = Real(None);
        let y = Real(None);
        assert_eq!(Real(None), x + y);
        // Check case where one is None.
        let y = Real(Some(1));
        assert_eq!(Real(None), x + y);
        assert_eq!(Real(None), y + x);
        // Check regular case.
        let x = Real(Some(2));
        let y = Real(Some(2));
        assert_eq!(Real(Some(4)), x + y);
        // Check chaining of different cases.
        let x = Real(Some(3));
        let y = Real(Some(0));
        let z = Real(Some(-1));
        assert_eq!(Real(Some(2)), x + y + z);
        // Check overflow and underflow.
        let x = Real(Some(10));
        let y = Real(Some(-10));
        assert_eq!(Real(None), x + Real(Some(i64::max_value())));
        assert_eq!(Real(None), y + Real(Some(i64::min_value())));
    }

    #[test]
    fn sub_works() {
        // Check case where both are None.
        let x = Real(None);
        let y = Real(None);
        assert_eq!(Real(None), x - y);
        // Check case where one is None.
        let y = Real(Some(1));
        assert_eq!(Real(None), x - y);
        assert_eq!(Real(None), y - x);
        // Check regular case.
        let x = Real(Some(4));
        let y = Real(Some(2));
        assert_eq!(Real(Some(2)), x - y);
        // Check chaining of different cases.
        let x = Real(Some(2));
        let y = Real(Some(0));
        let z = Real(Some(-1));
        assert_eq!(Real(Some(3)), x - y - z);
        // Check overflow and underflow.
        let x = Real(Some(10));
        let y = Real(Some(-10));
        assert_eq!(Real(None), x - Real(Some(i64::min_value())));
        assert_eq!(Real(None), y - Real(Some(i64::max_value())));
    }

    #[test]
    fn mul_works() {
        let s = SF as i64;
        // Check case where both are None.
        let x = Real(None);
        let y = Real(None);
        assert_eq!(Real(None), x * y);
        // Check case where one is None.
        let y = Real(Some(5));
        assert_eq!(Real(None), x * y);
        assert_eq!(Real(None), y * x);
        // Check regular case.
        let x = Real(Some(2 * s));
        let y = Real(Some(2 * s));
        assert_eq!(Real(Some(4 * s)), x * y);
        // Check chaining of different cases.
        let x = Real(Some(3 * s));
        let y = Real(Some(0 * s));
        let z = Real(Some(-1 * s));
        assert_eq!(Real(Some(0)), x * y * z);
        // Check the rounding.
        let w = Real(Some(-1));
        let x = Real(Some(1));
        let y = Real(Some(s / 2));
        let z = Real(Some(s / 4));
        assert_eq!(Real(Some(1)), x * y);
        assert_eq!(Real(Some(0)), x * z);
        assert_eq!(Real(Some(-1)), w * y);
        assert_eq!(Real(Some(0)), w * z);
        // Check overflow and underflow.
        let x = Real(Some(2 * s));
        assert_eq!(Real(None), x * Real(Some(i64::max_value())));
        assert_eq!(Real(None), x * Real(Some(i64::min_value())));
    }

    #[test]
    fn div_works() {
        let s = SF as i64;
        // Check case where both are None.
        let x = Real(None);
        let y = Real(None);
        assert_eq!(Real(None), x / y);
        // Check case where one is None.
        let y = Real(Some(5));
        assert_eq!(Real(None), x / y);
        assert_eq!(Real(None), y / x);
        // Check division by zero.
        let x = Real(Some(2 * s));
        let y = Real(Some(0));
        assert_eq!(Real(None), x / y);
        // Check regular case.
        let x = Real(Some(4 * s));
        let y = Real(Some(2 * s));
        assert_eq!(Real(Some(2 * s)), x / y);
        // Check chaining of different cases.
        let x = Real(Some(12 * s));
        let y = Real(Some(3 * s));
        let z = Real(Some(-2 * s));
        assert_eq!(Real(Some(-2 * s)), x / y / z);
        // Check the rounding.
        let w = Real(Some(-1));
        let x = Real(Some(1));
        let y = Real(Some(2 * s));
        let z = Real(Some(3 * s));
        assert_eq!(Real(Some(1)), x / y);
        assert_eq!(Real(Some(0)), x / z);
        assert_eq!(Real(Some(-1)), w / y);
        assert_eq!(Real(Some(0)), w / z);
        // Check overflow and underflow.
        let x = Real(Some(s / 10));
        assert_eq!(Real(None), Real(Some(i64::max_value())) / x);
        assert_eq!(Real(None), Real(Some(i64::min_value())) / x);
    }
}
