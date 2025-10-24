use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;

fn main() {
    let path = env::args().nth(1).expect("usage: rustwell <FILE>");
    let file = File::open(&path).expect("Failed to open file");
    let mut r = BufReader::new(file);

    let screenplay = rustwell::parse(&mut r);
    let mut bw = BufWriter::new(std::io::stdout());
    rustwell::export_html(&screenplay, &mut bw, true);
}
