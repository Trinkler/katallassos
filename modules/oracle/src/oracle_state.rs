use super::*;

// This struct defines the state of an oracle.
#[derive(Copy, Clone, Decode, Encode, Default, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct OracleState {
    pub time: Time,
    pub value: Real,
}
