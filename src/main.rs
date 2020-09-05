use blake2::{Blake2b, Digest};
use hex::encode;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use structopt::StructOpt;
mod error;
use error::HashitError;
use error::HashitResult;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short, long, parse(from_os_str))]
    outpath: Option<PathBuf>,
    #[structopt(short, long, parse(from_os_str))]
    sources: Vec<PathBuf>,
}

fn read_file<I>(path: I) -> HashitResult<Vec<u8>>
where
    I: AsRef<std::path::Path>,
{
    let path = path.as_ref();
    let mut f = File::open(path)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn blake_hash(input: &[u8]) -> Vec<u8> {
    let mut hasher = Blake2b::new();
    hasher.update(input);
    let res = hasher.finalize();
    res.to_vec()
}
use std::fs;

fn has_changed<P>(hash: &[u8], output_file: P) -> HashitResult<bool>
where
    P: AsRef<Path>,
{
    let output_file = output_file.as_ref();
    if !output_file.exists() {
        // create missing directory if it doesnt exist
        let output_path = output_file
            .parent()
            .ok_or_else(|| HashitError::MissingDir(output_file.to_string_lossy().into_owned()))?;
        fs::create_dir_all(output_path)?;
        // write hash to file
        let mut file = File::create(&output_file)?;
        file.write_all(hash)?;
        // has the input file changed? Well if there is no output path
        // then automatically that would be true
        return Ok(true);
    }
    // read contents and determine if hashes are equal
    let mut buffer = Vec::<u8>::new();
    {
        // create scope. file will be dropped
        let mut file = File::open(output_file)?;
        file.read_to_end(&mut buffer)?;
    }
    let differs = hash != &buffer[..];
    if differs {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(output_file)?;
        file.write_all(&hash)?;
    }
    Ok(differs)
}

fn calc_hash<P>(files: &[P]) -> HashitResult<Vec<u8>>
where
    P: AsRef<Path>,
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
fn main() -> HashitResult<()> {
    let opt = Opt::from_args();

    let resvec = calc_hash(&opt.sources[..])?;

    if let Some(outpath) = opt.outpath {
        println!("Has file changed? {}", has_changed(&resvec[..], &outpath)?);
    } else {
        let encoded = encode(&resvec);
        println!("Encoded:  {}", &encoded);
    }
    Ok(())
}
