use super::attributes::ContractStatus;
use super::*;

// All ACTUS contract variables as specifed in the ACTUS paper.
#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Variables {
    Performance: Option<ContractStatus>,
    LastEventDate: Real,
    NominalValue1: Real,
    NominalValue2: Real,
    NominalRate: Real,
    NominalAccrued1: Real,
    NominalAccrued2: Real,
    InterestCalculationBase: Real,
    NotionalScalingMultiplier: Real,
    InterestScalingMultiplier: Real,
    NextPrincipalRedemptionPayment: Real,
    PayoffAtSettlement: Real,
    TimeAtMaturityDate: Real,
    FeeAccrued: Real,
}
