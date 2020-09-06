use crate::traits::CalcHash;
use crate::utils::{blake_hash, read_file};
use crate::HashitError;
use crate::OpenMode;
use crate::Result as HResult;
use crate::{Open, OpenMut};
use std::fs;
use std::path::PathBuf;
#[derive(Debug)]
pub struct HtFile {}

impl Default for HtFile {
    fn default() -> Self {
        HtFile {}
    }
}
impl HtFile {
    pub fn new() -> Self {
        HtFile::default()
    }
}
impl Open for HtFile {
    type O = fs::File;
    //type E = HashitError;
    fn open<I>(&self, input: I) -> std::result::Result<Self::O, HashitError>
    where
        I: AsRef<str>,
    {
        let output_file = input.as_ref();
        Ok(fs::File::open(output_file).map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                HashitError::NotFound {
                    source: e,
                    file: PathBuf::from(output_file),
                }
            } else {
                e.into()
            }
        })?)
    }

    fn exists<I>(&self, input: I) -> bool
    where
        I: AsRef<str>,
    {
        let file = input.as_ref();
        let path = std::path::Path::new(file);
        path.exists()
    }
}

impl OpenMut for HtFile {
    type OW = fs::File;
    fn open_mut<I>(
        &mut self,
        input: I,
        mode: OpenMode,
    ) -> std::result::Result<Self::OW, HashitError>
    where
        I: AsRef<str>,
    {
        let output_file = input.as_ref();

        match mode {
            OpenMode::WriteAppend => Ok(fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(false)
                .open(output_file)
                .map_err(|e| {
                    if e.kind() == std::io::ErrorKind::NotFound {
                        HashitError::NotFound {
                            source: e,
                            file: PathBuf::from(output_file),
                        }
                    } else {
                        e.into()
                    }
                })?),
            OpenMode::WriteTruncate => Ok(fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(output_file)
                .map_err(|e| {
                    if e.kind() == std::io::ErrorKind::NotFound {
                        HashitError::NotFound {
                            source: e,
                            file: PathBuf::from(output_file),
                        }
                    } else {
                        e.into()
                    }
                })?),
        }
    }

    fn create<I>(&mut self, path: I) -> HResult<()>
    where
        I: AsRef<str>,
    {
        let mut pathb = std::path::PathBuf::from(path.as_ref());
        pathb.pop();
        fs::create_dir_all(&pathb)?;

        fs::File::create(path.as_ref())?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct FileHash {}
impl CalcHash for FileHash {
    fn calc_hash<P>(&self, files: &[P]) -> HResult<Vec<u8>>
    where
        P: AsRef<str>,
    {
        let mut resvec = Vec::new();

        for f in files {
            let file = f.as_ref();
            let file_contents = read_file(file)?;
            let result = blake_hash(&file_contents);
            resvec.extend(result);
        }
        Ok(resvec)
    }
}
