//! Hashit
//! Keep track of changes to items that are hashable via the Hashit struct.
//!
//! In order to make Hashit more testable, we provide the Open and OpenMut traits
//! which abstract (you guessed it) opening of types which implement Read and Write.
//! This is necessary because the native apis abstract reading and writing, but dont
//! provide a similar abstraction for opening, making it difficult to rely on the
//! Read and Write traits for testing.
//!
//! Hashit provides an implementation of the Open trait for std::fs::File, which
//! the Hashit struct uses by default (and for default construction)
//!
//! Hashit instances provide a single function - has_changed - which takes a slice of
//! inputs and an output. In brief, the method builds up a hash (built up as a Vec<u8>)
//! from the inputs, and compares the hash to the contents of output, which it interprets
//! as a Vec<u8> hash, returning a bool representing whether the inputs have changed (true)
//! or not (false).
//!
//! If the inputs' hash is different than the output hash, hashit.has_changed(...) overrites the
//! hash in the output with the newly calculated inputs' hash.
//!
//! If the output does not exist, hashit.has_changed(...) will create it and populate it with the
//! newly created inputs' hash, before returning ```true```.
//!
//! # Example
//!
//! Lets say we have two files which we want to keep track of. We could simply check
//! os specific metadata. Instead, we are just going to hash the files using Hashit,
//! and test to see if the files have changed since the last time we hashed them.
//!
//! ```
//! use hashtest::Hashit;
//! use hashtest::HashitError;
//!
//! # fn main() -> Result<(), HashitError> {
//!
//! let mut hashit = Hashit::new();
//! let input = format!("{}/eg/platform.rs", env!("CARGO_MANIFEST_DIR"));
//! let output = format!("/tmp/hashtest.hash");
//! let result = hashit.has_changed(&vec![input.as_str()], output)?;
//! # Ok(())
//! # }
//!
pub mod error;
pub use error::Result;
pub use error::*;

pub mod traits;
pub use traits::{Open, OpenMut};
//
pub mod open_mode;
pub use open_mode::OpenMode;
//
pub mod file;
pub use file::HtFile;
//
pub mod utils;
//use utils::*;
//
pub mod hashit;
pub use hashit::*;

#[cfg(test)]
pub mod string;
#[cfg(test)]
pub use string::*;
