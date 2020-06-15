extern crate clap;
use clap::{App, Arg};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Response {
    month: String,
    num: i32,
    link: String,
    year: String,
    news: String,
    safe_title: String,
    transcript: String,
    alt: String,
    img: String,
    title: String,
    day: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Comic {
    title: String,
    number: i32,
    date: String,
    description: String,
    image: String,
}

impl Response {
    fn title(&self) -> String {
        self.title.to_string()
    }

    fn number(&self) -> String {
        self.num.to_string()
    }

    fn date(&self) -> String {
        format!("{}/{}/{}", self.day, self.month, self.year)
    }

    fn description(&self) -> String {
        self.alt.to_string()
    }

    fn image(&self) -> String {
        self.img.to_string()
    }
}
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