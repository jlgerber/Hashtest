//! implements the traits required by Hashit in order to test
//! without writing to disk.
//!
//! NB: This module only gets compiled into the library for tests.
//!
use crate::traits::{CalcHash, FetchCachedHash, Open, OpenMut};
use crate::utils::blake_hash;
use crate::HashitError;
use crate::OpenMode;
use crate::Result as HResult;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::io;
use std::sync::Mutex;

// Since we only compile this for testing (via #[cfg(test)])
// we dont really care that the ResourceHashMap is using an
// owned String. It makes the implementation simpler, and it
// only affects testing
type ResourceHashMap = Mutex<HashMap<String, Vec<u8>>>;

lazy_static! {
    static ref RESOURCES: ResourceHashMap = {
        let map = Mutex::new(HashMap::new());
        map
    };
}
/// Used by testing to completely reset the resources hashmap. This should be
/// executed before each test.
pub fn reset_resources() {
    let mut resources = RESOURCES.lock().unwrap();
    resources.clear();
}

/// The ResourceReaderWriter owns the key, which arguably forces us to
/// allocate more than we would like, but since this is only for testing
/// purposes, it seems silly to spend any time on optimizations that would
/// make the implementation more complicated, and eat up developer time.
#[derive(Debug)]
pub struct ResourceReaderWriter {
    key: String,
}

impl io::Read for ResourceReaderWriter {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut resources = RESOURCES.lock().unwrap();
        let results = resources.entry(self.key.to_string()).or_insert(Vec::new());
        let buf_len = buf.len();
        let results_len = results.len();
        let max_cnt = if buf_len < results_len {
            buf_len
        } else {
            results_len
        };
        for (idx, x) in results.iter().enumerate() {
            if idx == max_cnt - 1 {
                break;
            }
            buf[idx] = *x;
        }
        Ok(max_cnt as usize)
    }
}

impl io::Write for ResourceReaderWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let buf_len = buf.len();
        let mut resources = RESOURCES.lock().unwrap();
        resources
            .entry(self.key.to_string())
            .and_modify(|v| v.extend(buf))
            .or_insert(Vec::from(buf));
        Ok(buf_len)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
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
    type O = ResourceReaderWriter;
    //type E = HashitError;
    fn open<I>(&self, input: I) -> std::result::Result<Self::O, HashitError>
    where
        I: AsRef<str>,
    {
        Ok(ResourceReaderWriter {
            key: input.as_ref().to_string(),
        })
    }

    fn exists<I>(&self, input: I) -> bool
    where
        I: AsRef<str>,
    {
        RESOURCES.lock().unwrap().contains_key(input.as_ref())
    }
}

impl<'a> OpenMut<'a> for HtString {
    type OW = ResourceReaderWriter;
    fn open_mut<I>(
        &mut self,
        input: I,
        mode: OpenMode,
    ) -> std::result::Result<Self::OW, HashitError>
    where
        I: AsRef<str>,
    {
        //let mut resources = RESOURCES.lock().unwrap();
        match mode {
            OpenMode::WriteAppend => Ok(ResourceReaderWriter {
                key: input.as_ref().to_string(),
            }),
            OpenMode::WriteTruncate => {
                let mut resource = RESOURCES.lock().unwrap();
                if let Some(val) = resource.get_mut(input.as_ref()) {
                    val.clear();
                }
                Ok(ResourceReaderWriter {
                    key: input.as_ref().to_string(),
                })
            }
        }
    }

    fn create<I>(&mut self, _path: I) -> HResult<()>
    where
        I: AsRef<str>,
    {
        Ok(())
    }
}

impl<'a> FetchCachedHash<'a> for HtString {
    fn fetch_cached_hash(&mut self, input: &str) -> HResult<Vec<u8>> {
        // we dont have anything to open. The "cached hash" is simply the
        // hash of the input
        let mut resources = RESOURCES.lock().unwrap();
        match resources.get(input) {
            Some(resource) => Ok(resource.clone()),
            None => {
                resources.insert(input.to_string(), Vec::new());
                Ok(Vec::new())
            }
        }

        //Ok(blake_hash(input.as_bytes()))
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
