use chrono::prelude::*;
use clap::{App, Arg};
use reqwest;
use std::fs;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

fn main() -> Result<(), std::io::Error> {
    let matches = App::new("Linz Port Webcam Scraper")
        .version("0.1.0")
        .author("Dominik Mydlil <dominik.mydlil@outlook.com>")
        .arg(
            Arg::with_name("download-dir")
                .short("d")
                .long("download-dir")
                .value_name("DIRECTORY")
                .help("Sets the directory that the pictures should be downloaded to.")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let download_path = Path::new(matches.value_of("download-dir").unwrap()).canonicalize()?;
    if is_webcam_active() {
        match fetch_image() {
            Ok(img) => save_image(&img, &download_path)?,
            Err(e) => {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
            }
        }
    } else {
        println!("Webcam is not currently active.");
    }
    Ok(())
}

fn is_webcam_active() -> bool {
    let now = DateTime::with_timezone(&Utc::now(), &chrono_tz::Europe::Vienna);
    let time = now.time();
    time >= NaiveTime::from_hms(6, 0, 0) && time <= NaiveTime::from_hms(18, 10, 0)
}

fn fetch_image() -> Result<Vec<u8>, reqwest::Error> {
    let mut response = reqwest::get("https://www.linzag.at/multimedia/neuland/webcamneuland.jpg")?;
    let mut buf: Vec<u8> = vec![];
    response.copy_to(&mut buf)?;
    Ok(buf)
}

fn save_image<P: AsRef<Path>>(img: &[u8], download_path: &P) -> std::io::Result<()> {
    let now = DateTime::with_timezone(&Utc::now(), &chrono_tz::Europe::Vienna);

    let mut pb = PathBuf::new();
    pb.push(download_path);
    pb.push(format!("webcamneuland.{}.jpg", now.format("%Y%m%dT%H%M%S")));
    let filepath = pb.as_path();

    let mut buffer = fs::File::create(filepath)?;
    buffer.write_all(img)?;

    println!("Image saved to {}.", filepath.display());
    Ok(())
}
