use super::*;

// TODO: Add the payoff functions.
pub fn progress_pam(event: ContractEvent, mut state: ContractState) -> MyResult<ContractState> {
    // Getting t0 from the status_date attribute since they are equal.
    // (And status_date is not supposed to change)
    let t0 = state.attributes.status_date;

    match event.event_type {
        ContractEventType::IED => {
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
        }
        ContractEventType::PR => {
            // State Transition Function
            state.variables.nominal_value_1 = Real::from(0);

            state.variables.nominal_rate = Real::from(0);

            state.variables.last_event_date = event.time;
        }
        ContractEventType::PP => {
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

            // TODO: Consider the user-initiated events based on the "OPMO".
            state.variables.nominal_value_1 = state.variables.nominal_value_1;

            state.variables.last_event_date = event.time;
        }
        ContractEventType::PY => {
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
        }
        ContractEventType::FP => {
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
        }
        ContractEventType::PRD => {
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
        }
        ContractEventType::TD => {
            // State Transition Function
            state.variables.nominal_value_1 = Real::from(0);

            state.variables.nominal_accrued_1 = Real::from(0);

            state.variables.fee_accrued = Real::from(0);

            state.variables.nominal_rate = Real::from(0);

            state.variables.last_event_date = event.time;
        }
        ContractEventType::IP => {
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
        }
        ContractEventType::IPCI => {
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
        }
        ContractEventType::RR => {
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

            // TODO: Consider the oracle based on the "RRMO" attribute.
            // TODO: Verify with Nils that it is indeed rate_multiplier.
            let delta_r = Real::min(
                Real::max(
                    state.attributes.rate_multiplier + state.attributes.rate_spread
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
        }
        ContractEventType::RRF => {
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
        }
        ContractEventType::SC => {
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
                // TODO: Consider the oracle based on the "SCMO" attribute.
                state.variables.notional_scaling_multiplier =
                    (state.attributes.scaling_index_at_status_date)
                        / state.attributes.scaling_index_at_status_date;
            }

            // Unwrap will never panic because of the lazy evaluation.
            if state.attributes.scaling_effect.is_some()
                && state.attributes.scaling_effect.unwrap().x == false
            {
                state.variables.interest_scaling_multiplier =
                    state.variables.interest_scaling_multiplier;
            } else {
                // TODO: Consider the oracle based on the "SCMO" attribute.
                state.variables.interest_scaling_multiplier =
                    (state.attributes.scaling_index_at_status_date)
                        / state.attributes.scaling_index_at_status_date;
            }

            state.variables.last_event_date = event.time;
        }
        ContractEventType::CD => {
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
        }
        _ => {}
    }
    // Return the contract state
    Ok(state)
}
