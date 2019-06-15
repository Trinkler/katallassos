use super::*;

// All ACTUS contract variables as specifed in the ACTUS paper.
#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Variables {
    FeeAccrued: Real,
    InterestCalculationBase: Real,
    InterestScalingMultiplier: Real,
    LastEventDate: Time,
    NextPrincipalRedemptionPayment: Real,
    NominalAccrued1: Real,
    NominalAccrued2: Real,
    NominalRate: Real,
    NominalValue1: Real,
    NominalValue2: Real,
    NotionalScalingMultiplier: Real,
    PayoffAtSettlement: Real,
    Performance: Option<ContractStatus>,
    TimeAtMaturityDate: Time,
}
