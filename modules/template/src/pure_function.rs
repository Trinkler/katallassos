// Each pure function should be on its own separate file.

// This line is necessary to import the parent file, and indirectly import
// all files in the module.
use super::*;

// This is a pure function. It is a normal Rust function. They can't access the storage, the
// dispatchable functions or the internal functions of this or any other module. They can
// only call other pure function in this or other modules. However, they can be called by
// any other function.
pub fn pure_function() -> u32 {
    23
}
