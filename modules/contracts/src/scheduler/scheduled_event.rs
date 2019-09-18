use super::*;

// This struct defines a scheduled event.
#[derive(Clone, Decode, Encode, Default, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct ScheduledEvent {
    pub time: Time,
    pub index: u32,
    pub contract_id: H256,
}
