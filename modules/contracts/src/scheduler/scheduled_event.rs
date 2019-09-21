use super::*;

// This struct defines a scheduled event.
#[derive(Copy, Clone, Decode, Encode, Default, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct ScheduledEvent {
    pub time: Time,
    pub contract_id: H256,
    pub index: u32,
}
