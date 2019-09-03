// Each internal function should be on its own separate file.

// This line is necessary to import the parent file, and indirectly import
// all files in the module.
use super::*;

// This is an internal function. It looks like a normal Rust function except that it's inside
// an "impl<T: Trait> Module<T>" block. An internal function can access the storage and the
// functions (dispatchable, internal or pure) of this or any other module. However, they
// can't be called by pure functions.
impl<T: Trait> Module<T> {
    pub fn internal_function() -> Result {
        // Call a pure function.
        let number = pure_function();

        // Store value.
        <SomeValue<T>>::put(number);

        // Return Ok.
        Ok(())
    }
}
