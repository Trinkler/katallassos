use super::*;

#[derive(Clone, Copy, Decode, Encode, Default, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct AnotherStruct {
    pub field1: u64,
    pub field2: u64,
}
