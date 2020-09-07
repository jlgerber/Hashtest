//! implements the traits required by Hashit in order to test
//! without writing to disk.
use crate::traits::{CalcHash, FetchCachedHash, Open, OpenMut};
use crate::utils::blake_hash;
use crate::HashitError;
use crate::OpenMode;
use crate::Result as HResult;

use std::io::Cursor;

#[derive(Debug)]
pub struct HtString {}

impl Default for HtString {
    fn default() -> Self {
        HtString {}
    }
}
impl HtString {
    pub fn new() -> Self {
        HtString::default()
    }
}
impl Open for HtString {
    type O = Cursor<Vec<u8>>;
    //type E = HashitError;
    fn open<I>(&self, input: I) -> std::result::Result<Self::O, HashitError>
    where
        I: AsRef<str>,
    {
        Ok(Cursor::new(input.as_ref().to_string().into_bytes()))
    }

    fn exists<I>(&self, _input: I) -> bool
    where
        I: AsRef<str>,
    {
        true
    }
}

impl OpenMut for HtString {
    type OW = Cursor<Vec<u8>>;
    fn open_mut<I>(
        &mut self,
        input: I,
        mode: OpenMode,
    ) -> std::result::Result<Self::OW, HashitError>
    where
        I: AsRef<str>,
    {
        match mode {
            OpenMode::WriteAppend => Ok(Cursor::new(input.as_ref().to_string().into_bytes())),
            OpenMode::WriteTruncate => Ok(Cursor::new(String::new().into_bytes())),
        }
    }

    fn create<I>(&mut self, _path: I) -> HResult<()>
    where
        I: AsRef<str>,
    {
        Ok(())
    }
}

impl FetchCachedHash for HtString {
    fn fetch_cached_hash(&mut self, input: &str) -> HResult<Vec<u8>> {
        // we dont have anything to open. The "cached hash" is simply the
        // hash of the input
        Ok(blake_hash(input.as_bytes()))
    }
}

#[derive(Debug)]
pub struct StringHash {}
impl CalcHash for StringHash {
    fn calc_hash<P>(&self, files: &[P]) -> HResult<Vec<u8>>
    where
        P: AsRef<str>,
    {
        let mut resvec = Vec::new();

        for f in files {
            let result = blake_hash(f.as_ref().as_bytes());
            resvec.extend(result);
        }
        Ok(resvec)
    }
}
