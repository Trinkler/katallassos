use super::*;

// All ACTUS utility functions, bla, bla, bla

// Contract Role Sign Convention
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
        DayCountConvention::_AAISDA => 1,
        DayCountConvention::_A360 => 1,
        DayCountConvention::_A365 => 1,
        DayCountConvention::_30E360ISDA => 1,
        DayCountConvention::_30E360 => 1,
        DayCountConvention::_30360 => 1,
        DayCountConvention::_BUS252 => 1,
    }
}
