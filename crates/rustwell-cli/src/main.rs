use std::{env, fs};

use rustwell::process;

fn main() {
    let path = env::args().nth(1).expect("usage: rustwell <FILE>");
    let s = fs::read_to_string(&path).expect("failed to read file");
    process(&s);
}
