#![allow(dead_code)]

use crate::sln_raw::Sln;

mod sln_raw;

const SLN: &str = include_str!("../resources/vostok.sln");

fn main() {
    Sln::parse(SLN).unwrap();
}
