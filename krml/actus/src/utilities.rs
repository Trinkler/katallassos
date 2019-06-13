use super::*;

// Time utility, convert from unix time to ISO8601, ignores leap seconds
// This assumes both that time fits in a real and that it is positive (so after the unix epoch)
#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Iso8601 {
    Year: u16,
    Month: u16,
    Day: u16,
    Hour: u16,
    Minute: u16,
    Second: u16,
}

fn unix_to_iso8601(time: Real) -> Iso8601 {
    const SECONDS_IN_A_MINUTE: i64 = 60;
    const SECONDS_IN_A_HOUR: i64 = 3600;
    const SECONDS_IN_A_DAY: i64 = 86400;
    const SECONDS_IN_A_YEAR: i64 = 31536000;
    let mut unix_time = Real::to(time);
    let mut iso_time = Iso8601 {
        Year: 1970,
        Month: 01,
        Day: 01,
        Hour: 00,
        Minute: 00,
        Second: 00,
    };
    while unix_time >= SECONDS_IN_A_YEAR {
        iso_time.Year += 1;
        unix_time -= SECONDS_IN_A_YEAR;
        if is_leap_year(iso_time.Year) {
            unix_time -= SECONDS_IN_A_DAY;
        }
    }
    let mut secs = seconds_in_month(iso_time.Year, iso_time.Month);
    while unix_time >= secs {
        iso_time.Month += 1;
        unix_time -= secs;
        secs = seconds_in_month(iso_time.Year, iso_time.Month);
    }
    // days
    iso_time.Day += (unix_time / SECONDS_IN_A_DAY) as u16;
    unix_time %= SECONDS_IN_A_DAY;
    // hours
    iso_time.Hour += (unix_time / SECONDS_IN_A_HOUR) as u16;
    unix_time %= SECONDS_IN_A_HOUR;
    // minutes
    iso_time.Minute += (unix_time / SECONDS_IN_A_MINUTE) as u16;
    unix_time %= SECONDS_IN_A_MINUTE;
    // seconds
    iso_time.Second += unix_time as u16;
    // return
    iso_time
}

fn is_leap_year(year: u16) -> bool {
    if year % 4 != 0 {
        return false;
    }
    if year % 100 != 0 {
        return true;
    }
    if year % 400 != 0 {
        return false;
    }
    return true;
}

fn seconds_in_month(year: u16, month: u16) -> i64 {
    if month == 1
        || month == 3
        || month == 5
        || month == 7
        || month == 8
        || month == 10
        || month == 12
    {
        return 2678400;
    } else if month == 4 || month == 6 || month == 9 || month == 11 {
        return 2592000;
    } else if is_leap_year(year) {
        return 2505600;
    } else {
        return 2419200;
    }
}

/// All ACTUS utility functions
/// Contract Role Sign Convention
fn contract_role_sign(contract_role: &ContractRole) -> Real {
    match contract_role {
        ContractRole::RPA => Real::from(1),
        ContractRole::RPL => Real::from(-1),
        ContractRole::CLO => Real::from(1),
        ContractRole::CNO => Real::from(1),
        ContractRole::COL => Real::from(1),
        ContractRole::LG => Real::from(1),
        ContractRole::ST => Real::from(-1),
        ContractRole::BUY => Real::from(1),
        ContractRole::SEL => Real::from(-1),
        ContractRole::RFL => Real::from(1),
        ContractRole::PFL => Real::from(-1),
        ContractRole::RF => Real::from(1),
        ContractRole::PF => Real::from(-1),
    }
}

// Contract Default Convention
fn contract_default(contract_status: &ContractStatus) -> Real {
    match contract_status {
        ContractStatus::PF => Real::from(1),
        ContractStatus::DL => Real::from(1),
        ContractStatus::DQ => Real::from(1),
        ContractStatus::DF => Real::from(0),
    }
}

// Year Fraction Convention (https://en.wikipedia.org/wiki/Day_count_convention)
fn utility_function_Y(s: Real, t: Real, day_cont_convention: &DayCountConvention) -> Real {
    match day_cont_convention {
        DayCountConvention::_AAISDA => Real::from(1),
        DayCountConvention::_A360 => Real::from(1),
        DayCountConvention::_A365 => Real::from(1),
        DayCountConvention::_30E360ISDA => Real::from(1),
        DayCountConvention::_30E360 => Real::from(1),
        DayCountConvention::_30360 => Real::from(1),
        DayCountConvention::_BUS252 => Real::from(1),
    }
}
