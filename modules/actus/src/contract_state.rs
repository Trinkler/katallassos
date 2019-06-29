use super::*;

// This struct contains all the information that defines a contract state.
#[derive(Clone, Decode, Encode, Default, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct ContractState {
    attributes: Attributes,
    variables: Variables,
    schedule: Vec<ContractEvent>,
}
