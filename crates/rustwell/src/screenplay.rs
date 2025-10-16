use crate::rich_string::RichString;

pub struct Screenplay {
    pub elements: Vec<Element>,
}

pub enum Element {
    Heading(String),
    Action(RichString),
    Dialogue {
        character: String,
        elements: Vec<DialogueElement>,
    },
}

pub enum DialogueElement {
    Paranthetical(RichString),
    Line(RichString),
}
