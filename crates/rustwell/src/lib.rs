use std::{fs::File, io::BufWriter};

mod export;
mod parser;
mod rich_string;
mod screenplay;

pub use parser::parse;

pub fn process(input: &str) {
    let sp = parser::parse(input);
    // let mut bw = BufWriter::new(std::io::stdout());
    // export::export_html(&sp, &mut bw, true);
    let file = File::create("s.pdf").unwrap();
    let mut fw = BufWriter::new(file);
    export::export_pdf(&sp, &mut fw);
}
