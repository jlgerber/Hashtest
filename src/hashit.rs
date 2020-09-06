use crate::error::Result;
use crate::file::HtFile;
use crate::open_mode::OpenMode;
use crate::traits::OpenMut;
use crate::utils::*;

use std::io::prelude::*;

use std::path::Path;
/// Hashit is constructed with a
/// Hashit exists as a struct to facilitate testing.
#[derive(Debug)]
pub struct Hashit<R> {
    inner: R,
}

impl Default for Hashit<HtFile> {
    fn default() -> Self {
        Hashit::<HtFile> {
            inner: HtFile::new(),
        }
    }
}

impl Hashit<HtFile> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a, R: OpenMut> Hashit<R> {
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
        let hash = calc_hash(inputs)?;
        let mut buffer = Vec::<u8>::new();
        let output_str = output.as_ref().to_string_lossy();
        {
            let inner = &mut self.inner;
            let exists = inner.exists(output_str.as_ref());
            if !exists {
                inner.create(output_str.as_ref())?;
            }
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
