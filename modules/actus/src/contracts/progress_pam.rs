use super::*;

// TODO: Add the payoff functions.
pub fn progress_pam(event: ContractEvent, mut state: ContractState) -> MyResult<ContractState> {
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

            // Return the contract state
            Ok(state)
        }
        ContractEventType::PR => {
            // State Transition Function
            state.variables.nominal_value_1 = Real::from(0);

            state.variables.nominal_rate = Real::from(0);

            state.variables.last_event_date = event.time;

            // Return the contract state
            Ok(state)
        }
        _ => Err("Exterminate!"),
    }
}
