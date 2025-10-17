use bitflags::bitflags;

pub struct RichString {
    pub elements: Vec<Element>,
}

pub struct Element {
    pub text: String,
    attributes: Attributes,
}

impl Element {
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
    pub struct Attributes: u8 {
        const BOLD      = 0b001;
        const UNDERLINE = 0b010;
        const ITALIC    = 0b100;
    }
}
