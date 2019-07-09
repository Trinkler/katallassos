use super::*;

mod attributes;
mod contract_events;
mod variables;

pub use attributes::*;
pub use contract_events::*;
pub use variables::*;

// This struct contains all the information that defines a contract state.
#[derive(Clone, Decode, Encode, Default, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct ContractState {
    pub attributes: Attributes,
    pub variables: Variables,
    pub schedule: Vec<ContractEvent>,
}
