use std::io::Read;

mod export;
mod parser;

pub mod rich_string;
pub mod screenplay;
pub use screenplay::Screenplay;

pub use export::export_html;

pub fn parse(mut r: impl Read) -> Screenplay {
    let mut src = String::new();
    r.read_to_string(&mut src).expect("Failed to read data");
    parser::parse(&src)
}
