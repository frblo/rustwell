use crate::screenplay::Element;

pub struct Style {
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underline: Option<bool>,
}

const NO_STYLE: Style = Style {
    bold: None,
    italic: None,
    underline: None,
};

pub fn element_style(element: &Element) -> Style {
    match element {
        Element::Heading { slug: _, number: _ } => Style {
            bold: Some(true),
            italic: None,
            underline: None,
        },
        Element::Action(_) => NO_STYLE,
        Element::Dialogue(_) => NO_STYLE,
        Element::DualDialogue(_, _) => NO_STYLE,
        Element::Lyrics(_) => Style {
            bold: None,
            italic: Some(true),
            underline: None,
        },
        Element::Transition(_) => NO_STYLE,
        Element::CenteredText(_) => NO_STYLE,
        Element::Note(_) => NO_STYLE,
        Element::PageBreak => NO_STYLE,
    }
}
