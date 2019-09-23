use super::*;

// This function sets an (or creates a new) oracle.
impl<T: Trait> Module<T> {
    pub fn set(id: H256, value: Real) -> Result {
        let unix_time = <timestamp::Module<T>>::get().as_();
        let time = Time::from_unix(unix_time);

        // Create the oracle state struct.
        let state = OracleState {
            time: time,
            value: value,
        };

        // Store input value in storage.
        <Oracles<T>>::insert(id, state);

        // Return Ok if successful.
        Ok(())
    }
}
