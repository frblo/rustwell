use crate::rich_string::RichString;

pub struct Screenplay {
    pub titlepage: Option<TitlePage>,
    pub elements: Vec<Element>,
}

pub enum Element {
    Heading {
        slug: RichString,
        number: Option<String>,
    },
    Action(RichString),
    Dialogue(Dialogue),
    DualDialogue(Dialogue, Dialogue),
    Lyrics(RichString),
    Transition(RichString),
    CenteredText(RichString),
    Note(RichString),
    PageBreak,
}

pub struct Dialogue {
    pub character: RichString,
    pub elements: Vec<DialogueElement>,
}

pub enum DialogueElement {
    Paranthetical(RichString),
    Line(RichString),
}

pub struct TitlePage {
    pub title: Vec<RichString>,
    pub credit: Vec<RichString>,
    pub authors: Vec<RichString>,
    pub source: Vec<RichString>,
    pub draft_date: Vec<RichString>,
    pub contact: Vec<RichString>,
}
