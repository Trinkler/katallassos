use super::*;

/// Contract Role Sign Convention: it maps a given contract role to either a 1 or a -1, representing
/// a direction for cashflows. See section 4.7 of the ACTUS paper for details.
pub fn contract_role_sign(contract_role: ContractRole) -> Real {
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
