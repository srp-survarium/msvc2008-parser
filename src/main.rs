#![allow(dead_code)]

use crate::sln_raw::Sln;

mod sln_raw;

const SLN: &str = include_str!("../resources/vostok.sln");

fn main() {
    let (i, sln) = Sln::parse(SLN).unwrap();

    println!("{:#?}", sln);

    println!("\nINPUT LEFT:");
    println!("{}", i);
}
