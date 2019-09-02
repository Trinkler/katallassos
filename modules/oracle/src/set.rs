use super::*;

// This function sets an (or creates a new) oracle.
impl<T: Trait> Module<T> {
    pub fn set(id: H256, time: Time, value: Real) -> Result {
        // Create the oracle state struct.
        let state = OracleState {
            time: time,
            value: value,
        };

        // Store input value in storage.
        <Self as Store>::Oracles::insert(id, state);

        // Return Ok if successful.
        Ok(())
    }
}
