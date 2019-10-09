use super::*;

// This function creates a new ACTUS contract.
impl<T: Trait> Module<T> {
    pub fn progress(event: ContractEvent, mut state: ContractState) -> Result {
        // Getting the contract ID.
        let id = state.attributes.contract_id;

        // Calculating the resulting contract state.
        let mut payoff = Real::from(0);
        match state.attributes.contract_type {
            Some(ContractType::PAM) => {
                let result = Self::progress_pam(event, state)?;
                state = result.0;
                payoff = result.1;
            }
            _ => {
                Err("Contract type not supported")?;
            }
        }

        // Executing the payoff.
        // Note: not sure if those unwrap() will not panic.
        if payoff >= Real::from(0) {
            <assets::Module<T>>::transfer(
                state.attributes.counterparty_id.unwrap(),
                state.attributes.creator_id.unwrap(),
                state.attributes.currency.unwrap(),
                payoff.abs(),
            )?;
        } else {
            <assets::Module<T>>::transfer(
                state.attributes.creator_id.unwrap(),
                state.attributes.counterparty_id.unwrap(),
                state.attributes.currency.unwrap(),
                payoff.abs(),
            )?;
        }

        // Storing the contract state.
        <ContractStates<T>>::insert(id, state);

        // Return Ok if successful.
        Ok(())
    }
}
