//! This crate is an implementation of the [Fountain](https://fountain.io/) screenplay markup
//! language specification, and acts as a tool for parsing and exporting the *Fountain*
//! language to different kinds of formatted documents, such as `html` and `pdf`.
//!
//! The crate can be used both for the whole compilation process, but also as an intermediary step
//! for parsing *Fountain* text into an abstract syntax tree (AST) that can then be used when
//! constructing a new exporter.
//!
//! # Examples
//!
//! ```rust
//! use std::io::BufWriter;
//! use std::io::stdout;
//!
//! fn main() {
//!     let script = r#"
//!     Title: Example Screenplay
//!
//!     INT. HOUSE – DAY
//!
//!     Someone is knocking on the door.
//!
//!     GUEST
//!     (tired)
//!     Hey, open up!
//!     "#;
//!
//!     let parsed = parse_str(script);
//!     let mut output = BufWriter::new(stdout());
//!
//!     export_html(&parsed, &mut output, false);
//! }
//! ```

use std::io::Read;

mod export;
mod parser;

pub mod rich_string;
pub mod screenplay;
pub use screenplay::Screenplay;

pub use export::export_html;
pub use export::export_pdf;
pub use export::export_typst;

/// Parses a Fountain source string into a [Screenplay] structure.
///
/// Preprocesses the source text by removing
/// comments and normalizing tabs to spaces.
///
/// # Examples
///
/// ```
/// let input = r#"
/// Title: Example Screenplay
///
/// INT. ROOM – DAY
/// A man stands alone.
/// "#;
///
/// let screenplay = parse(input);
/// assert!(screenplay.elements.len() > 0);
/// ```
pub fn parse(src: impl AsRef<str>) -> Screenplay {
    parser::parse(src.as_ref())
}

/// Parses a Fountain source file into a [Screenplay] structure.
pub fn parse_reader(mut r: impl Read) -> Screenplay {
    let mut src = String::new();
    r.read_to_string(&mut src).expect("Failed to read data");
    parser::parse(&src)
}
