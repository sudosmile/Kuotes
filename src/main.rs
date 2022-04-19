use serde_json;
/// get kanye quotes from https://api.kanye.rest/

use structopt::StructOpt;
use std::{path::PathBuf, error::Error};

#[derive(StructOpt)]
#[structopt(name = "kuotes", about = "Get random Kanye West quotes (aka \'kuotes\')")]
struct Opt {
    /// Clear screen before printing anything
    #[structopt(
        short = "c", 
        long = "clear",
    )]
    clear: bool,

    /// disable double quotes around the kuote(s)
    #[structopt(
        short = "q", 
        long = "quotes",
    )]
    quotes: bool,

    /// Number of kuotes to get (1 by default)
    #[structopt(
        short = "n", 
        long = "count",
    )]
    number: Option<i32>,

    /// Output file, stdout if not present
    #[structopt(parse(from_os_str))]
    output: Option<PathBuf>,
}

async fn get_kuote(client: &reqwest::Client) -> Result<String, Box<dyn Error>> {
    let response = client
        .get("https://api.kanye.rest/")
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let json_kuote = response.get("quote");
    serde_json::to_string(&json_kuote).map_err(|e| e.into())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Opt::from_args();
    let client = reqwest::Client::new();

    let n = match args.number {
        Some(n) => n,
        None => 1,
    };

    // get the necessary number of kuotes from the api, using an async function
    let mut kuotes: Vec<String> = Vec::new();
    for _ in 0..n {
        match get_kuote(&client).await {
            Ok(s) => kuotes.push(
                if args.quotes {
                    let mut str = s;
                    // remove quotes, which are the first and last character
                    str.remove(0);
                    str.pop();
                    str
                } else {
                    s
                }
            ),
            Err(e) => return Err(e),
        }
    }

    // join the kuotes into a single string
    let kuotes = kuotes.join("\n");

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

