use std::path;

use structopt::StructOpt;

use kuotes::Result;

#[derive(StructOpt)]
#[structopt(name = "kuotes", about = "Get random Kanye West quotes (aka \'kuotes\')")]
struct Opt {
    /// Clear screen before printing anything (only on stdout)
    #[structopt(
        short = "c", 
        long = "clear",
    )]
    clear: bool,

    /// Disable double quotes around the kuote(s)
    #[structopt(
        short = "q", 
        long = "quotes",
    )]
    quotes: bool,

    /// Number of kuotes to get
    #[structopt(
        short = "n", 
        long = "count",
        default_value = "1",
    )]
    number: i32,

    /// Output file, stdout if not present
    #[structopt(parse(from_os_str))]
    output: Option<path::PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Opt::from_args();

    let kuotes = kuotes::get(args.number, args.quotes).await?;

    // print the kuotes to path or stdout if no output file given
    if let Some(path) = args.output {
        if let Err(e) = std::fs::write(&path, kuotes) {
            return Err(e.into())
        }
    } else {
        if args.clear {
            clearscreen::clear()?;
        }
        println!("{kuotes}");
    }
    Ok(())
}

