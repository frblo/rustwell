use std::io::Write;

use crate::{
    export::Exporter,
    rich_string::{self, RichString},
    screenplay::{Dialogue, DialogueElement, Element, Screenplay, Span, TitlePage},
};

/// Contents of the `style.css` file with all css rules for the `html` output.
const CSS: &str = include_str!("style.css");

/// A [`Screenplay`] exporter for `HTML`
///
/// The variables configure the exporter
#[derive(Default)]
pub struct HtmlExporter {
    /// Decides if output should be standalone or if only the inner html
    /// should be produced.
    ///
    /// By standalone is meant that the output includes !DOCTYPE, <html> tags,
    /// and <head> tags,
    pub standalone: bool,
    /// If synopses should be included in the output
    pub synopses: bool,
    /// Decides if the `html` should include the line span for each element
    pub include_source_positions: bool,
}

impl Exporter for HtmlExporter {
    fn file_extension(&self) -> &'static str {
        "html"
    }

    fn export(&self, screenplay: &Screenplay, writer: &mut dyn Write) -> std::io::Result<()> {
        if self.standalone {
            writeln!(
                writer,
                r"<!DOCTYPE html><html>{}<body>",
                Self::export_head()
            )?;
        }

        writeln!(writer, r#"<div class="rustwell screenplay">"#)?;
        if let Some(titlepage) = &screenplay.titlepage {
            writeln!(writer, "{}", self.export_titlepage(titlepage))?;
        }
        for e in &screenplay.elements {
            writeln!(writer, "{}", self.export_element(e))?;
        }
        writeln!(writer, "</div>")?;

        if self.standalone {
            writeln!(writer, "</body></html>")?;
        }
        Ok(())
    }
}

impl HtmlExporter {
    /// Exports only the head for the `html` output.
    pub fn export_head() -> String {
        format!(
            r#"<head>
            <title>Screenplay</title>
            <style type="text/css">{CSS}</style></head>"#,
        )
    }

    /// Exports only the `css` for the `html` output.
    pub fn export_css() -> &'static str {
        CSS
    }

    /// Exports the [`TitlePage`] to a `html` string.
    fn export_titlepage(&self, titlepage: &TitlePage) -> String {
        format!(
            r#"
        <div class="title-page">
            {}
            {}
            {}
            {}
            {}
            {}
        </div>"#,
            self.export_titlepage_element("title", &titlepage.title),
            self.export_titlepage_element("credit", &titlepage.credit),
            self.export_titlepage_element("authors", &titlepage.authors),
            self.export_titlepage_element("source", &titlepage.source),
            self.export_titlepage_element("draft-date", &titlepage.draft_date),
            self.export_titlepage_element("contact", &titlepage.contact),
        )
    }

    /// Exports the [`TitlePage`] element, meaning one of values that can be included
    /// on the [`TitlePage`] to a `html` string. If there are no [`RichString`]s
    /// we do not include the value on the [`TitlePage`],
    /// and only return `""` here.
    fn export_titlepage_element(&self, value: &str, element: &[RichString]) -> String {
        if element.is_empty() {
            return String::new();
        }

        let content = element
            .iter()
            .map(|s| format!("<p>{}</p>", self.format_rich_string(s)))
            .collect::<Vec<String>>()
            .concat();

        format!(r#"<div class="{value}">{content}</div>"#)
    }

    /// Formats an [Element] into a `html`-[String].
    fn export_element(&self, element: &Span<Element>) -> String {
        if !self.synopses && matches!(element.inner, Element::Synopsis(_)) {
            return String::new();
        }

        let (class, content) = match &element.inner {
            Element::Heading { slug, number } => (
                "scene-heading",
                format!(
                    "{}{}{}",
                    if let Some(x) = number {
                        format!(r#"<span class="scnuml">{x}</span>"#)
                    } else {
                        String::new()
                    },
                    self.format_rich_string(slug),
                    if let Some(x) = number {
                        format!(
                            r#"<span class="scnumr">{}</span>"#,
                            Self::encode_special_html_characters(&x)
                        )
                    } else {
                        String::new()
                    },
                ),
            ),
            Element::Action(s) => ("action", format!("<p>{}</p>", self.format_rich_string(s))),
            Element::Dialogue(dialogue) => (
                "dialogue",
                format!(
                    r#"<p class="character">{}</p>{}"#,
                    self.format_character(dialogue),
                    self.format_dialogue(&dialogue.elements),
                ),
            ),
            Element::DualDialogue(dialogue1, dialogue2) => (
                "dual",
                format!(
                    r#"
                <div class="left">
                    <p class="character">{}</p>
                    {}
                </div>
                <div class="right">
                    <p class="character">{}</p>
                    {}
                </div>"#,
                    self.format_character(dialogue1),
                    self.format_dialogue(&dialogue1.elements),
                    self.format_character(dialogue2),
                    self.format_dialogue(&dialogue2.elements),
                ),
            ),
            Element::Lyrics(s) => (
                "lyrics",
                format!(r#"<p>{}</p>"#, self.format_rich_string(s)),
            ),
            Element::Transition(s) => ("transition", self.format_rich_string(s)),
            Element::CenteredText(s) => (
                "action centered",
                format!(r"<p>{}</p>", self.format_rich_string(s)),
            ),
            Element::Synopsis(s) => (
                "synopsis",
                format!(r#"<p>{}</p>"#, self.format_rich_string(s)),
            ),
            Element::PageBreak => ("", String::new()), // No pagebreaks in html
        };

        format!(
            r#"<div class="{}" {}>{}</div>"#,
            class,
            if self.include_source_positions {
                format!(
                    r#"data-start-line="{}" data-end-line="{}""#,
                    element.start_line, element.end_line
                )
            } else {
                String::new()
            },
            content
        )
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

    /// Formats a [`RichString`] into a `html`-[String].
    fn format_rich_string(&self, str: &RichString) -> String {
        str.elements
            .iter()
            .map(|e| self.format_rich_element(e))
            .collect::<Vec<String>>()
            .concat()
    }

    /// Formats a [`RichString`] [`rich_string::Element`] into a `html`-[String].
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
        format!(
            "{prepend}{}{append}",
            Self::encode_special_html_characters(&element.text)
        )
    }

    /// Formats the [Vec<DialogueElement>] of the dialogue into a `html`-[String], combining the
    /// [`DialogueElement`]s.
    fn format_dialogue(&self, dialogue: &[DialogueElement]) -> String {
        dialogue
            .iter()
            .map(|d| self.format_dialogue_element(d))
            .collect::<Vec<String>>()
            .join("\n")
    }

    /// Formats a [`DialogueElement`] into a `html`-[String].
    fn format_dialogue_element(&self, element: &DialogueElement) -> String {
        match element {
            DialogueElement::Parenthetical(s) => {
                format!(
                    r#"<p class="parenthetical">{}</p>"#,
                    self.format_rich_string(s)
                )
            }
            DialogueElement::Line(s) => format!(r"<p>{}</p>", self.format_rich_string(s)),
        }
    }

    /// Encodes potentially dangerous patterns in `html`
    fn encode_special_html_characters(s: &str) -> String {
        s.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
    }
}
