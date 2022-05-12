/* use std::io;
use std::collections::HashMap; */
use std::io; //::BufReader;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short = 'u', long = "url")]
    url: String,

    #[clap(short = 'f', long = "file")]
    path: std::path::PathBuf,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

#[tokio::main]
async fn main() -> Result<(),reqwest::Error> {
    let args = Args::parse();
    let content = std::fs::read_to_string(&args.path)
        .expect("could not read file");
    let client = reqwest::Client::new();

    let reader = io::stdin();
    let mut input_text = String::new();
    reader.read_line(&mut input_text).expect("Error reading");

    //io::Box::new(File::open(path).expect("could not open file"));
    let res = client.get(args.url).send().await?;

    for _ in 0..args.count {
        if res.status() == 200 {} {
            println!("{}", res.status());
        }
    }
    Ok(())
}