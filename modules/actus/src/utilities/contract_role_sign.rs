use super::*;

/// Contract Role Sign Convention: it maps a given contract role to either a 1 or a -1, representing
/// a direction for cashflows. See section 4.7 of the ACTUS paper for details.
pub fn contract_role_sign(contract_role: Option<ContractRole>) -> Real {
    match contract_role {
        Some(ContractRole::RPA) => Real::from(1),
        Some(ContractRole::RPL) => Real::from(-1),
        Some(ContractRole::CLO) => Real::from(1),
        Some(ContractRole::CNO) => Real::from(1),
        Some(ContractRole::COL) => Real::from(1),
        Some(ContractRole::LG) => Real::from(1),
        Some(ContractRole::ST) => Real::from(-1),
        Some(ContractRole::BUY) => Real::from(1),
        Some(ContractRole::SEL) => Real::from(-1),
        Some(ContractRole::RFL) => Real::from(1),
        Some(ContractRole::PFL) => Real::from(-1),
        Some(ContractRole::RF) => Real::from(1),
        Some(ContractRole::PF) => Real::from(-1),
        None => Real(None),
    }
}
