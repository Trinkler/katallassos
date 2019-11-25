// Defines an alias for the Result type. It has the name ContractResult because Substrate
// already uses the name Result for their own type Result<(), &'static str>.
pub type ContractResult<T> = rstd::result::Result<T, &'static str>;
