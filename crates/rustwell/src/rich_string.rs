//! This module implements a [`RichString`], meaning a *rich* string which can have multiple
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
use unicode_properties::{GeneralCategoryGroup, UnicodeGeneralCategory};

/// A string that can have different parts styled.
///
/// New lines will always appear as their own non styled element.
/// The [`RichString`] is comprised of a collection of [Element]s that each
/// hold a [String] and a combination of stylings. The available styles are:
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
    /// Create a new, empty, [`RichString`].
    pub fn new() -> Self {
        RichString {
            elements: Vec::new(),
        }
    }

    /// Pushes a string onto the [`RichString`]. Will divide the string into
    /// multiple elements with different styles if input string can be parsed with styles.
    pub fn push_str(&mut self, str: impl AsRef<str>) {
        let s = str.as_ref();
        let mut chars = s.chars().peekable();

        let mut buf = String::new();
        let mut attrs = Attributes::empty();

        let flush = |this: &mut Self, buf: &mut String, attrs: Attributes| {
            if !buf.is_empty() {
                this.push_run(std::mem::take(buf), attrs);
            }
        };

        while let Some(ch) = chars.next() {
            match ch {
                '*' => {
                    flush(self, &mut buf, attrs);
                    let mut count = 1;
                    while count < 3 && chars.peek() == Some(&'*') {
                        chars.next();
                        count += 1;
                    }
                    match count {
                        1 => attrs ^= Attributes::ITALIC,
                        2 => attrs ^= Attributes::BOLD,
                        3 => attrs ^= Attributes::ITALIC | Attributes::BOLD,
                        _ => unreachable!("Count can't be increased further than 3"),
                    }
                }
                '_' => {
                    flush(self, &mut buf, attrs);
                    attrs ^= Attributes::UNDERLINE;
                }
                '\n' => {
                    flush(self, &mut buf, attrs);
                    attrs = Attributes::empty();
                    self.push_run('\n'.to_string(), Attributes::empty());
                }
                '\\' => {
                    if let Some(next) = chars.next() {
                        buf.push(next);
                    }
                }
                _ => buf.push(ch),
            }
        }
        flush(self, &mut buf, attrs);
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

    fn tokenize(input: &str) -> (Vec<&str>, Vec<Delimiter>) {
        let mut tokens = Vec::new();
        let mut delimiters = Vec::new();

        let mut chars = input.char_indices().peekable();
        let mut start = 0;
        let mut before = None;

        while let Some((i, ch)) = chars.next() {
            match ch {
                '*' | '_' => {
                    if i > start {
                        tokens.push(&input[start..i]);
                    }

                    let run_start = i;
                    while chars.peek().map(|(_, c)| *c == ch).unwrap_or(false) {
                        chars.next();
                    }
                    let run_end = chars.peek().map(|(i, _)| *i).unwrap_or(input.len());
                    let after = chars.peek().map(|(_, c)| *c);
                    let count = run_end - run_start;

                    delimiters.push(Delimiter {
                        char: ch,
                        count,
                        token_idx: tokens.len(),
                        can_open: is_left_flanking(before, after),
                        can_close: is_right_flanking(before, after),
                    });

                    tokens.push(&input[run_start..run_end]);
                    before = Some(ch);
                    start = run_end;
                }
                '\\' => {
                    if let Some((next_idx, next)) = chars.next() {
                        if i > start {
                            tokens.push(&input[start..i]);
                        }
                        before = Some(next);
                        start = next_idx;
                    }
                }
                _ => before = Some(ch),
            }
        }

        if start < input.len() {
            tokens.push(&input[start..]);
        }

        (tokens, delimiters)
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

/// A [`RichString`] component, containing a [String] and the style attributes
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

#[derive(Debug, PartialEq, Eq)]
struct Delimiter {
    char: char,
    count: usize,
    token_idx: usize,
    can_open: bool,
    can_close: bool,
}

fn is_left_flanking(before: Option<char>, after: Option<char>) -> bool {
    match after {
        None => false,
        Some(a) if is_whitespace(a) => false,
        Some(a) if is_punctuation(a) => match before {
            None => true,
            Some(b) if is_whitespace(b) || is_punctuation(b) => true,
            _ => false,
        },
        _ => true,
    }
}

fn is_right_flanking(before: Option<char>, after: Option<char>) -> bool {
    // right-flanking delimiter run is checked the same way as a left-flanking
    // but going from the other direction.
    is_left_flanking(after, before)
}

fn is_whitespace(char: char) -> bool {
    match char {
        '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{000D}' => true,
        c => matches!(c.general_category_group(), GeneralCategoryGroup::Separator),
    }
}

fn is_punctuation(char: char) -> bool {
    matches!(
        char.general_category_group(),
        GeneralCategoryGroup::Punctuation | GeneralCategoryGroup::Symbol
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    mod tokenize {
        use super::*;

        #[test]
        fn splits_at_delimiter_run() {
            let (tokens, delimiter) = RichString::tokenize("* a _ b **");
            let expected_tokens = vec!["*", " a ", "_", " b ", "**"];
            let expected_delimiter = vec![
                Delimiter {
                    char: '*',
                    count: 1,
                    token_idx: 0,
                    can_open: false,
                    can_close: false,
                },
                Delimiter {
                    char: '_',
                    count: 1,
                    token_idx: 2,
                    can_open: false,
                    can_close: false,
                },
                Delimiter {
                    char: '*',
                    count: 2,
                    token_idx: 4,
                    can_open: false,
                    can_close: false,
                },
            ];
            assert_eq!(tokens, expected_tokens);
            assert_eq!(delimiter, expected_delimiter);
        }

        #[test]
        fn left_flanking() {
            // Can open
            let (_, delimiter) = RichString::tokenize("**a");
            assert!(
                delimiter
                    .first()
                    .expect("There should be a delimiter")
                    .can_open
            );
            let (_, delimiter) = RichString::tokenize("*.a");
            assert!(
                delimiter
                    .first()
                    .expect("There should be a delimiter")
                    .can_open
            );
            let (_, delimiter) = RichString::tokenize(".*.a");
            assert!(
                delimiter
                    .first()
                    .expect("There should be a delimiter")
                    .can_open
            );

            // Can't open
            let (_, delimiter) = RichString::tokenize("* a");
            assert!(
                !delimiter
                    .first()
                    .expect("There should be a delimiter")
                    .can_open
            );
            let (_, delimiter) = RichString::tokenize("a*.a");
            assert!(
                !delimiter
                    .first()
                    .expect("There should be a delimiter")
                    .can_open
            );
        }

        #[test]
        fn right_flanking() {
            // Can open
            let (_, delimiter) = RichString::tokenize("a**");
            assert!(
                delimiter
                    .first()
                    .expect("There should be a delimiter")
                    .can_close
            );
            let (_, delimiter) = RichString::tokenize("a.*");
            assert!(
                delimiter
                    .first()
                    .expect("There should be a delimiter")
                    .can_close
            );
            let (_, delimiter) = RichString::tokenize("a.*.");
            assert!(
                delimiter
                    .first()
                    .expect("There should be a delimiter")
                    .can_close
            );

            // Can't open
            let (_, delimiter) = RichString::tokenize("a *");
            assert!(
                !delimiter
                    .first()
                    .expect("There should be a delimiter")
                    .can_close
            );
            let (_, delimiter) = RichString::tokenize("a.*a");
            assert!(
                !delimiter
                    .first()
                    .expect("There should be a delimiter")
                    .can_close
            );
        }

        #[test]
        fn dont_include_escape_character() {
            let (tokens, delimiter) = RichString::tokenize("a\\*b");
            // Doesn't create a delimiter run for the escaped character
            assert!(delimiter.is_empty());
            // The backslash isn't included as a token
            for token in tokens {
                assert!(!token.contains('\\'))
            }
        }
    }

    mod parse {
        use super::*;

        const B: Attributes = Attributes::BOLD;
        const I: Attributes = Attributes::ITALIC;
        const U: Attributes = Attributes::UNDERLINE;
        const E: Attributes = Attributes::empty();

        macro_rules! test_emphasis {
            ($name:ident, $input:expr, [$(($text:expr, $attrs:expr)),*]) => {
                #[test]
                fn $name() {
                test_parse($input, [$(($text, $attrs)),*]);
                }
            };
        }

        fn test_parse<'a>(input: &str, expected: impl IntoIterator<Item = (&'a str, Attributes)>) {
            let rs = RichString::from(input);
            for (elem, expected) in rs.elements.iter().zip(expected.into_iter()) {
                assert_eq!(elem.text, expected.0);
                assert_eq!(elem.attributes, expected.1);
            }
        }

        // Basic
        test_emphasis!(italic, "*foo bar*", [("foo bar", I)]);
        test_emphasis!(bold, "**foo bar**", [("foo bar", B)]);
        test_emphasis!(bold_italic, "***foo bar***", [("foo bar", B | I)]);
        test_emphasis!(underline, "_foo bar_", [("foo bar", U)]);

        // Non left-flanking delimiter run not opening
        test_emphasis!(
            not_open_because_whitespace_after_delimiter,
            "* foo bar*",
            [("* foo bar*", E)]
        );
        test_emphasis!(
            not_open_because_punctuation_after_delimiter_alphanumeric_before,
            "a*.foo bar*",
            [("a*.foo bar*", E)]
        );

        // Non right-flanking delimiter run not closing
        test_emphasis!(
            not_closed_because_whitespace_before_delimiter,
            "*foo bar *",
            [("*foo bar *", E)]
        );
        test_emphasis!(
            not_closed_because_newline_before_delimiter,
            "*foo bar\n*",
            [("*foo bar\n*", E)]
        );
        test_emphasis!(
            not_closed_because_punctuation_before_delimiter_alphanumeric_after,
            "*(*foo)",
            [("*(*foo)", E)]
        );

        test_emphasis!(
            closed_because_newline_then_alphanumeric_before_delimiter,
            "*foo\nbar*",
            [("foo\nbar", I)]
        );

        // Nested empgasis
        test_emphasis!(
            nested_bold_in_italics,
            "*foo **bar** baz*",
            [("foo ", I), ("bar", I | B), (" baz", I)]
        );
        test_emphasis!(
            nested_bold_in_italics_no_whitepace,
            "*foo**bar**baz*",
            [("foo", I), ("bar", I | B), ("baz", I)]
        );
        test_emphasis!(
            nested_bold_in_italics_complicated,
            "*foo**bar***",
            [("foo", I), ("bar", I | B)]
        );

        // matching delimiter runs
        test_emphasis!(no_empty_emphasis, "__foo", [("__foo", E)]);
        test_emphasis!(
            cant_close_when_sum_is_multiple_of_three_but_not_both_lengths_are_multiples_of_three,
            "*foo**bar*",
            [("foo**bar", I)]
        );
        test_emphasis!(
            can_close_when_sum_is_multiple_of_three_and_both_lengths_are_multiples_of_three,
            "foo***bar***baz",
            [("foo", E), ("bar", I | B), ("baz", E)]
        );

        test_emphasis!(
            literal_delimiter_cant_appear_at_begining_or_end_of_run,
            "foo *** foo *\\**",
            [("foo *** foo ", E), ("*", I)]
        );
        test_emphasis!(mismatch_more_before, "**foo*", [("*", E), ("foo", I)]);
        test_emphasis!(mismatch_more_after, "*foo****", [("foo", I), ("***", E)]);
        test_emphasis!(
            two_potential_opening_share_same_closing_pick_shortest,
            "**foo **bar baz**",
            [("**foo ", E), ("bar baz", B)]
        );
    }
}
