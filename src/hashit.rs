use crate::error::Result;
use crate::file::{FileHash, HtFile};
use crate::open_mode::OpenMode;
use crate::traits::*;

use std::io::prelude::*;
use std::path::Path;
/// Hashit is constructed with a
/// Hashit exists as a struct to facilitate testing.
#[derive(Debug)]
pub struct Hashit<R, H> {
    inner: R,
    hasher: H,
}

/// Simplify default construction for production usage
/// We could go farther and mark as #[cfg(test)] vs non and
/// set up a default impl in both cases.
impl Default for Hashit<HtFile, FileHash> {
    fn default() -> Self {
        Hashit::<HtFile, FileHash> {
            inner: HtFile::new(),
            hasher: FileHash {},
        }
    }
}

impl Hashit<HtFile, FileHash> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a, R: OpenMut + FetchCachedHash, H: CalcHash + std::fmt::Debug> Hashit<R, H> {
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
    pub fn has_changed<IP, OP>(&'a mut self, inputs: &[IP], output: OP) -> Result<bool>
    where
        IP: AsRef<Path>,
        OP: AsRef<Path>,
    {
        // this is unfortunate. Because I designed has_changed to work with paths
        // I am stuck converting from a path to a string. I should probably
        // rething this and make has_changed take an Asref<str> or a &str
        let inputs2 = inputs
            .iter()
            .map(|x| x.as_ref().to_string_lossy())
            .collect::<Vec<_>>();

        // Here we are calculating the hash of each of the inputs and
        // returning an accumulated value.
        let hash = (self.hasher).calc_hash(&inputs2[..])?;
        // now we are going to read the value of the hash that has previously been cached.
        let output_str = output.as_ref().to_string_lossy();

        // fetch_cached_hash will create the output if it does not exist, returning an
        // empty buffer in that case.
        let buffer = self.inner.fetch_cached_hash(output_str.as_ref())?;
        let differs = hash != &buffer[..];
        if differs {
            let mut writer = self
                .inner
                .open_mut(output_str.as_ref(), OpenMode::WriteTruncate)?;
            writer.write_all(&hash)?;
        }
        Ok(differs)
    }
}

#[cfg(test)]
#[path = "./unit_tests/hashit_test.rs"]
mod tests;
