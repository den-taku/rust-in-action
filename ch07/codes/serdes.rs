// [package]
// name = "ch07"
// version = "0.1.0"
// edition = "2021"

// # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

// [dependencies]
// bincode = "1.3.3"
// serde = "1.0.136"
// serde_cbor = "0.11.2"
// serde_derive = "1.0.136"
// serde_json = "1.0.79"

use bincode::serialize as to_bincode;
use serde_cbor::to_vec as to_cbor;
use serde_derive::Serialize;
use serde_json::to_string as to_json;

#[derive(Serialize)]
struct City {
    name: String,
    population: usize,
    latitude: f64,
    longitude: f64,
}

fn main() {
    let calabar = City {
        name: String::from("Calabar"),
        population: 470_000,
        latitude: 4.95,
        longitude: 8.33,
    };

    let as_json = to_json(&calabar).unwrap();
    let as_cbor = to_cbor(&calabar).unwrap();
    let as_bincode = to_bincode(&calabar).unwrap();

    println!("json:\n{as_json}\n");
    println!("cbor:\n{as_cbor:?}\n");
    println!("bincode:\n{as_bincode:?}\n");

    println!(
        "json (as UTF-8):\n{}\n",
        String::from_utf8_lossy(&as_json.as_bytes())
    );
    println!("cbor (as UTF-8):\n{}\n", String::from_utf8_lossy(&as_cbor));
    println!(
        "bincode (as UTF-8):\n{}\n",
        String::from_utf8_lossy(&as_bincode)
    );
}
