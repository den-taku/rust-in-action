// [package]
// name = "ch08"
// version = "0.1.0"
// edition = "2021"

// # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

// [dependencies]
// reqwest = "0.9"

use reqwest;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://www.rustinaction.com/";
    let mut response = reqwest::get(url)?;

    let content = response.text()?;
    println!("{content}");
    Ok(())
}
