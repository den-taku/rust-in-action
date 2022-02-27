use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::PathBuf;

const BYTES_PER_LINE: usize = 16;

fn main() {
    let arg1 = env::args().nth(1);

    let fname = arg1.expect("usage: fview FILENAME");

    let mut f = File::open(&fname).expect("Unable to open file");
    let mut pos = 0;
    let mut buffer = [0; BYTES_PER_LINE];

    while let Ok(_) = f.read_exact(&mut buffer) {
        print!("[0x{pos:08x}] ");
        for byte in &buffer {
            match *byte {
                0x00 => print!(".  "),
                0xff => print!("## "),
                _ => print!("{byte:02x} "),
            }
        }
        println!();
        pos += BYTES_PER_LINE;
    }

    let f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(&fname);
    println!("{f:?}");

    let hello = PathBuf::from("/tmp/hello.txt");
    println!("{hello:?}");
    println!("{:?}", hello.extension());
}
