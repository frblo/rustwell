use std::io::BufWriter;

mod export;
mod parser;
mod rich_string;
mod screenplay;

pub fn process(input: &str) {
    let sp = parser::parse(input);
    let mut bw = BufWriter::new(std::io::stdout());
    export::export_html(&sp, &mut bw, true);
}
