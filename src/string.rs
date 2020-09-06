//! implement
use crate::traits::CalcHash;
use crate::utils::{blake_hash, read_file};
use crate::HashitError;
use crate::OpenMode;
use crate::Result as HResult;
use crate::{Open, OpenMut};
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
