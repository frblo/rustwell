use std::io::Write;

use crate::{
    rich_string::{self, RichString},
    screenplay::{Dialogue, DialogueElement, Element, Screenplay},
};

/// Contents of the `style.css` file with all css rules for the `html` output.
const CSS: &str = include_str!("style.css");

/// Exports the [Screenplay] in `html`-format to the given writer.
/// The function allows the caller to choose to include the default `css`
/// styling as part of the outputed file.
pub fn export_html(screenplay: &Screenplay, mut writer: impl Write, css: bool) {
    writeln!(
        &mut writer,
        r#"<!DOCTYPE html>
<html>
    <head>
        <title>Screenplay</title>
        {}
    </head>
    <body>
        <div id="wrapper" class="screenplay">"#,
        if css {
            format!(r#"<style type="text/css">{}</style>"#, CSS)
        } else {
            "".to_string()
        }
    )
    .expect("Failed to write to output");
    for e in &screenplay.elements {
        writeln!(&mut writer, "{}", export_element(e)).expect("Failed to write to output");
    }
    writeln!(
        &mut writer,
        r#"</div>
    </body>
</html>"#
    )
    .expect("Failed to write to output");
}

/// Formats an [Element] into a `html`-[String].
fn export_element(element: &Element) -> String {
    match element {
        Element::Heading { slug, number } => {
            format!(
                r#"<h6>{}{}{}</h6>"#,
                if let Some(x) = number {
                    format!(r#"<span class="scnuml">{}</span>"#, x)
                } else {
                    "".to_string()
                },
                format_rich_string(slug),
                if let Some(x) = number {
                    format!(r#"<span class="scnumr">{}</span>"#, x)
                } else {
                    "".to_string()
                },
            )
        }
        Element::Action(s) => format!(
            r#"<div class="action"><p>{}</p></div>"#,
            format_rich_string(s)
        ),
        Element::Dialogue(dialogue) => format!(
            r#"<div class="dialog"><p class="character">{}</p>{}</div>"#,
            format_character(&dialogue),
            format_dialogue(&dialogue.elements),
        ),
        Element::DualDialogue(dialogue1, dialogue2) => format!(
            r#"<div class="dual">
                <div class="left">
                    <p class="character">{}</p>
                    {}
                </div>
                <div class="right">
                    <p class="character">{}</p>
                    {}
                </div>
            </div>"#,
            format_character(&dialogue1),
            format_dialogue(&dialogue1.elements),
            format_character(&dialogue2),
            format_dialogue(&dialogue2.elements),
        ),
        Element::Lyrics(s) => format!(
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
            r#"<div class="note"><p>{}</p></div>"#,
            format_rich_string(s)
        ),
        Element::PageBreak => "".to_string(), // No pagebreaks in html
    }
}

fn format_character(dialogue: &Dialogue) -> String {
    if let Some(extension) = &dialogue.extension {
        format!(
            "{} ({})",
            format_rich_string(&dialogue.character),
            format_rich_string(extension)
        )
    } else {
        format_rich_string(&dialogue.character)
    }
}

/// Formats a [RichString] into a `html`-[String].
fn format_rich_string(str: &RichString) -> String {
    str.elements
        .iter()
        .map(|e| format_rich_element(e))
        .collect::<Vec<String>>()
        .concat()
}

/// Formats a [RichString] [rich_string::Element] into a `html`-[String].
fn format_rich_element(element: &rich_string::Element) -> String {
    // Assumes newlines '\n' will only occur sole elements
    if element.text == "\n" {
        return "<br />".to_string();
    }

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

/// Formats the [Vec<DialogueElement>] of the dialogue into a `html`-[String], combining the
/// [DialogueElement]s.
fn format_dialogue(dialogue: &Vec<DialogueElement>) -> String {
    dialogue
        .iter()
        .map(format_dialogue_element)
        .collect::<Vec<String>>()
        .join("\n")
}

/// Formats a [DialogueElement] into a `html`-[String].
fn format_dialogue_element(element: &DialogueElement) -> String {
    match element {
        DialogueElement::Parenthetical(s) => {
            format!(r#"<p class="parenthetical">{}</p>"#, format_rich_string(s))
        }
        DialogueElement::Line(s) => format!(r#"<p>{}</p>"#, format_rich_string(s)),
    }
}
