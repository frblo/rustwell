use crate::{
    rich_string::{self, RichString},
    screenplay::{DialogueElement, Element},
};

fn export_element(element: &Element) -> String {
    match element {
        Element::Heading { slug, number } => {
            format!(
                r#"<h6>{}{}{}</h6>"#,
                if let Some(x) = number {
                    &format!(r#"<span class="scnuml">{}</span>"#, x)
                } else {
                    ""
                },
                format_rich_string(slug),
                if let Some(x) = number {
                    &format!(r#"<span class="scnumr">{}</span>"#, x)
                } else {
                    ""
                },
            )
        }
        Element::Action(s) => format!(
            r#"<div class="action"><p>{}</p></div>"#,
            format_rich_string(s)
        ),
        Element::Dialogue(dialogue) => format!(
            r#"<div class="dialog"><p class="character">{}</p>{}</div>"#,
            format_rich_string(&dialogue.character),
            format_dialogue(&dialogue.elements),
        ),
        Element::DualDialogue(dialogue, dialogue1) => todo!(),
        Element::Lyrics(s) => format!(
            // TODO: Class "lyrics" is not yet implemented in css
            r#"<div class="lyrics"><p>{}</p></div>"#,
            format_rich_string(s)
        ),
        Element::Transition(s) => {
            format!(r#"<div class="transition">{}</div>"#, format_rich_string(s))
        }
        Element::CenteredText(s) => format!(
            r#"<div class="action centered"><p>{}</p></div>"#,
            format_rich_string(s)
        ),
        Element::Note(s) => format!(
            // TODO: Class "note" is not yet implemented in css
            r#"<div class="note"><p>{}</p></div>"#,
            format_rich_string(s)
        ),
        Element::PageBreak => "".to_string(), // No pagebreaks in html
    }
}

fn format_rich_string(str: &RichString) -> String {
    str.elements
        .iter()
        .map(format_rich_element)
        .collect::<Vec<String>>()
        .concat()
}

fn format_rich_element(element: &rich_string::Element) -> String {
    let prepend = format!(
        "{}{}{}",
        if element.is_bold() { "<strong>" } else { "" },
        if element.is_italic() { "<em>" } else { "" },
        if element.is_underline() { "<u>" } else { "" },
    );
    let append = format!(
        "{}{}{}",
        if element.is_underline() { "</u>" } else { "" },
        if element.is_italic() { "</em>" } else { "" },
        if element.is_bold() { "</strong>" } else { "" },
    );
    format!("{prepend}{}{append}", element.text)
}

fn format_dialogue(dialogue: &Vec<DialogueElement>) -> String {
    dialogue
        .iter()
        .map(format_dialogue_element)
        .collect::<Vec<String>>()
        .join("\n")
}

fn format_dialogue_element(element: &DialogueElement) -> String {
    match element {
        DialogueElement::Parenthetical(s) => {
            format!(r#"<p class="parenthetical">{}</p>"#, format_rich_string(s))
        }
        DialogueElement::Line(s) => format!(r#"<p>{}</p>"#, format_rich_string(s)),
    }
}
