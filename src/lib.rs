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
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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
use utils::*;
//
pub mod hashit;
pub use hashit::*;
//
pub mod string;
pub use string::*;

/// Given a list of inputs, compare their collective hash to the value stored
/// in a file to determine if any of the files has changed since the last
/// time this function was run.
///
/// - If the output file does not exist, we assume that this is the first time
///   that this function has been run. We create the file, store the calculated
///   hash from inputs, and return true (the input(s) have changed)
/// - If the inputs' hash differs from the stored hash, we replace the stored hash
///   with the new hash, and return true (the file has changed)
/// - If the inputs' hash matches the stored hash, we return false (the input(s)
///   have not changed)
pub fn has_changed<IP, P>(inputs: &[IP], output_file: P) -> Result<bool>
where
    IP: AsRef<Path>,
    P: AsRef<Path>,
{
    let hash = calc_hash(inputs)?;
    let output_file = output_file.as_ref();
    if !output_file.exists() {
        // create missing directory if it doesnt exist
        let output_path = output_file
            .parent()
            .ok_or_else(|| HashitError::MissingDir(output_file.to_string_lossy().into_owned()))?;
        fs::create_dir_all(output_path)?;
        // write hash to file
        let mut file = File::create(&output_file)?;
        file.write_all(&hash)?;
        // has the input file changed? Well if there is no output path
        // then automatically that would be true
        return Ok(true);
    }
    // read contents and determine if hashes are equal
    let mut buffer = Vec::<u8>::new();
    {
        // create scope. file will be dropped
        let mut file = File::open(output_file)?;
        file.read_to_end(&mut buffer)?;
    }
    let differs = hash != &buffer[..];
    if differs {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(output_file)?;
        file.write_all(&hash)?;
    }
    Ok(differs)
}

// fn _has_changed<IP, OP>(inputs: &[IP], mut output: OP) -> Result<bool>
// where
//     IP: AsRef<Path>,
//     OP: Read + Write + Seek,
// {
//     let hash = calc_hash(inputs)?;
//     let mut buffer = Vec::<u8>::new();
//     output.read_to_end(&mut buffer)?;
//     let differs = hash != &buffer[..];
//     if differs {
//         output.seek(SeekFrom::Start(0))?;
//         output.write_all(&hash)?;
//     }
//     Ok(differs)
// }
