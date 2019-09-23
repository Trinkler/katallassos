use super::*;

// This function basically performs auto-run for this module.
impl<T: Trait> Module<T> {
    pub fn init() -> Result {
        // Get current time.
        let t = Time::from_unix(<timestamp::Module<T>>::get().as_());

        // Calculating the initial contract state.
        Self::scheduler_run(t)?;

        // Return Ok if successful.
        Ok(())
    }
}
