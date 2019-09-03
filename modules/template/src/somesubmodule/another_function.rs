use super::*;

// This could also be a pure function.
impl<T: Trait> Module<T> {
    pub fn another_function() -> Result {
        // Create a TemplateState.
        let state = TemplateState {
            field1: 0,
            field2: 1,
            field3: None,
        };

        // Store value.
        <SomeMap<T>>::insert(101, state);

        // Return Ok.
        Ok(())
    }
}
