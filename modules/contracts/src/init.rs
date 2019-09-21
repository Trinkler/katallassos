use super::*;

// This function basically performs auto-run for this module.
impl<T: Trait> Module<T> {
    pub fn init() -> Result {
        // TODO: Get current time.
        let t = Time::from_values(1969, 07, 20, 20, 17, 00);

        // Calculating the initial contract state.
        Self::scheduler_run(t)?;

        // Return Ok if successful.
        Ok(())
    }
}
