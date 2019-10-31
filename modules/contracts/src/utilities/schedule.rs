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

use super::*;

/// Schedule: a function mapping two different times *s* and *t*, with *s<t*, and a cycle *c* onto a
/// sequence of times. See section 4.1 of the ACTUS paper for details.
pub fn schedule(
    s: Time,
    t: Time,
    cycle: Option<Cycle>,
    end_of_month_convention: Option<EndOfMonthConvention>,
) -> MyResult<Vec<Time>> {
    // Checking some assumptions about the inputs.
    if s != Time(None) && t != Time(None) && s >= t {
        return Err("Couldn't create schedule");
    }
    if s == Time(None) && t == Time(None) {
        return Err("Couldn't create schedule");
    }

    // Checking two specific cases of the schedule function.
    let mut vec: Vec<Time> = Vec::new();
    if t == Time(None) {
        vec.push(s);
        return Ok(vec);
    }
    if cycle == None {
        vec.push(s);
        vec.push(t);
        return Ok(vec);
    }

    // Checking the main case of the schedule function.
    let mut unchecked_s = s.0.unwrap();
    let unchecked_t = t.0.unwrap();
    let cycle = cycle.unwrap();
    let end_of_month_convention = end_of_month_convention.unwrap_or(EndOfMonthConvention::SD);

    match cycle {
        Cycle::Days(int, stub) => {
            if int == 0 {
                return Err("Couldn't create schedule");
            }
            vec.push(s);
            let mut x = s;
            while x != Time(None) && x < t {
                x = Time::add_days(x, int);
                vec.push(x);
            }
            if x > t && stub {
                vec.pop();
            }
        }
        Cycle::Months(int, stub) => {
            if int == 0 {
                return Err("Couldn't create schedule");
            }
            vec.push(s);
            while unchecked_s < unchecked_t {
                unchecked_s.year += (unchecked_s.month as u16 + int) / 12;
                unchecked_s.month = ((unchecked_s.month as u16 - 1 + int) % 12 + 1) as i8;
                vec.push(Time::from_unchecked(end_of_month_shift(
                    unchecked_s,
                    end_of_month_convention,
                )));
            }
            if unchecked_s > unchecked_t && stub {
                vec.pop();
            }
        }
        Cycle::Years(int, stub) => {
            if int == 0 {
                return Err("Couldn't create schedule");
            }
            vec.push(s);
            while unchecked_s < unchecked_t {
                unchecked_s.year += int;
                vec.push(Time::from_unchecked(end_of_month_shift(
                    unchecked_s,
                    end_of_month_convention,
                )));
            }
            if unchecked_s > unchecked_t && stub {
                vec.pop();
            }
        }
    }

    Ok(vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schedule_works() {
        // Testing s==None && t==None.
        assert!(schedule(Time(None), Time(None), None, None).is_err());

        // Testing t>s.
        let s = Time::from_values(2019, 06, 01, 12, 00, 00);
        let t = Time::from_values(2019, 06, 15, 12, 00, 00);
        assert!(schedule(t, s, None, None).is_err());

        // Testing t==None.
        let mut vec: Vec<Time> = Vec::new();
        vec.push(s);
        assert_eq!(schedule(s, Time(None), None, None), Ok(vec.clone()));

        // Testing cycle==None.
        vec.push(t);
        assert_eq!(schedule(s, t, None, None), Ok(vec.clone()));

        // Testing Cycle::Days==0.
        let c = Some(Cycle::Days(0, true));
        assert!(schedule(s, t, c, None).is_err());

        // Testing Cycle::Days with long stub.
        let mut vec: Vec<Time> = Vec::new();
        let c = Some(Cycle::Days(5, true));
        vec.push(s);
        vec.push(Time::from_values(2019, 06, 06, 12, 00, 00));
        vec.push(Time::from_values(2019, 06, 11, 12, 00, 00));
        assert_eq!(schedule(s, t, c, None), Ok(vec.clone()));

        // Testing Cycle::Days with short stub.
        let c = Some(Cycle::Days(5, false));
        vec.push(Time::from_values(2019, 06, 16, 12, 00, 00));
        assert_eq!(schedule(s, t, c, None), Ok(vec.clone()));

        // Testing Cycle::Months==0.
        let t = Time::from_values(2020, 06, 01, 12, 00, 00);
        let c = Some(Cycle::Months(0, true));
        assert!(schedule(s, t, c, None).is_err());

        // Testing Cycle::Months with long stub.
        let mut vec: Vec<Time> = Vec::new();
        let c = Some(Cycle::Months(5, true));
        vec.push(s);
        vec.push(Time::from_values(2019, 11, 01, 12, 00, 00));
        vec.push(Time::from_values(2020, 04, 01, 12, 00, 00));
        assert_eq!(schedule(s, t, c, None), Ok(vec.clone()));

        // Testing Cycle::Months with short stub.
        let c = Some(Cycle::Months(5, false));
        vec.push(Time::from_values(2020, 09, 01, 12, 00, 00));
        assert_eq!(schedule(s, t, c, None), Ok(vec.clone()));

        // Testing Cycle::Years==0.
        let t = Time::from_values(2030, 06, 01, 12, 00, 00);
        let c = Some(Cycle::Years(0, true));
        assert!(schedule(s, t, c, None).is_err());

        // Testing Cycle::Years with long stub.
        let mut vec: Vec<Time> = Vec::new();
        let c = Some(Cycle::Years(5, true));
        vec.push(s);
        vec.push(Time::from_values(2024, 06, 01, 12, 00, 00));
        vec.push(Time::from_values(2029, 06, 01, 12, 00, 00));
        assert_eq!(schedule(s, t, c, None), Ok(vec.clone()));

        // Testing Cycle::Years with short stub.
        let c = Some(Cycle::Years(5, false));
        vec.push(Time::from_values(2034, 06, 01, 12, 00, 00));
        assert_eq!(schedule(s, t, c, None), Ok(vec.clone()));
    }
}
