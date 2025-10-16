pub struct Screenplay {
    pub elements: Vec<Element>,
}

pub enum Element {
    Heading(String),
    Action(String),
    Dialogue {
        character: String,
        elements: Vec<DialogueElement>,
    },
}

pub enum DialogueElement {
    Paranthetical(String),
    Line(String),
}
