// [package]
// name = "ch12"
// version = "0.1.0"
// edition = "2021"

// # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

// [dependencies]
// rand = "0.6"

use rand;

static mut SHUT_DOWN: bool = false;

fn main() {
    loop {
        unsafe {
            SHUT_DOWN = rand::random();
        }
        print!(".");

        if unsafe { SHUT_DOWN } {
            break;
        }
    }
    println!()
}
