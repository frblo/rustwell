use std::hash::Hash;

use bitflags::bitflags;

#[derive(Debug, PartialEq, Eq, Clone)]
/// A [String] that can have different parts styled.
///
/// New lines will always appear as their own non syled element.
pub struct RichString {
    pub elements: Vec<Element>,
}

impl RichString {
    pub fn new() -> Self {
        RichString {
            elements: Vec::new(),
        }
    }

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

impl Hash for RichString {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let s: String = self.into();
        s.hash(state);
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

impl From<&RichString> for String {
    fn from(rs: &RichString) -> Self {
        rs.elements.iter().map(|s| s.text.clone()).collect()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Element {
    pub text: String,
    attributes: Attributes,
}

impl Element {
    pub fn new(text: String) -> Self {
        Self {
            text,
            attributes: Attributes::empty(),
        }
    }

    pub fn is_bold(&self) -> bool {
        self.attributes.contains(Attributes::BOLD)
    }

    pub fn is_underline(&self) -> bool {
        self.attributes.contains(Attributes::UNDERLINE)
    }

    pub fn is_italic(&self) -> bool {
        self.attributes.contains(Attributes::ITALIC)
    }
}

bitflags! {
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct Attributes: u8 {
        const BOLD      = 0b001;
        const UNDERLINE = 0b010;
        const ITALIC    = 0b100;
    }
}
