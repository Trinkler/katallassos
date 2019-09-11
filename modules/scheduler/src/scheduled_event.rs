use super::*;

// This struct defines a scheduled event.
#[derive(Clone, Decode, Encode, Default, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct ScheduledEvent {
    pub contract_id: H256,
    pub time: Time,
    pub index: u32,
}
