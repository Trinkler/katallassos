use super::*;

// This function creates a new ACTUS contract.
impl<T: Trait> Module<T> {
    pub fn progress(event: ContractEvent, mut state: ContractState) -> Result {
        // Getting the contract ID.
        let id = state.attributes.contract_id;

        // Calculating the resulting contract state.
        match state.attributes.contract_type {
            Some(ContractType::PAM) => {
                state = Self::progress_pam(event, state)?;
            }
            _ => {
                state = Err("Contract type not supported")?;
            }
        }

        // Note: The payoff calls to the issuer module should happen here.

        // Storing the contract state.
        <Contracts<T>>::insert(id, state);

        // Return Ok if successful.
        Ok(())
    }
}
