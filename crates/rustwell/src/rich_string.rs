use bitflags::bitflags;

#[derive(Debug)]
pub struct RichString {
    pub elements: Vec<Element>,
}

impl RichString {
    pub fn new() -> Self {
        RichString {
            elements: Vec::new(),
        }
    }

    pub fn push_run(&mut self, text: String, attributes: Attributes) {
        if text.is_empty() {
            return;
        }

        if let Some(last) = self.elements.last_mut() {
            if last.attributes == attributes {
                last.text.push_str(&text);
                return;
            }
        }

        self.elements.push(Element { text, attributes });
    }
}

impl<T> From<T> for RichString
where
    T: AsRef<str>,
{
    fn from(str: T) -> Self {
        let s = str.as_ref();
        let bytes = s.as_bytes();

        let mut out = RichString::new();
        let mut buf = String::new();
        let mut attrs = Attributes::empty();

        let mut i = 0;
        while i < bytes.len() {
            match bytes[i] {
                b'*' => {
                    if !buf.is_empty() {
                        out.push_run(std::mem::take(&mut buf), attrs);
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
                        out.push_run(std::mem::take(&mut buf), attrs);
                    }
                    attrs ^= Attributes::UNDERLINE;
                    i += 1;
                }
                b'\n' => {
                    if !buf.is_empty() {
                        out.push_run(std::mem::take(&mut buf), attrs);
                    }
                    attrs = Attributes::empty();
                    out.push_run('\n'.to_string(), Attributes::empty());
                    i += 1;
                }
                _ => {
                    let ch = s[i..].chars().next().expect("Should be a valid charpoint");
                    buf.push(ch);
                    i += ch.len_utf8();
                }
            }
        }
        if !buf.is_empty() {
            out.push_run(std::mem::take(&mut buf), attrs);
        }

        out
    }
}

#[derive(Debug)]
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

impl From<String> for RichString {
    fn from(str: String) -> Self {
        RichString {
            elements: vec![Element {
                text: str,
                attributes: Attributes::empty(),
            }],
        }
    }
}

impl From<&str> for RichString {
    fn from(str: &str) -> Self {
        RichString {
            elements: vec![Element {
                text: str.to_string(),
                attributes: Attributes::empty(),
            }],
        }
    }
}
