//use hashtest::has_changed;
use hashtest::Hashit;
use hashtest::Result as HtResult;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short, long, parse(from_os_str))]
    outpath: PathBuf,
    #[structopt(short, long, parse(from_os_str))]
    sources: Vec<PathBuf>,
}

fn main() -> HtResult<()> {
    let opt = Opt::from_args();
    let mut hashit = Hashit::new();
    println!(
        "Has file changed? {}",
        hashit.has_changed(&opt.sources[..], &opt.outpath)?
    );

    Ok(())
}
