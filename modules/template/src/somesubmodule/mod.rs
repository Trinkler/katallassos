// For each level of submodule, you should have a file like this. It must be named 'mod.rs'
// and it should only be used to do imports.

// This line imports the parent of this file. Because of the structure that we are using,
// this allows the children of this file to use any function or type defined in the rest
// of the module.
use super::*;

// This imports the children of this files.
mod another_function;
mod another_struct;

// This exports the children to the parent of this file, making them available to the
// rest of the module. Note the use of the command 'pub'.
pub use another_function::*;
pub use another_struct::*;
