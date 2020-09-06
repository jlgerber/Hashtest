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

pub trait OpenMut: Open {
    type OW: Read + Write;
    //type E: std::error::Error;

    /// Open the output for reading or writing, depending upon the OpenMode
    fn open_mut<I>(
        &mut self,
        input: I,
        mode: OpenMode,
    ) -> std::result::Result<Self::OW, HashitError>
    where
        I: AsRef<str>;

    fn create<I>(&mut self, input: I) -> HResult<()>
    where
        I: AsRef<str>;
}
