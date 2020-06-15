extern crate clap;
use clap::{App, Arg};

extern crate image;

use serde::{Deserialize, Serialize};

use std::fs::File;
use std::path::Path;

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

    let n = opts.value_of("number").unwrap().parse::<i32>().unwrap();
    let out = opts.value_of("output").unwrap();

    let url = match n {
        0 => String::from("https://xkcd.com/info.0.json"),
        _ => format!("https://xkcd.com/{}/info.0.json", n),
    };

    let res = reqwest::get(&url).await?;
    let body = res.text().await?;

    let rsp: Response = serde_json::from_str(&body).unwrap();
    let title = rsp.title();
    let number = rsp.number();
    let date = rsp.date();
    let description = rsp.description();
    let image = rsp.image();
    let payload = format!(
        "\nTitle: {}\nComic No: {}\nDate: {}\nDescription: {}\nImage: {}",
        title, number, date, description, image
    );

    match out {
        "text" => {
            println!("{}", payload);
        }
        "json" => {
            println!("{}", body);
        }
        _ => {}
    }

    match opts.is_present("save") {
        true => match opts.value_of("save").unwrap() {
            "true" => {
                let res = reqwest::get(&rsp.image()).await?;
                let body = res.bytes().await?;
                let img = image::load_from_memory(&body).unwrap();

                let fout =
                    &mut File::create(&Path::new(&format!("xkcd-{}-{}", title, number))).unwrap();

                img.write_to(fout, image::ImageFormat::Png).unwrap();
            }
            "false" => {}
            _ => {}
        },
        false => {}
    }

    Ok(())
}
