use super::*;

/// All ACTUS contract variables as specifed in the ACTUS paper.
#[derive(Clone, Copy, Decode, Encode, Default, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Variables {
    pub fee_accrued: Real,
    pub interest_calculation_base: Real,
    pub interest_scaling_multiplier: Real,
    pub last_event_date: Time,
    pub next_principal_redemption_payment: Real,
    pub nominal_accrued_1: Real,
    pub nominal_accrued_2: Real,
    pub nominal_rate: Real,
    pub nominal_value_1: Real,
    pub nominal_value_2: Real,
    pub notional_scaling_multiplier: Real,
    pub payoff_at_settlement: Real,
    pub performance: Option<ContractStatus>,
    pub time_at_maturity_date: Time,
}

impl Variables {
    pub fn new() -> Variables {
        Variables {
            fee_accrued: Real(None),
            interest_calculation_base: Real(None),
            interest_scaling_multiplier: Real(None),
            last_event_date: Time(None),
            next_principal_redemption_payment: Real(None),
            nominal_accrued_1: Real(None),
            nominal_accrued_2: Real(None),
            nominal_rate: Real(None),
            nominal_value_1: Real(None),
            nominal_value_2: Real(None),
            notional_scaling_multiplier: Real(None),
            payoff_at_settlement: Real(None),
            performance: None,
            time_at_maturity_date: Time(None),
        }
    }
}
