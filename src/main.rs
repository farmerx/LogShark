#![deny(warnings)]

// extern crate logshark;

#[cfg(unix)]
fn main() {
    println!("Hello, world, linux!");
}

#[cfg(windows)]
fn main() {
    println!("Hello, world, windows!");
}



