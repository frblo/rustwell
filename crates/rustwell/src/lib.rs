mod export;
mod parser;
mod rich_string;
mod screenplay;

pub fn process(input: &str) {
    let sp = parser::parse(input);
}
