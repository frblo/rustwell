//! This module implements a [RichString], meaning a *rich* string which can have multiple
//! attributes for style, and can have these on different parts of the same string.
//!
//! # Examples
//!
//! ```
//! use rustwell::rich_string::RichString;
//!
//! let rs: RichString = "_Hello _**world!**".into();
//!
//! assert_eq!(rs.elements[0].text, "Hello ".to_string());
//! assert!(rs.elements[0].is_underline());
//! assert_eq!(rs.elements[1].text, "world!".to_string());
//! assert!(rs.elements[1].is_bold());
//! ```

use bitflags::bitflags;

/// A string that can have different parts styled.
///
/// New lines will always appear as their own non styled element.
/// The [RichString] is comprised of a collection of [Element]s that each hold a [String] and a
/// combination of stylings. The available styles are:
///
/// - `**bold**` → **bold**
/// - `*italic*` → *italic*
/// - `_underline_` → <u>underline</u>
///
/// as specified in the [Fountain specification](https://fountain.io/syntax/). Furthermore,
/// these can be combined in any overlapping order. Use `\` for a styling character to be
/// ignored for style parsing.
///
/// # Examples
///
/// ```
/// use rustwell::rich_string::RichString;
///
/// let mut rs = RichString::new();
/// rs.push_str("Hello **world!**");
///
/// assert_eq!(rs.elements[0].text, "Hello ".to_string());
/// assert_eq!(rs.elements[1].text, "world!".to_string());
/// assert!(rs.elements[1].is_bold());
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct RichString {
    pub elements: Vec<Element>,
}

impl RichString {
    /// Create a new, empty, [RichString].
    pub fn new() -> Self {
        RichString {
            elements: Vec::new(),
        }
    }

    /// Pushes a string onto the [RichString]. Will divide the string into multiple elements with
    /// different styles if input string can be parsed with styles.
    pub fn push_str(&mut self, str: impl AsRef<str>) {
        let s = str.as_ref();
        let bytes = s.as_bytes();

        let mut buf = String::new();
        let mut attrs = Attributes::empty();

        let mut i = 0;
        while i < bytes.len() {
            match bytes[i] {
                b'*' => {
                    if !buf.is_empty() {
                        self.push_run(std::mem::take(&mut buf), attrs);
                    }

                    let mut count = 0;
                    while i < bytes.len() && bytes[i] == b'*' && count < 3 {
                        count += 1;
                        i += 1;
                    }
                    match count {
                        1 => attrs ^= Attributes::ITALIC,
                        2 => attrs ^= Attributes::BOLD,
                        3 => attrs ^= Attributes::ITALIC | Attributes::BOLD,
                        _ => unreachable!("Count can't be increased further than 3"),
                    }
                }
                b'_' => {
                    if !buf.is_empty() {
                        self.push_run(std::mem::take(&mut buf), attrs);
                    }
                    attrs ^= Attributes::UNDERLINE;
                    i += 1;
                }
                b'\n' => {
                    if !buf.is_empty() {
                        self.push_run(std::mem::take(&mut buf), attrs);
                    }
                    attrs = Attributes::empty();
                    self.push_run('\n'.to_string(), Attributes::empty());
                    i += 1;
                }
                b'\\' => {
                    let back_slash = s[i..].chars().next().expect("Should be a valid charpoint");
                    i += back_slash.len_utf8();
                    // Discard the \

                    // Push the next char
                    let ch = s[i..].chars().next().expect("Should be a valid charpoint");
                    buf.push(ch);
                    i += ch.len_utf8();
                }
                _ => {
                    let ch = s[i..].chars().next().expect("Should be a valid charpoint");
                    buf.push(ch);
                    i += ch.len_utf8();
                }
            }
        }
        if !buf.is_empty() {
            self.push_run(std::mem::take(&mut buf), attrs);
        }
    }

    fn push_run(&mut self, text: String, attributes: Attributes) {
        if text.is_empty() {
            return;
        }

        if let Some(last) = self.elements.last_mut()
            && last.attributes == attributes
        {
            last.text.push_str(&text);
            return;
        }

        self.elements.push(Element { text, attributes });
    }
}

impl Default for RichString {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> From<T> for RichString
where
    T: AsRef<str>,
{
    fn from(str: T) -> Self {
        let mut out = RichString::new();
        out.push_str(str);
        out
    }
}

/// A [RichString] component, containing a [String] and the style attributes
/// belonging to said string.
#[derive(Debug, PartialEq, Eq, Clone, Default, Hash)]
pub struct Element {
    pub text: String,
    attributes: Attributes,
}

impl Element {
    /// Creates a new element based on a [String] with no attributes. Does not parse the
    /// string.
    pub fn new(text: String) -> Self {
        Self {
            text,
            attributes: Attributes::empty(),
        }
    }

    /// If the element is styled as bold.
    pub fn is_bold(&self) -> bool {
        self.attributes.contains(Attributes::BOLD)
    }

    /// If the element is styled as underline.
    pub fn is_underline(&self) -> bool {
        self.attributes.contains(Attributes::UNDERLINE)
    }

    /// If the element is styled as italic.
    pub fn is_italic(&self) -> bool {
        self.attributes.contains(Attributes::ITALIC)
    }
}

bitflags! {
    /// A bit array keeping track of style attributes for a [RichString].
    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
    struct Attributes: u8 {
        const BOLD      = 0b001;
        const UNDERLINE = 0b010;
        const ITALIC    = 0b100;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_bold() {
        let mut rs = RichString::new();
        rs.push_str("**text**");

        assert!(rs.elements[0].is_bold());
        assert_eq!(rs.elements[0].text, "text".to_string());
    }

    #[test]
    fn parses_italic() {
        let mut rs = RichString::new();
        rs.push_str("*text*");

        assert!(rs.elements[0].is_italic());
        assert_eq!(rs.elements[0].text, "text".to_string());
    }

    #[test]
    fn parses_underline() {
        let mut rs = RichString::new();
        rs.push_str("_text_");

        assert!(rs.elements[0].is_underline());
        assert_eq!(rs.elements[0].text, "text".to_string());
    }

    #[test]
    fn parses_overlapping_styles() {
        let mut rs = RichString::new();
        rs.push_str("t_e**x**t_");

        assert!(!rs.elements[0].is_bold());
        assert!(!rs.elements[0].is_italic());
        assert!(!rs.elements[0].is_underline());
        assert_eq!(rs.elements[0].text, "t".to_string());

        assert!(!rs.elements[1].is_bold());
        assert!(!rs.elements[1].is_italic());
        assert!(rs.elements[1].is_underline());
        assert_eq!(rs.elements[1].text, "e".to_string());

        assert!(rs.elements[2].is_bold());
        assert!(!rs.elements[2].is_italic());
        assert!(rs.elements[2].is_underline());
        assert_eq!(rs.elements[2].text, "x".to_string());

        assert!(!rs.elements[3].is_bold());
        assert!(!rs.elements[3].is_italic());
        assert!(rs.elements[3].is_underline());
        assert_eq!(rs.elements[3].text, "t".to_string());
    }

    #[test]
    fn parser_ignores_backslash() {
        let mut rs = RichString::new();
        rs.push_str(r#"\*text\*"#);

        assert!(!rs.elements[0].is_italic());
        assert_eq!(rs.elements[0].text, "*text*".to_string());
    }
}
