// [package]
// name = "ch12"
// version = "0.1.0"
// edition = "2021"

// # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

// [dependencies]
// libc = "0.2"

#![cfg(not(windows))]

use libc::{SIGTERM, SIGUSR1};
use std::thread::sleep;
use std::time::Duration;

static mut SHUT_DOWN: bool = false;

fn main() {
    register_signal_handlers();

    let delay = Duration::from_secs(1);

    for i in 1usize.. {
        println!("{i}");
        unsafe {
            if SHUT_DOWN {
                println!("*");
                return;
            }
        }

        sleep(delay);

        let signal = if i > 2 { SIGTERM } else { SIGUSR1 };

        unsafe {
            libc::raise(signal);
        }
    }

    unreachable!()
}

fn register_signal_handlers() {
    unsafe {
        libc::signal(SIGTERM, handle_sigterm as usize);
        libc::signal(SIGUSR1, handle_sigusr1 as usize);
    }
}

#[allow(dead_code)]
fn handle_sigterm(_signal: i32) {
    register_signal_handlers();
    println!("SIGTERM");

    unsafe {
        SHUT_DOWN = true;
    }
}

#[allow(dead_code)]
fn handle_sigusr1(_signal: i32) {
    register_signal_handlers();
    println!("SIGUSR1");
}
