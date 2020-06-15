extern crate clap;
use clap::{App, Arg};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let opts = App::new("xkcd")
        .version("0.0.1")
        .author("vedranvinko")
        .arg(
            Arg::with_name("number")
                .long("number")
                .required(false)
                .short("n")
                .takes_value(true)
                .default_value("0")
                .help("Comic number to fetch (default latest)"),
        )
        .arg(
            Arg::with_name("output")
                .long("output")
                .required(false)
                .short("o")
                .takes_value(true)
                .default_value("text")
                .help("Print output in format: text/json"),
        )
        .arg(
            Arg::with_name("save")
                .long("save")
                .required(false)
                .short("s")
                .takes_value(true)
                .help("Save image to current directory"),
        )
        .get_matches();

    Ok(())
}