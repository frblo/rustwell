use crate::rich_string::RichString;

#[derive(Debug)]
pub struct Screenplay {
    pub titlepage: Option<TitlePage>,
    pub elements: Vec<Element>,
}

impl Screenplay {
    pub fn new() -> Self {
        Self {
            titlepage: None,
            elements: Vec::new(),
        }
    }

    pub fn set_titlepage(&mut self, titlepage: Option<TitlePage>) {
        self.titlepage = titlepage;
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Dialogue {
    pub character: RichString,
    pub elements: Vec<DialogueElement>,
}

impl Dialogue {
    pub fn new() -> Self {
        Self {
            character: RichString::new(),
            elements: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub enum DialogueElement {
    Parenthetical(RichString),
    Line(RichString),
}

#[derive(Debug)]
pub struct TitlePage {
    pub title: Vec<RichString>,
    pub credit: Vec<RichString>,
    pub authors: Vec<RichString>,
    pub source: Vec<RichString>,
    pub draft_date: Vec<RichString>,
    pub contact: Vec<RichString>,
}

impl TitlePage {
    pub fn new() -> Self {
        Self {
            title: Vec::new(),
            credit: Vec::new(),
            authors: Vec::new(),
            source: Vec::new(),
            draft_date: Vec::new(),
            contact: Vec::new(),
        }
    }
}
