use std::io::Write;

use crate::{
    export::styles::{Style, element_style},
    rich_string::{self, RichString},
    screenplay::{DialogueElement, Element, Screenplay},
};

const CSS: &str = include_str!("style.css");

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

fn export_element(element: &Element) -> String {
    let style = element_style(element);
    match element {
        Element::Heading { slug, number } => {
            format!(
                r#"<h6>{}{}{}</h6>"#,
                if let Some(x) = number {
                    &format!(r#"<span class="scnuml">{}</span>"#, x)
                } else {
                    ""
                },
                format_rich_string(slug, &style),
                if let Some(x) = number {
                    &format!(r#"<span class="scnumr">{}</span>"#, x)
                } else {
                    ""
                },
            )
        }
        Element::Action(s) => format!(
            r#"<div class="action"><p>{}</p></div>"#,
            format_rich_string(s, &style)
        ),
        Element::Dialogue(dialogue) => format!(
            r#"<div class="dialog"><p class="character">{}</p>{}</div>"#,
            format_rich_string(&dialogue.character, &style),
            format_dialogue(&dialogue.elements, &style),
        ),
        Element::DualDialogue(dialogue, dialogue1) => todo!(),
        Element::Lyrics(s) => format!(
            // TODO: Class "lyrics" is not yet implemented in css
            r#"<div class="lyrics"><p>{}</p></div>"#,
            format_rich_string(s, &style)
        ),
        Element::Transition(s) => {
            format!(
                r#"<div class="transition">{}</div>"#,
                format_rich_string(s, &style)
            )
        }
        Element::CenteredText(s) => format!(
            r#"<div class="action centered"><p>{}</p></div>"#,
            format_rich_string(s, &style)
        ),
        Element::Note(s) => format!(
            // TODO: Class "note" is not yet implemented in css
            r#"<div class="note"><p>{}</p></div>"#,
            format_rich_string(s, &style)
        ),
        Element::PageBreak => "".to_string(), // No pagebreaks in html
    }
}

fn format_rich_string(str: &RichString, style: &Style) -> String {
    str.elements
        .iter()
        .map(|e| format_rich_element(e, style))
        .collect::<Vec<String>>()
        .concat()
}

fn format_rich_element(element: &rich_string::Element, style: &Style) -> String {
    let bold = (style.bold.is_none() && element.is_bold()) || style.bold.unwrap_or(false);
    let italic = (style.italic.is_none() && element.is_italic()) || style.italic.unwrap_or(false);
    let underline =
        (style.underline.is_none() && element.is_underline()) || style.underline.unwrap_or(false);

    let prepend = format!(
        "{}{}{}",
        if bold { "<strong>" } else { "" },
        if italic { "<em>" } else { "" },
        if underline { "<u>" } else { "" },
    );
    let append = format!(
        "{}{}{}",
        if underline { "</u>" } else { "" },
        if italic { "</em>" } else { "" },
        if bold { "</strong>" } else { "" },
    );
    format!("{prepend}{}{append}", element.text)
}

fn format_dialogue(dialogue: &Vec<DialogueElement>, style: &Style) -> String {
    dialogue
        .iter()
        .map(|d| format_dialogue_element(d, style))
        .collect::<Vec<String>>()
        .join("\n")
}

fn format_dialogue_element(element: &DialogueElement, style: &Style) -> String {
    match element {
        DialogueElement::Parenthetical(s) => {
            format!(
                r#"<p class="parenthetical">{}</p>"#,
                format_rich_string(s, style)
            )
        }
        DialogueElement::Line(s) => format!(r#"<p>{}</p>"#, format_rich_string(s, style)),
    }
}

#[cfg(test)]
mod tests {
    use std::io::{BufWriter, stdout};

    use super::*;

    #[test]
    fn test_thing() {
        let play = Screenplay {
            titlepage: None,
            elements: vec![
                Element::Heading {
                    slug: "INT. Hej".into(),
                    number: None,
                },
                Element::Action("Bosse går till affären".into()),
            ],
        };

        let writer = BufWriter::new(stdout());
        export_html(&play, writer, true);
        assert!(true)
    }
}
