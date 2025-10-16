pub struct RichString {
    elements: Vec<Element>,
}

pub struct Element {
    text: String,
    attributes: Vec<Attribute>,
}

pub enum Attribute {
    Bold,
    Underline,
    Italic,
}
