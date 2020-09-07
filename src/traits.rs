use crate::error::HashitError;
use crate::OpenMode;
use crate::Result as HResult;
use std::io::prelude::*;
pub trait Open {
    type O: Read;
    //type E: std::error::Error;

    /// Open the output for reading or writing, depending upon the OpenMode
    fn open<I>(&self, input: I) -> std::result::Result<Self::O, HashitError>
    where
        I: AsRef<str>;

    /// Determine whether
    fn exists<I>(&self, input: I) -> bool
    where
        I: AsRef<str>;
}

pub trait OpenMut<'a>: Open {
    type OW: Read + Write;
    //type E: std::error::Error;

    /// Open the output for reading or writing, depending upon the OpenMode
    fn open_mut<I>(
        &'a mut self,
        input: I,
        mode: OpenMode,
    ) -> std::result::Result<Self::OW, HashitError>
    where
        I: AsRef<str>;

    fn create<I>(&'a mut self, input: I) -> HResult<()>
    where
        I: AsRef<str>;
}
pub trait FetchCachedHash<'a>: OpenMut<'a> {
    fn fetch_cached_hash(&mut self, input: &str) -> HResult<Vec<u8>>;
}

pub trait CalcHash {
    fn calc_hash<R>(&self, inputs: &[R]) -> HResult<Vec<u8>>
    where
        R: AsRef<str>;
}
