use super::*;

/// Contract Default Convention:it maps a given contract status to either a 1 or a 0, representing
/// a performant or a defaulted contract. See section 4.8 of the ACTUS paper for details.
pub fn contract_default(contract_status: Option<ContractStatus>) -> Real {
    match contract_status {
        Some(ContractStatus::PF) => Real::from(1),
        Some(ContractStatus::DL) => Real::from(1),
        Some(ContractStatus::DQ) => Real::from(1),
        Some(ContractStatus::DF) => Real::from(0),
        None => Real(None),
    }
}
