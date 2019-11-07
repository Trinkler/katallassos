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

/// Array Schedule: a generalization of a regular schedule. It allows the creation of a schedule
/// that consists of several diferent schedules with different cycles pieced together. See section
/// 4.2 of the ACTUS paper for details.
pub fn array_schedule(
    arr_s: Vec<Time>,
    t: Time,
    arr_cycle: Vec<Option<Cycle>>,
    end_of_month_convention: Option<EndOfMonthConvention>,
) -> MyResult<Vec<Time>> {
    if arr_s.len() != arr_cycle.len() {
        return Err("Couldn't create array schedule");
    }
    if arr_s.len() < 2 {
        return Err("Couldn't create array schedule");
    }

    let mut vec: Vec<Time> = Vec::new();
    let mut vec_2: Vec<Time> = Vec::new();
    for i in 0..(m - 1) {
        vec_2 = schedule(
            arr_s[i],
            arr_s[i + 1],
            arr_cycle[i],
            end_of_month_convention,
        )?;
        vec_2.pop();
        vec.append(&mut vec_2);
    }

    vec_2 = schedule(arr_s[m - 1], t, arr_cycle[m - 1], end_of_month_convention)?;
    vec.append(&mut vec_2);

    Ok(vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_schedule_works() {
        let t = Time::from_values(2021, 08, 03, 12, 00, 00);
        let mut arr_s: Vec<Time> = Vec::new();
        let mut arr_c: Vec<Option<Cycle>> = Vec::new();

        // Testing different sizes of vectors.
        arr_s.push(t);
        assert!(array_schedule(arr_s, t, arr_c, None).is_err());

        // Testing an arbitrary schedule.
        let s1 = Time::from_values(2019, 06, 01, 12, 00, 00);
        let c1 = Some(Cycle::Days(1, true));
        let s2 = Time::from_values(2019, 06, 03, 12, 00, 00);
        let c2 = Some(Cycle::Months(1, true));
        let s3 = Time::from_values(2019, 08, 03, 12, 00, 00);
        let c3 = Some(Cycle::Years(1, true));
        let mut arr_s = vec![s1, s2, s3];
        let mut arr_c = vec![c1, c2, c3];
        let mut vec: Vec<Time> = Vec::new();
        vec.push(s1);
        vec.push(Time::from_values(2019, 06, 02, 12, 00, 00));
        vec.push(s2);
        vec.push(Time::from_values(2019, 07, 03, 12, 00, 00));
        vec.push(s3);
        vec.push(Time::from_values(2020, 08, 03, 12, 00, 00));
        vec.push(t);
        assert_eq!(array_schedule(arr_s, t, arr_c, None), Ok(vec.clone()));
    }
}
