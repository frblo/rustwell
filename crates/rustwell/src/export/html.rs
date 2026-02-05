use std::io::Write;

use crate::{
    export::Exporter,
    rich_string::{self, RichString},
    screenplay::{Dialogue, DialogueElement, Element, Screenplay, TitlePage},
};

/// Contents of the `style.css` file with all css rules for the `html` output.
const CSS: &str = include_str!("style.css");

#[derive(Default)]
pub struct HtmlExporter {
    pub css: bool,
    pub synopses: bool,
}

impl Exporter for HtmlExporter {
    fn file_extension(&self) -> &'static str {
        "html"
    }

    fn export(&self, screenplay: &Screenplay, writer: &mut dyn Write) -> std::io::Result<()> {
        writeln!(
            writer,
            r#"<!DOCTYPE html>
<html>
    <head>
        <title>Screenplay</title>
        {}
    </head>
    <body>
        <div id="wrapper" class="screenplay">"#,
            if self.css {
                format!(r#"<style type="text/css">{}</style>"#, CSS)
            } else {
                "".to_string()
            }
        )?;
        if let Some(titlepage) = &screenplay.titlepage {
            writeln!(writer, "{}", self.export_titlepage(titlepage))?;
        }
        for e in &screenplay.elements {
            writeln!(writer, "{}", self.export_element(e))?;
        }
        writeln!(
            writer,
            r#"</div>
        </body>
    </html>"#
        )?;

        Ok(())
    }
}

impl HtmlExporter {
    /// Exports the [TitlePage] to a `html` string.
    fn export_titlepage(&self, titlepage: &TitlePage) -> String {
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
            self.export_titlepage_element("title", &titlepage.title),
            self.export_titlepage_element("credit", &titlepage.credit),
            self.export_titlepage_element("authors", &titlepage.authors),
            self.export_titlepage_element("source", &titlepage.source),
            self.export_titlepage_element("draft-date", &titlepage.draft_date),
            self.export_titlepage_element("contact", &titlepage.contact),
        )
    }

    /// Exports the [TitlePage] element, meaning one of values that can be included
    /// on the [TitlePage] to a `html` string. If there are no [RichString]s we do not include
    /// the value on the [TitlePage], and only return `""` here.
    fn export_titlepage_element(&self, value: &str, element: &[RichString]) -> String {
        if element.is_empty() {
            return "".to_string();
        }

        let content = element
            .iter()
            .map(|s| format!("<p>{}</p>", self.format_rich_string(s)))
            .collect::<Vec<String>>()
            .concat();

        format!(r#"<div class="{}">{}</div>"#, value, content)
    }

    /// Formats an [Element] into a `html`-[String].
    fn export_element(&self, element: &Element) -> String {
        match element {
            Element::Heading { slug, number } => {
                format!(
                    r#"<h6>{}{}{}</h6>"#,
                    if let Some(x) = number {
                        format!(r#"<span class="scnuml">{}</span>"#, x)
                    } else {
                        "".to_string()
                    },
                    self.format_rich_string(slug),
                    if let Some(x) = number {
                        format!(r#"<span class="scnumr">{}</span>"#, x)
                    } else {
                        "".to_string()
                    },
                )
            }
            Element::Action(s) => format!(
                r#"<div class="action"><p>{}</p></div>"#,
                self.format_rich_string(s)
            ),
            Element::Dialogue(dialogue) => format!(
                r#"<div class="dialog"><p class="character">{}</p>{}</div>"#,
                self.format_character(dialogue),
                self.format_dialogue(&dialogue.elements),
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
                self.format_character(dialogue1),
                self.format_dialogue(&dialogue1.elements),
                self.format_character(dialogue2),
                self.format_dialogue(&dialogue2.elements),
            ),
            Element::Lyrics(s) => format!(
                r#"<div class="lyrics"><p>{}</p></div>"#,
                self.format_rich_string(s)
            ),
            Element::Transition(s) => {
                format!(
                    r#"<div class="transition">{}</div>"#,
                    self.format_rich_string(s)
                )
            }
            Element::CenteredText(s) => format!(
                r#"<div class="action centered"><p>{}</p></div>"#,
                self.format_rich_string(s)
            ),
            Element::Synopsis(s) => {
                if self.synopses {
                    format!(
                        r#"<div class="synopsis"><p>{}</p></div>"#,
                        self.format_rich_string(s)
                    )
                } else {
                    "".to_string()
                }
            }
            Element::PageBreak => "".to_string(), // No pagebreaks in html
        }
    }

    fn format_character(&self, dialogue: &Dialogue) -> String {
        if let Some(extension) = &dialogue.extension {
            format!(
                "{} ({})",
                self.format_rich_string(&dialogue.character),
                self.format_rich_string(extension)
            )
        } else {
            self.format_rich_string(&dialogue.character)
        }
    }

    /// Formats a [RichString] into a `html`-[String].
    fn format_rich_string(&self, str: &RichString) -> String {
        str.elements
            .iter()
            .map(|e| self.format_rich_element(e))
            .collect::<Vec<String>>()
            .concat()
    }

    /// Formats a [RichString] [rich_string::Element] into a `html`-[String].
    fn format_rich_element(&self, element: &rich_string::Element) -> String {
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
    fn format_dialogue(&self, dialogue: &[DialogueElement]) -> String {
        dialogue
            .iter()
            .map(|d| self.format_dialogue_element(d))
            .collect::<Vec<String>>()
            .join("\n")
    }

    /// Formats a [DialogueElement] into a `html`-[String].
    fn format_dialogue_element(&self, element: &DialogueElement) -> String {
        match element {
            DialogueElement::Parenthetical(s) => {
                format!(
                    r#"<p class="parenthetical">{}</p>"#,
                    self.format_rich_string(s)
                )
            }
            DialogueElement::Line(s) => format!(r#"<p>{}</p>"#, self.format_rich_string(s)),
        }
    }
}
