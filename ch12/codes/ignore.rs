// [package]
// name = "ch12"
// version = "0.1.0"
// edition = "2021"

// # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

// [dependencies]
// libc = "0.2"

use libc::{raise, signal, SIGTERM, SIG_DFL, SIG_IGN};

fn main() {
    unsafe {
        signal(SIGTERM, SIG_IGN);
        raise(SIGTERM);
    }

    println!("ok");

    unsafe {
        signal(SIGTERM, SIG_DFL);
        raise(SIGTERM);
    }

    println!("not ok");
}
