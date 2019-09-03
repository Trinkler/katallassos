// Each struct (and its child structs and enums) should be on its own separate file.

// This line is necessary to import the parent file, and indirectly import
// all files in the module.
use super::*;

// Structs work just like in Rust. The only difference is that the two next lines always
// need to prefix the definition of a struct. Also, don't use Substrate types.
#[derive(Clone, Copy, Decode, Encode, Default, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct TemplateState {
    pub field1: i16,
    pub field2: u128,
    pub field3: Option<SomeEnum>,
}

// Enums are just like the structs, but not that in the first line different traits
// are derived. That's because enums can't derive the trait Default.
#[derive(Clone, Copy, Decode, Encode, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum SomeEnum {
    A,
    B,
    C,
}
