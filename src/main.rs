#![allow(dead_code)]

use crate::sln_raw::Project;

mod sln_raw;

const SLN: &str = include_str!("../resources/vostok.sln");

fn main() {
    Project::parse(SLN).unwrap();
}
