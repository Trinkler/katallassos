use super::*;

// This function creates a new ACTUS contract.
impl<T: Trait> Module<T> {
    pub fn progress_contract(event: ContractEvent, mut state: ContractState) -> Result {
        // Getting the contract ID.
        let id = state.attributes.contract_id;

        // Calculating the resulting contract state.
        let state = Self::progress(event, state)?;

        // Note: The payoff calls to the issuer module should happen here.

        // Storing the contract state.
        <Contracts<T>>::insert(id, state);

        // Return Ok if successful.
        Ok(())
    }
}
