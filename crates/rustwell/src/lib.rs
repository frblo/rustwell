use std::io::Read;

mod export;
mod parser;

pub mod rich_string;
pub mod screenplay;
pub use screenplay::Screenplay;

pub use export::export_html;

pub fn parse_str(src: &str) -> Screenplay {
    parser::parse(src)
}

pub fn parse_reader(mut r: impl Read) -> Screenplay {
    let mut src = String::new();
    r.read_to_string(&mut src).expect("Failed to read data");
    parser::parse(&src)
}

pub fn parse(src: impl AsRef<str>) -> Screenplay {
    parser::parse(src.as_ref())
}
