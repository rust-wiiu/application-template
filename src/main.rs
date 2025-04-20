#![no_std]
#![no_main]

use wut;
use wut::prelude::*;
use wut::time::Duration;

#[wut::main(Console)]
fn main() {
    while wut::process::running() {
        println!("{}", wut::time::DateTime::now());
        wut::thread::sleep(Duration::from_secs(1));
    }
}
