use super::*;

// TODO: Add support for user-initiated events.
// TODO: Add the calls to transfer tokens (assets module?).
impl<T: Trait> Module<T> {
    pub fn progress_pam(
        event: ContractEvent,
        mut state: ContractState,
    ) -> MyResult<(ContractState, Real)> {
        // Getting t0 from the status_date attribute since they are equal.
        // (And status_date is not supposed to change)
        let t0 = state.attributes.status_date;

        match event.event_type {
            ContractEventType::IED => {
                // Payoff Function
                let payoff = utilities::contract_default(state.variables.performance)
                    * utilities::contract_role_sign(state.attributes.contract_role)
                    * Real::from(-1)
                    * (state.attributes.notional_principal
                        + state.attributes.premium_discount_at_ied);

                // State Transition Function
                state.variables.nominal_value_1 =
                    utilities::contract_role_sign(state.attributes.contract_role)
                        * state.attributes.notional_principal;

                if state.attributes.nominal_interest_rate == Real(None) {
                    state.variables.nominal_rate = Real::from(0);
                } else {
                    state.variables.nominal_rate = state.attributes.nominal_interest_rate;
                }

                if state.attributes.accrued_interest != Real(None) {
                    state.variables.nominal_accrued_1 = state.attributes.accrued_interest;
                } else if state.attributes.cycle_anchor_date_of_interest_payment != Time(None)
                    && state.attributes.cycle_anchor_date_of_interest_payment < event.time
                {
                    let y = utilities::year_fraction(
                        state.attributes.cycle_anchor_date_of_interest_payment,
                        event.time,
                        state.attributes.day_count_convention.unwrap(), // Unwraping poses no danger since day_count_convention is mandatory for the PAM contract. It will never panic.
                    );
                    state.variables.nominal_accrued_1 =
                        y * state.variables.nominal_value_1 * state.variables.nominal_rate;
                } else {
                    state.variables.nominal_accrued_1 = Real::from(0);
                }

                state.variables.last_event_date = event.time;

                // Return the contract state and payoff
                Ok((state, payoff))
            }
            ContractEventType::PR => {
                // Payoff Function
                let payoff = utilities::contract_default(state.variables.performance)
                    * state.variables.notional_scaling_multiplier
                    * state.variables.nominal_value_1;

                // State Transition Function
                state.variables.nominal_value_1 = Real::from(0);

                state.variables.nominal_rate = Real::from(0);

                state.variables.last_event_date = event.time;

                // Return the contract state and payoff
                Ok((state, payoff))
            }
            ContractEventType::PP => {
                // Payoff Function
                // TODO: Add the user-initiated events based on the "OPMO".
                let payoff = utilities::contract_default(state.variables.performance);

                // State Transition Function
                state.variables.nominal_accrued_1 = state.variables.nominal_accrued_1
                    + utilities::year_fraction(
                        state.variables.last_event_date,
                        event.time,
                        state.attributes.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) * state.variables.nominal_rate
                        * state.variables.nominal_value_1;

                if state.attributes.fee_basis == Some(FeeBasis::N) {
                    state.variables.fee_accrued = state.variables.fee_accrued
                        + utilities::year_fraction(
                            state.variables.last_event_date,
                            event.time,
                            state.attributes.day_count_convention.unwrap(), // This unwrap will never panic.
                        ) * state.variables.nominal_value_1
                            * state.attributes.fee_rate;
                } else {
                    let mut t_minus = Time(None);
                    let mut t_plus = Time(None);
                    for e in state.schedule.clone() {
                        if e.event_type == ContractEventType::FP {
                            if e.time >= t0 {
                                t_plus = e.time;
                                break;
                            }
                            t_minus = e.time;
                        }
                    }
                    state.variables.fee_accrued = utilities::year_fraction(
                        t_minus,
                        event.time,
                        state.attributes.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) / year_fraction(
                        t_minus,
                        t_plus,
                        state.attributes.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) * state.attributes.fee_rate;
                }

                // TODO: Add the user-initiated events based on the "OPMO".
                state.variables.nominal_value_1 = state.variables.nominal_value_1;

                state.variables.last_event_date = event.time;

                // Return the contract state and payoff
                Ok((state, payoff))
            }
            ContractEventType::PY => {
                // Payoff Function
                let mut payoff = Real::from(0);
                if state.attributes.penalty_type == Some(PenaltyType::A) {
                    payoff = utilities::contract_default(state.variables.performance)
                        * utilities::contract_role_sign(state.attributes.contract_role)
                        * state.attributes.penalty_rate;
                }
                if state.attributes.penalty_type == Some(PenaltyType::N) {
                    payoff = utilities::contract_default(state.variables.performance)
                        * utilities::contract_role_sign(state.attributes.contract_role)
                        * utilities::year_fraction(
                            state.variables.last_event_date,
                            event.time,
                            state.attributes.day_count_convention.unwrap(), // This unwrap will never panic.
                        )
                        * state.variables.nominal_value_1
                        * state.attributes.penalty_rate;
                }
                if state.attributes.penalty_type == Some(PenaltyType::I) {
                    payoff = utilities::contract_default(state.variables.performance)
                        * utilities::contract_role_sign(state.attributes.contract_role)
                        * utilities::year_fraction(
                            state.variables.last_event_date,
                            event.time,
                            state.attributes.day_count_convention.unwrap(), // This unwrap will never panic.
                        )
                        * state.variables.nominal_value_1
                        * Real::max(
                            Real::from(0),
                            state.variables.nominal_rate
                                - <oracle::Oracles<T>>::get(
                                    state.attributes.market_object_code_rate_reset.unwrap(), //This unwrap will never panic.
                                )
                                .value,
                        );
                }

                // State Transition Function
                state.variables.nominal_accrued_1 = state.variables.nominal_accrued_1
                    + utilities::year_fraction(
                        state.variables.last_event_date,
                        event.time,
                        state.attributes.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) * state.variables.nominal_rate
                        * state.variables.nominal_value_1;

                if state.attributes.fee_basis == Some(FeeBasis::N) {
                    state.variables.fee_accrued = state.variables.fee_accrued
                        + utilities::year_fraction(
                            state.variables.last_event_date,
                            event.time,
                            state.attributes.day_count_convention.unwrap(), // This unwrap will never panic.
                        ) * state.variables.nominal_value_1
                            * state.attributes.fee_rate;
                } else {
                    let mut t_minus = Time(None);
                    let mut t_plus = Time(None);
                    for e in state.schedule.clone() {
                        if e.event_type == ContractEventType::FP {
                            if e.time >= t0 {
                                t_plus = e.time;
                                break;
                            }
                            t_minus = e.time;
                        }
                    }
                    state.variables.fee_accrued = utilities::year_fraction(
                        t_minus,
                        event.time,
                        state.attributes.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) / year_fraction(
                        t_minus,
                        t_plus,
                        state.attributes.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) * state.attributes.fee_rate;
                }

                state.variables.last_event_date = event.time;

                // Return the contract state and payoff
                Ok((state, payoff))
            }
            ContractEventType::FP => {
                // Payoff Function
                let mut payoff = Real::from(0);
                if state.attributes.fee_basis == Some(FeeBasis::A) {
                    payoff = utilities::contract_default(state.variables.performance)
                        * utilities::contract_role_sign(state.attributes.contract_role)
                        * state.attributes.fee_rate;
                }
                if state.attributes.fee_basis == Some(FeeBasis::N) {
                    payoff = utilities::contract_default(state.variables.performance)
                        * utilities::contract_role_sign(state.attributes.contract_role)
                        * state.attributes.fee_rate
                        * utilities::year_fraction(
                            state.variables.last_event_date,
                            event.time,
                            state.attributes.day_count_convention.unwrap(), // This unwrap will never panic.
                        )
                        * state.variables.nominal_value_1
                        + state.variables.fee_accrued;
                }

                // State Transition Function
                state.variables.nominal_accrued_1 = state.variables.nominal_accrued_1
                    + utilities::year_fraction(
                        state.variables.last_event_date,
                        event.time,
                        state.attributes.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) * state.variables.nominal_rate
                        * state.variables.nominal_value_1;

                state.variables.fee_accrued = Real::from(0);

                state.variables.last_event_date = event.time;

                // Return the contract state and payoff
                Ok((state, payoff))
            }
            ContractEventType::PRD => {
                // Payoff Function
                let payoff = utilities::contract_default(state.variables.performance)
                    * utilities::contract_role_sign(state.attributes.contract_role)
                    * Real::from(-1)
                    * (state.attributes.price_at_purchase_date
                        + state.variables.nominal_accrued_1
                        + utilities::year_fraction(
                            state.variables.last_event_date,
                            event.time,
                            state.attributes.day_count_convention.unwrap(), // This unwrap will never panic.
                        ) * state.variables.nominal_rate
                            * state.variables.nominal_value_1);

                // State Transition Function
                state.variables.nominal_accrued_1 = state.variables.nominal_accrued_1
                    + utilities::year_fraction(
                        state.variables.last_event_date,
                        event.time,
                        state.attributes.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) * state.variables.nominal_rate
                        * state.variables.nominal_value_1;

                if state.attributes.fee_basis == Some(FeeBasis::N) {
                    state.variables.fee_accrued = state.variables.fee_accrued
                        + utilities::year_fraction(
                            state.variables.last_event_date,
                            event.time,
                            state.attributes.day_count_convention.unwrap(), // This unwrap will never panic.
                        ) * state.variables.nominal_value_1
                            * state.attributes.fee_rate;
                } else {
                    let mut t_minus = Time(None);
                    let mut t_plus = Time(None);
                    for e in state.schedule.clone() {
                        if e.event_type == ContractEventType::FP {
                            if e.time >= t0 {
                                t_plus = e.time;
                                break;
                            }
                            t_minus = e.time;
                        }
                    }
                    state.variables.fee_accrued = utilities::year_fraction(
                        t_minus,
                        event.time,
                        state.attributes.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) / year_fraction(
                        t_minus,
                        t_plus,
                        state.attributes.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) * state.attributes.fee_rate;
                }

                state.variables.last_event_date = event.time;

                // Return the contract state and payoff
                Ok((state, payoff))
            }
            ContractEventType::TD => {
                // Payoff Function
                let payoff = utilities::contract_default(state.variables.performance)
                    * utilities::contract_role_sign(state.attributes.contract_role)
                    * (state.attributes.price_at_termination_date
                        + state.variables.nominal_accrued_1
                        + utilities::year_fraction(
                            state.variables.last_event_date,
                            event.time,
                            state.attributes.day_count_convention.unwrap(), // This unwrap will never panic.
                        ) * state.variables.nominal_rate
                            * state.variables.nominal_value_1);

                // State Transition Function
                state.variables.nominal_value_1 = Real::from(0);

                state.variables.nominal_accrued_1 = Real::from(0);

                state.variables.fee_accrued = Real::from(0);

                state.variables.nominal_rate = Real::from(0);

                state.variables.last_event_date = event.time;

                // Return the contract state and payoff
                Ok((state, payoff))
            }
            ContractEventType::IP => {
                // Payoff Function
                let payoff = utilities::contract_default(state.variables.performance)
                    * state.variables.interest_scaling_multiplier
                    * (state.variables.nominal_accrued_1
                        + utilities::year_fraction(
                            state.variables.last_event_date,
                            event.time,
                            state.attributes.day_count_convention.unwrap(), // This unwrap will never panic.
                        ) * state.variables.nominal_rate
                            * state.variables.nominal_value_1);

                // State Transition Function
                state.variables.nominal_accrued_1 = Real::from(0);

                if state.attributes.fee_basis == Some(FeeBasis::N) {
                    state.variables.fee_accrued = state.variables.fee_accrued
                        + utilities::year_fraction(
                            state.variables.last_event_date,
                            event.time,
                            state.attributes.day_count_convention.unwrap(), // This unwrap will never panic.
                        ) * state.variables.nominal_value_1
                            * state.attributes.fee_rate;
                } else {
                    let mut t_minus = Time(None);
                    let mut t_plus = Time(None);
                    for e in state.schedule.clone() {
                        if e.event_type == ContractEventType::FP {
                            if e.time >= t0 {
                                t_plus = e.time;
                                break;
                            }
                            t_minus = e.time;
                        }
                    }
                    state.variables.fee_accrued = utilities::year_fraction(
                        t_minus,
                        event.time,
                        state.attributes.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) / year_fraction(
                        t_minus,
                        t_plus,
                        state.attributes.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) * state.attributes.fee_rate;
                }

                state.variables.last_event_date = event.time;

                // Return the contract state and payoff
                Ok((state, payoff))
            }
            ContractEventType::IPCI => {
                // Payoff Function
                let payoff = Real::from(0);

                // State Transition Function
                let nominal_value_1_minus = state.variables.nominal_value_1; // Temporary variable.

                state.variables.nominal_value_1 = state.variables.nominal_value_1
                    + state.variables.nominal_accrued_1
                    + utilities::year_fraction(
                        state.variables.last_event_date,
                        event.time,
                        state.attributes.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) * state.variables.nominal_value_1
                        * state.variables.nominal_rate;

                state.variables.nominal_accrued_1 = Real::from(0);

                if state.attributes.fee_basis == Some(FeeBasis::N) {
                    state.variables.fee_accrued = state.variables.fee_accrued
                        + utilities::year_fraction(
                            state.variables.last_event_date,
                            event.time,
                            state.attributes.day_count_convention.unwrap(), // This unwrap will never panic.
                        ) * nominal_value_1_minus
                            * state.attributes.fee_rate;
                } else {
                    let mut t_minus = Time(None);
                    let mut t_plus = Time(None);
                    for e in state.schedule.clone() {
                        if e.event_type == ContractEventType::FP {
                            if e.time >= t0 {
                                t_plus = e.time;
                                break;
                            }
                            t_minus = e.time;
                        }
                    }
                    state.variables.fee_accrued = utilities::year_fraction(
                        t_minus,
                        event.time,
                        state.attributes.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) / year_fraction(
                        t_minus,
                        t_plus,
                        state.attributes.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) * state.attributes.fee_rate;
                }

                state.variables.last_event_date = event.time;

                // Return the contract state and payoff
                Ok((state, payoff))
            }
            ContractEventType::RR => {
                // Payoff Function
                let payoff = Real::from(0);

                // State Transition Function
                state.variables.nominal_accrued_1 = state.variables.nominal_accrued_1
                    + utilities::year_fraction(
                        state.variables.last_event_date,
                        event.time,
                        state.attributes.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) * state.variables.nominal_rate
                        * state.variables.nominal_value_1;

                if state.attributes.fee_basis == Some(FeeBasis::N) {
                    state.variables.fee_accrued = state.variables.fee_accrued
                        + utilities::year_fraction(
                            state.variables.last_event_date,
                            event.time,
                            state.attributes.day_count_convention.unwrap(), // This unwrap will never panic.
                        ) * state.variables.nominal_value_1
                            * state.attributes.fee_rate;
                } else {
                    let mut t_minus = Time(None);
                    let mut t_plus = Time(None);
                    for e in state.schedule.clone() {
                        if e.event_type == ContractEventType::FP {
                            if e.time >= t0 {
                                t_plus = e.time;
                                break;
                            }
                            t_minus = e.time;
                        }
                    }
                    state.variables.fee_accrued = utilities::year_fraction(
                        t_minus,
                        event.time,
                        state.attributes.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) / year_fraction(
                        t_minus,
                        t_plus,
                        state.attributes.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) * state.attributes.fee_rate;
                }

                let delta_r = Real::min(
                    Real::max(
                        <oracle::Oracles<T>>::get(
                            state.attributes.market_object_code_rate_reset.unwrap(), //This unwrap will never panic.
                        )
                        .value
                            * state.attributes.rate_multiplier
                            + state.attributes.rate_spread
                            - state.variables.nominal_rate,
                        state.attributes.period_floor,
                    ),
                    state.attributes.period_cap,
                );
                state.variables.nominal_rate = Real::min(
                    Real::max(
                        state.variables.nominal_rate + delta_r,
                        state.attributes.life_floor,
                    ),
                    state.attributes.life_cap,
                );

                state.variables.last_event_date = event.time;

                // Return the contract state and payoff
                Ok((state, payoff))
            }
            ContractEventType::RRF => {
                // Payoff Function
                let payoff = Real::from(0);

                // State Transition Function
                state.variables.nominal_accrued_1 = state.variables.nominal_accrued_1
                    + utilities::year_fraction(
                        state.variables.last_event_date,
                        event.time,
                        state.attributes.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) * state.variables.nominal_rate
                        * state.variables.nominal_value_1;

                if state.attributes.fee_basis == Some(FeeBasis::N) {
                    state.variables.fee_accrued = state.variables.fee_accrued
                        + utilities::year_fraction(
                            state.variables.last_event_date,
                            event.time,
                            state.attributes.day_count_convention.unwrap(), // This unwrap will never panic.
                        ) * state.variables.nominal_value_1
                            * state.attributes.fee_rate;
                } else {
                    let mut t_minus = Time(None);
                    let mut t_plus = Time(None);
                    for e in state.schedule.clone() {
                        if e.event_type == ContractEventType::FP {
                            if e.time >= t0 {
                                t_plus = e.time;
                                break;
                            }
                            t_minus = e.time;
                        }
                    }
                    state.variables.fee_accrued = utilities::year_fraction(
                        t_minus,
                        event.time,
                        state.attributes.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) / year_fraction(
                        t_minus,
                        t_plus,
                        state.attributes.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) * state.attributes.fee_rate;
                }

                state.variables.nominal_rate = state.attributes.next_reset_rate;

                state.variables.last_event_date = event.time;

                // Return the contract state and payoff
                Ok((state, payoff))
            }
            ContractEventType::SC => {
                // Payoff Function
                let payoff = Real::from(0);

                // State Transition Function
                state.variables.nominal_accrued_1 = state.variables.nominal_accrued_1
                    + utilities::year_fraction(
                        state.variables.last_event_date,
                        event.time,
                        state.attributes.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) * state.variables.nominal_rate
                        * state.variables.nominal_value_1;

                if state.attributes.fee_basis == Some(FeeBasis::N) {
                    state.variables.fee_accrued = state.variables.fee_accrued
                        + utilities::year_fraction(
                            state.variables.last_event_date,
                            event.time,
                            state.attributes.day_count_convention.unwrap(), // This unwrap will never panic.
                        ) * state.variables.nominal_value_1
                            * state.attributes.fee_rate;
                } else {
                    let mut t_minus = Time(None);
                    let mut t_plus = Time(None);
                    for e in state.schedule.clone() {
                        if e.event_type == ContractEventType::FP {
                            if e.time >= t0 {
                                t_plus = e.time;
                                break;
                            }
                            t_minus = e.time;
                        }
                    }
                    state.variables.fee_accrued = utilities::year_fraction(
                        t_minus,
                        event.time,
                        state.attributes.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) / year_fraction(
                        t_minus,
                        t_plus,
                        state.attributes.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) * state.attributes.fee_rate;
                }

                // Unwrap will never panic because of the lazy evaluation.
                if state.attributes.scaling_effect.is_some()
                    && state.attributes.scaling_effect.unwrap().y == false
                {
                    state.variables.notional_scaling_multiplier =
                        state.variables.notional_scaling_multiplier;
                } else {
                    state.variables.notional_scaling_multiplier = (<oracle::Oracles<T>>::get(
                        state.attributes.market_object_code_rate_reset.unwrap(), //This unwrap will never panic.
                    )
                    .value
                        - state.attributes.scaling_index_at_status_date)
                        / state.attributes.scaling_index_at_status_date;
                }

                // Unwrap will never panic because of the lazy evaluation.
                if state.attributes.scaling_effect.is_some()
                    && state.attributes.scaling_effect.unwrap().x == false
                {
                    state.variables.interest_scaling_multiplier =
                        state.variables.interest_scaling_multiplier;
                } else {
                    state.variables.interest_scaling_multiplier = (<oracle::Oracles<T>>::get(
                        state.attributes.market_object_code_rate_reset.unwrap(), //This unwrap will never panic.
                    )
                    .value
                        - state.attributes.scaling_index_at_status_date)
                        / state.attributes.scaling_index_at_status_date;
                }

                state.variables.last_event_date = event.time;

                // Return the contract state and payoff
                Ok((state, payoff))
            }
            ContractEventType::CD => {
                // Payoff Function
                let payoff = Real::from(0);

                // State Transition Function
                state.variables.nominal_accrued_1 = state.variables.nominal_accrued_1
                    + utilities::year_fraction(
                        state.variables.last_event_date,
                        event.time,
                        state.attributes.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) * state.variables.nominal_rate
                        * state.variables.nominal_value_1;

                if state.attributes.fee_basis == Some(FeeBasis::N) {
                    state.variables.fee_accrued = state.variables.fee_accrued
                        + utilities::year_fraction(
                            state.variables.last_event_date,
                            event.time,
                            state.attributes.day_count_convention.unwrap(), // This unwrap will never panic.
                        ) * state.variables.nominal_value_1
                            * state.attributes.fee_rate;
                } else {
                    let mut t_minus = Time(None);
                    let mut t_plus = Time(None);
                    for e in state.schedule.clone() {
                        if e.event_type == ContractEventType::FP {
                            if e.time >= t0 {
                                t_plus = e.time;
                                break;
                            }
                            t_minus = e.time;
                        }
                    }
                    state.variables.fee_accrued = utilities::year_fraction(
                        t_minus,
                        event.time,
                        state.attributes.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) / year_fraction(
                        t_minus,
                        t_plus,
                        state.attributes.day_count_convention.unwrap(), // This unwrap will never panic.
                    ) * state.attributes.fee_rate;
                }

                state.variables.performance = Some(ContractPerformance::DF);

                state.variables.last_event_date = event.time;

                // Return the contract state and payoff
                Ok((state, payoff))
            }
            _ => Err("Event not applicable"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use primitives::{Blake2Hasher, H256};
    use runtime_io::with_externalities;
    use runtime_primitives::{
        testing::{Digest, DigestItem, Header},
        traits::{BlakeTwo256, IdentityLookup},
        BuildStorage,
    };
    use support::{assert_ok, impl_outer_origin};

    impl_outer_origin! {
        pub enum Origin for Test {}
    }

    #[derive(Clone, Eq, PartialEq)]
    pub struct Test;
    impl system::Trait for Test {
        type Origin = Origin;
        type Index = u64;
        type BlockNumber = u64;
        type Hash = H256;
        type Hashing = BlakeTwo256;
        type Digest = Digest;
        type AccountId = u64;
        type Lookup = IdentityLookup<Self::AccountId>;
        type Header = Header;
        type Event = ();
        type Log = DigestItem;
    }
    impl timestamp::Trait for Test {
        type Moment = u64;
        type OnTimestampSet = ();
    }
    impl oracle::Trait for Test {}
    impl assets::Trait for Test {}
    impl Trait for Test {}
    type Contracts = Module<Test>;

    fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
        system::GenesisConfig::<Test>::default()
            .build_storage()
            .unwrap()
            .0
            .into()
    }

    #[test]
    fn progress_pam_works() {
        with_externalities(&mut new_test_ext(), || {
            let t0 = Time::from_values(2015, 01, 01, 00, 00, 00);
            let id = H256::random();
            let mut attributes = Attributes::new(id);
            attributes.contract_id = id;
            attributes.contract_type = Some(ContractType::PAM);
            attributes.currency = Some(1);
            attributes.day_count_convention = Some(DayCountConvention::_30E360);
            attributes.initial_exchange_date = Time::from_values(2015, 01, 02, 00, 00, 00);
            attributes.maturity_date = Time::from_values(2015, 04, 02, 00, 00, 00);
            attributes.nominal_interest_rate = Real::from(0);
            attributes.notional_principal = Real::from(1000);
            attributes.contract_deal_date = Time::from_values(2015, 01, 01, 00, 00, 00);
            attributes.contract_role = Some(ContractRole::RPA);
            attributes.creator_id = Some(H256::random());
            attributes.counterparty_id = Some(H256::random());
            attributes.scaling_effect = None;
            attributes.rate_spread = Real::from(0);
            attributes.premium_discount_at_ied = Real::from(-5);

            let mut state = Contracts::deploy_pam(t0, attributes).unwrap();

            assert_eq!(
                state.schedule[0],
                ContractEvent::new(
                    Time::from_values(2015, 01, 02, 00, 00, 00),
                    ContractEventType::IED
                )
            );
            state = Contracts::progress_pam(state.schedule[0], state).unwrap().0;
            assert_eq!(state.variables.nominal_value_1, Real::from(1000));
            assert_eq!(state.variables.nominal_rate, Real::from(0));
            assert_eq!(state.variables.nominal_accrued_1, Real::from(0));

            // Event 2 is being used, instead of the next in the sequence 1, because the
            // given test vectors don't mention event 1 (probably because it has no effect
            // on the state).
            assert_eq!(
                state.schedule[2],
                ContractEvent::new(
                    Time::from_values(2015, 04, 02, 00, 00, 00),
                    ContractEventType::PR
                )
            );
            state = Contracts::progress_pam(state.schedule[2], state).unwrap().0;
            assert_eq!(state.variables.nominal_value_1, Real::from(0));
            assert_eq!(state.variables.nominal_rate, Real::from(0));
            assert_eq!(state.variables.nominal_accrued_1, Real::from(0));
        });
    }
}
