use crate::screenplay::Element;

/// [Style] is way to "force" a certain style upon a [RichString].
/// The [Style] struct contains all attributes of a [RichString] inside an `option`.
/// When pairing a [RichString] with a [Style], if a certain style is [None] in the [Style]
/// the exporter should go about business as usual, only taking into account the styles in
/// the [RichString]. If the style is [Some] in the [Style], the exporter should ignore
/// that style setting in the [RichString] and only use the value in [Style].
pub struct Style {
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underline: Option<bool>,
}

/// A [Style] with only [None], meaning it won't force any styling.
pub const NO_STYLE: Style = Style {
    bold: None,
    italic: None,
    underline: None,
};

/// The [Style] for scene headings. It forces bold.
const HEADING_STYLE: Style = Style {
    bold: Some(true),
    italic: None,
    underline: None,
};

/// Gets the [Style] of any given [Element]. If no style is enforced
/// it will return an full [None]-[Style], forcing nothing.
pub fn element_style(element: &Element) -> &Style {
    match element {
        Element::Heading { slug: _, number: _ } => &HEADING_STYLE,
        Element::Action(_) => &NO_STYLE,
        Element::Dialogue(_) => &NO_STYLE,
        Element::DualDialogue(_, _) => &NO_STYLE,
        Element::Lyrics(_) => &NO_STYLE,
        Element::Transition(_) => &NO_STYLE,
        Element::CenteredText(_) => &NO_STYLE,
        Element::Note(_) => &NO_STYLE,
        Element::PageBreak => &NO_STYLE,
    }
}
