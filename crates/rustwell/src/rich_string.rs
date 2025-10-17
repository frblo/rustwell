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

    pub fn from(text: &str) -> Self {
        let mut elements = Vec::new();
        elements.push(Element::new(text.to_owned()));

        RichString { elements }
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
    #[derive(Debug)]
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
