use std::io::Write;

use crate::{
    export::styles::{NO_STYLE, Style, element_style},
    rich_string::{self, RichString},
    screenplay::{Dialogue, DialogueElement, Element, Screenplay, TitlePage},
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
    if let Some(titlepage) = &screenplay.titlepage {
        writeln!(&mut writer, "{}", export_titlepage(titlepage))
            .expect("Failed to write to output");
    }
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

fn export_titlepage(titlepage: &TitlePage) -> String {
    format!(
        r#"
        <div id="title-page">
            {}
            {}
            {}
            {}
            {}
            {}
        </div>
    "#,
        export_titlepage_element("title", &titlepage.title),
        export_titlepage_element("credit", &titlepage.credit),
        export_titlepage_element("authors", &titlepage.authors),
        export_titlepage_element("source", &titlepage.source),
        export_titlepage_element("draft-date", &titlepage.draft_date),
        export_titlepage_element("contact", &titlepage.title),
    )
}

fn export_titlepage_element(value: &str, element: &Vec<RichString>) -> String {
    if element.is_empty() {
        return "".to_string();
    }

    let content = element
        .iter()
        .map(|s| format!("<p>{}</p>", format_rich_string(s, &NO_STYLE)))
        .collect::<Vec<String>>()
        .concat();

    format!(r#"<div class="{}">{}</div>"#, value, content)
}

/// Formats an [Element] into a `html`-[String].
fn export_element(element: &Element) -> String {
    let style = element_style(element);
    match element {
        Element::Heading { slug, number } => {
            format!(
                r#"<h6>{}{}{}</h6>"#,
                if let Some(x) = number {
                    format!(r#"<span class="scnuml">{}</span>"#, x)
                } else {
                    "".to_string()
                },
                format_rich_string(slug, style),
                if let Some(x) = number {
                    format!(r#"<span class="scnumr">{}</span>"#, x)
                } else {
                    "".to_string()
                },
            )
        }
        Element::Action(s) => format!(
            r#"<div class="action"><p>{}</p></div>"#,
            format_rich_string(s, style)
        ),
        Element::Dialogue(dialogue) => format!(
            r#"<div class="dialog"><p class="character">{}</p>{}</div>"#,
            format_character(&dialogue, style),
            format_dialogue(&dialogue.elements, style),
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
            format_character(&dialogue1, style),
            format_dialogue(&dialogue1.elements, style),
            format_character(&dialogue2, style),
            format_dialogue(&dialogue2.elements, style),
        ),
        Element::Lyrics(s) => format!(
            r#"<div class="lyrics"><p>{}</p></div>"#,
            format_rich_string(s, style)
        ),
        Element::Transition(s) => {
            format!(
                r#"<div class="transition">{}</div>"#,
                format_rich_string(s, style)
            )
        }
        Element::CenteredText(s) => format!(
            r#"<div class="action centered"><p>{}</p></div>"#,
            format_rich_string(s, style)
        ),
        Element::Note(s) => format!(
            r#"<div class="note"><p>{}</p></div>"#,
            format_rich_string(s, style)
        ),
        Element::PageBreak => "".to_string(), // No pagebreaks in html
    }
}

fn format_character(dialogue: &Dialogue, style: &Style) -> String {
    if let Some(extension) = &dialogue.extension {
        format!(
            "{} ({})",
            format_rich_string(&dialogue.character, style),
            format_rich_string(extension, style)
        )
    } else {
        format_rich_string(&dialogue.character, style)
    }
}

/// Formats a [RichString] into a `html`-[String].
fn format_rich_string(str: &RichString, style: &Style) -> String {
    str.elements
        .iter()
        .map(|e| format_rich_element(e, style))
        .collect::<Vec<String>>()
        .concat()
}

/// Formats a [RichString] [rich_string::Element] into a `html`-[String].
/// Here the [Style] is taken into consideration and will overrule any styling
/// in the [rich_string::Element], if it's [Some] in the given [Style].
fn format_rich_element(element: &rich_string::Element, style: &Style) -> String {
    // Assumes newlines '\n' will only occur sole elements
    if element.text == "\n" {
        return "<br />".to_string();
    }
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

/// Formats the [Vec<DialogueElement>] of the dialogue into a `html`-[String], combining the
/// [DialogueElement]s.
fn format_dialogue(dialogue: &Vec<DialogueElement>, style: &Style) -> String {
    dialogue
        .iter()
        .map(|d| format_dialogue_element(d, style))
        .collect::<Vec<String>>()
        .join("\n")
}

/// Formats a [DialogueElement] into a `html`-[String].
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
