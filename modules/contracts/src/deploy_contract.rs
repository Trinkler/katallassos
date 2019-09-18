use super::*;

// This function creates a new ACTUS contract.
impl<T: Trait> Module<T> {
    pub fn deploy_contract(attributes: Attributes) -> Result {
        // Getting the contract ID.
        let id = attributes.contract_id;

        // Checking if ID is available.
        if <Contracts<T>>::exists(id) {
            return Err("Contract ID already exists");
        }

        // TODO: Get current time.
        let t0 = Time::from_values(1969, 07, 20, 20, 17, 00);

        // Calculating the initial contract state.
        let state = Self::initialize(t0, attributes)?;

        // Storing the contract state.
        <Contracts<T>>::insert(id, state);

        // Return Ok if successful.
        Ok(())
    }
}
