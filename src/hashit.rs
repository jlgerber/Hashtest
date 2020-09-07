use crate::error::Result;
use crate::file::{FileHash, HtFile};
use crate::open_mode::OpenMode;
use crate::traits::*;
use crate::utils::*;

use std::io::prelude::*;

use std::path::Path;
/// Hashit is constructed with a
/// Hashit exists as a struct to facilitate testing.
#[derive(Debug)]
pub struct Hashit<R, H> {
    inner: R,
    hasher: H,
}

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

impl<'a, R: OpenMut, H: CalcHash + std::fmt::Debug> Hashit<R, H> {
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
        let inputs2 = inputs
            .iter()
            .map(|x| x.as_ref().to_string_lossy())
            .collect::<Vec<_>>();
        //let hash = calc_hash(inputs)?;
        // Here we are calculating the hash of each of the inputs and
        // returning an accumulated value.
        let hash = (self.hasher).calc_hash(&inputs2[..])?;
        let mut buffer = Vec::<u8>::new();
        let output_str = output.as_ref().to_string_lossy();
        {
            let inner = &mut self.inner;
            let exists = inner.exists(output_str.as_ref());
            if !exists {
                inner.create(output_str.as_ref())?;
            }
            // Here we are reading the buffer directly and expecting the value to
            // be a hash already
            let mut reader = inner.open(output_str.as_ref())?;
            reader.read_to_end(&mut buffer)?;
            drop(reader);
        }
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
