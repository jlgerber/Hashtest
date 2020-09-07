use crate::error::HashitError;
use crate::error::Result;
use blake2::{Blake2b, Digest};
use std::fs::File;
use std::io::prelude::*;
//use std::path::Path;
use std::path::PathBuf;

/// Given a slice of u8, calculate a hash using the Blake2 algorithm
pub(crate) fn blake_hash(input: &[u8]) -> Vec<u8> {
    let mut hasher = Blake2b::new();
    hasher.update(input);
    let res = hasher.finalize();
    res.to_vec()
}

// Read a file in and return its contents as a Vec<u8>
pub(crate) fn read_file<I>(path: I) -> Result<Vec<u8>>
where
    I: AsRef<std::path::Path>,
{
    let path = path.as_ref();
    let mut f = File::open(path).map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            HashitError::NotFound {
                source: e,
                file: PathBuf::from(path),
            }
        } else {
            e.into()
        }
    })?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    Ok(buffer)
}

// // Calculate a unique hash given a list of Paths
// pub(crate) fn calc_hash<P>(files: &[P]) -> Result<Vec<u8>>
// where
//     P: AsRef<Path>,
// {
//     let mut resvec = Vec::new();

//     for f in files {
//         let file = f.as_ref();
//         let file_contents = read_file(file)?;
//         let result = blake_hash(&file_contents);
//         resvec.extend(result);
//     }
//     Ok(resvec)
// }
// // Calculate a unique hash given a list of Paths
// pub(crate) fn calc_hash2<P>(files: &[P]) -> Result<Vec<u8>>
// where
//     P: AsRef<str>,
// {
//     let mut resvec = Vec::new();

//     for f in files {
//         let file = f.as_ref();
//         let file_contents = read_file(file)?;
//         let result = blake_hash(&file_contents);
//         resvec.extend(result);
//     }
//     Ok(resvec)
// }
