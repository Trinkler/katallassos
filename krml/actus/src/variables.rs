use super::*;

// All ACTUS contract variables as specifed in the ACTUS paper.
#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Variables {
    fee_accrued: Real,
    interest_calculation_base: Real,
    interest_scaling_multiplier: Real,
    last_event_date: Time,
    next_principal_redemption_payment: Real,
    nominal_accrued_1: Real,
    nominal_accrued_2: Real,
    nominal_rate: Real,
    nominal_value_1: Real,
    nominal_value_2: Real,
    notional_scaling_multiplier: Real,
    payoff_at_settlement: Real,
    performance: Option<ContractStatus>,
    time_at_maturity_date: Time,
}
