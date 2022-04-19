/// get kanye quotes from https://api.kanye.rest/

use structopt::StructOpt;
use std::{path::PathBuf, error::Error};

#[derive(StructOpt)]
#[structopt(name = "kotes", about = "Get random kanye quotes (aka \'Kotes\')")]
struct Opt {
    /// Clear screen before printing anything
    #[structopt(
        short = "c", 
        long = "clear",
    )]
    clear: bool,

    /// Number of Kotes to get
    #[structopt(
        short = "n", 
        long = "count",
    )]
    number: Option<i32>,

    /// Output file, stdout if not present
    #[structopt(parse(from_os_str))]
    output: Option<PathBuf>,
}

async fn get_kote(client: &reqwest::Client) -> Result<String, Box<dyn Error>> {
    let response = client
        .get("https://api.kanye.rest/")
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    serde_json::to_string(response.get("quote").unwrap())
        .map_err(|e| e.into())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Opt::from_args();
    let client = reqwest::Client::new();

    if args.clear {
        clearscreen::clear()?;
    }

    let n = match args.number {
        Some(n) => n,
        None => 1,
    };

    // get the necessary number of Kotes from the api, using an async function
    let mut kotes: Vec<String> = Vec::new();
    for _ in 0..n {
        match get_kote(&client).await {
            Ok(s) => kotes.push(s),
            Err(e) => return Err(e),
        }
    }
    let kotes = kotes.join("\n");

    // print the kotes to path or stdout if no output file given
    if let Some(path) = args.output {
        if let Err(e) = std::fs::write(&path, kotes) {
            return Err(e.into())
        }
    } else {
        println!("{kotes}");
    }
    Ok(())
}

