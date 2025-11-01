use std::{collections::HashMap, io::Write};

use typst::{
    self, Library,
    diag::{FileError, FileResult},
    foundations::{Bytes, Datetime},
    layout::PagedDocument,
    syntax::{FileId, Source, VirtualPath},
    text::{Font, FontBook, FontInfo},
    utils::LazyHash,
};

use crate::{
    rich_string::{self, RichString},
    screenplay::{DialogueElement, Element, Screenplay},
};

/// The contents of the [typst] template `template.typ` found in the
/// export module.
const TEMPLATE: &str = include_str!("template.typ");

/// Exports the provided [Screenplay] as a pure [typst] document that can be
/// manually compiled with any [typst]-compiler. The document will not be very
/// readable nor be provided with comments explaining anything. This is mainly included
/// for debugging.
pub fn export_typst(screenplay: &Screenplay, mut writer: impl Write) {
    let content = format_as_typst(screenplay);
    write!(writer, "{content}").expect("Failed to write to typst document");
}

/// Generates a [PagedDocument], which is a layouted [typst] document which can then
/// be exported and written with any [typst] exporter, like [typst_pdf].
pub fn compile_document(screenplay: &Screenplay) -> PagedDocument {
    let (fontbook, fonts) = create_fontbook();
    let content = format_as_typst(screenplay);
    let worldplay = WorldPlay::new(content, &fontbook, &fonts);
    typst::compile(&worldplay)
        .output
        .expect("Error compiling typst document")
}

/// Formats the [Screenplay] as a [typst] document, meaning it essentially gets
/// converted into [typst]-compilable code.
fn format_as_typst(screenplay: &Screenplay) -> String {
    let formatted_elements = screenplay
        .elements
        .iter()
        .map(export_element)
        .collect::<Vec<String>>();
    let titlepage = export_titlepage(screenplay);
    format!("{TEMPLATE}\n{titlepage}\n{}", formatted_elements.join("\n"))
}

/// Exports the [crate::screenplay::TitlePage] in the provided [Screenplay] to [typst] code.
/// This function also provides the necessary `#show: screenplay.with(...)` that
/// handles the page layout for the whole screenplay.
fn export_titlepage(screenplay: &Screenplay) -> String {
    if let Some(titlepage) = &screenplay.titlepage {
        let title = format_titlepage_element(&titlepage.title);
        let credit = format_titlepage_element(&titlepage.credit);
        let authors = format_titlepage_element(&titlepage.authors);
        let source = format_titlepage_element(&titlepage.source);
        let draft_date = format_titlepage_element(&titlepage.draft_date);
        let contact = format_titlepage_element(&titlepage.contact);
        format!(
            r#"#show: screenplay.with(
  titlepage: true,
  title: {title},
  credit: {credit},
  authors: {authors},
  source: {source},
  draft_date: {draft_date},
  contact: {contact},
)"#
        )
    } else {
        "#show: screenplay.with(titlepage: false)".to_string()
    }
}

/// Exports a single [Element] as [typst] code. Primarily done by calling the associated
/// [typst] function found in the template.
fn export_element(element: &Element) -> String {
    match element {
        Element::Heading { slug, number } => {
            if let Some(num) = number {
                format!(
                    r#"#scene(number: "{}")[{}]"#,
                    replace_escaping(num),
                    format_rich_string(slug)
                )
            } else {
                format!("#scene[{}]", format_rich_string(slug))
            }
        }
        Element::Action(s) => format_rich_string(s),
        Element::Dialogue(dialogue) => format!(
            "#dialogue(paren: {})[{}][{}]",
            format_character_extension(&dialogue.extension),
            format_rich_string(&dialogue.character),
            format_dialogue(&dialogue.elements),
        ),
        Element::DualDialogue(dialogue1, dialogue2) => format!(
            "#dual_dialogue(paren1: {}, paren2: {})[{}][{}][{}][{}]",
            format_character_extension(&dialogue1.extension),
            format_character_extension(&dialogue2.extension),
            format_rich_string(&dialogue1.character),
            format_dialogue(&dialogue1.elements),
            format_rich_string(&dialogue2.character),
            format_dialogue(&dialogue2.elements),
        ),
        Element::Lyrics(s) => format!("#lyrics[{}]", format_rich_string(s)),
        Element::Transition(s) => format!("#transition[{}]", format_rich_string(s)),
        Element::CenteredText(s) => format!("#centered[{}]", format_rich_string(s)),
        Element::Note(_) => "".to_string(), // TODO: Implement when decided on what do to
        Element::PageBreak => "#pagebreak()".to_string(),
    }
}

/// Formats the dialogue into [typst] code.
fn format_dialogue(dialogue: &Vec<DialogueElement>) -> String {
    dialogue
        .iter()
        .map(format_dialogue_element)
        .collect::<Vec<String>>()
        .join(" ")
}

/// Formats the character extension (`(V.O)`, for example) that is
/// next to a character's name in a dialogue.
fn format_character_extension(opt_ext: &Option<RichString>) -> String {
    if let Some(ext) = opt_ext {
        format!("[{}]", format_rich_string(ext))
    } else {
        "none".to_string()
    }
}

/// Formats a [DialogueElement] into a [typst] code.
fn format_dialogue_element(element: &DialogueElement) -> String {
    match element {
        DialogueElement::Parenthetical(s) => {
            format!("#parenthetical[{}]", format_rich_string(s))
        }
        DialogueElement::Line(s) => format_rich_string(s),
    }
}

/// Formats a [RichString] into a [typst]-[String].
fn format_rich_string(str: &RichString) -> String {
    str.elements
        .iter()
        .map(format_rich_element)
        .collect::<Vec<String>>()
        .concat()
}

/// Formats a [RichString] [rich_string::Element] into a [typst]-[String].
/// All elements will be explicitly contained in a `#text("{element.text}")`
/// function from [typst], with styling using `weight: "bold"`, `style: "italic"`
/// and `#underline[#text(...)]`.
///
/// This function also iterates over each string twice to replace all escaping
/// characters `\` and `"` with `\\` and `\*` respectively.
fn format_rich_element(element: &rich_string::Element) -> String {
    // Assumes newlines '\n' will only occur sole elements
    if element.text == "\n" {
        return "\\ ".to_string();
    }

    let mut out = format!(
        "#text({}{}\"{}\")",
        if element.is_bold() {
            "weight: \"bold\","
        } else {
            ""
        },
        if element.is_italic() {
            "style: \"italic\","
        } else {
            ""
        },
        replace_escaping(&element.text)
    );
    if element.is_underline() {
        out = format!("#underline[{}]", out);
    }

    out
}

/// This function also iterates over each string twice to replace all escaping
/// characters `\` and `"` with `\\` and `\*` respectively.
fn replace_escaping(s: &str) -> String {
    s.replace("\\", "\\\\").replace("\"", "\\\"")
}

/// Formats a single [crate::screenplay::TitlePage] element into [typst] code.
/// If no value has been declared it will return `"none"`.
fn format_titlepage_element(element: &Vec<RichString>) -> String {
    if element.is_empty() {
        return "none".to_string();
    }
    format!(
        "[{}]",
        element
            .iter()
            .map(format_rich_string)
            .collect::<Vec<String>>()
            .join("\\ ")
    )
}

/// Internal [typst::World] which is basically the whole underlying structure of the [typst]
/// document. This is significantly more slimmed down than a real [typst::World] is, as
/// everything not needed for Rustwell has been stripped away.
struct WorldPlay<'a> {
    library: LazyHash<Library>,
    book: &'a LazyHash<FontBook>,
    source: HashMap<FileId, Source>,
    fonts: &'a Vec<Font>,
}

/// `MAIN` contains the "filename" of the main file, which in [typst] **has** to be `/main.typ`.
const MAIN: &str = "/main.typ";

/// The font bundled together with Rustwell; Courier Prime.
/// Includes the data of the font styles Regular, Bold, Italic
/// and BoldItalic, in stated order.
const FONTS: [&[u8]; 4] = [
    include_bytes!("fonts/CourierPrime-Regular.ttf"),
    include_bytes!("fonts/CourierPrime-Bold.ttf"),
    include_bytes!("fonts/CourierPrime-Italic.ttf"),
    include_bytes!("fonts/CourierPrime-BoldItalic.ttf"),
];

impl<'a> WorldPlay<'a> {
    fn new(content: String, book: &'a LazyHash<FontBook>, fonts: &'a Vec<Font>) -> Self {
        let mut sources = HashMap::with_capacity(1);
        let main = create_source(MAIN, content);
        let main_id = main.id();
        sources.insert(main_id, main);

        Self {
            library: LazyHash::new(Library::default()),
            book,
            fonts,
            source: sources,
        }
    }
}

impl typst::World for WorldPlay<'_> {
    /// The standard library.
    ///
    /// Can be created through `Library::build()`.
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    /// Metadata about all known fonts.
    fn book(&self) -> &LazyHash<FontBook> {
        self.book
    }

    /// Get the file id of the main source file.
    fn main(&self) -> FileId {
        create_file_id(MAIN)
    }

    /// Try to access the specified source file.
    fn source(&self, id: FileId) -> FileResult<Source> {
        match self.source.get(&id) {
            Some(s) => Ok(s.clone()),
            None => FileResult::Err(FileError::NotSource),
        }
    }

    /// Try to access the specified file.
    /// WARNING: This function will only return [FileError] as it is
    /// is not implemented, nor needed for Rustwell.
    fn file(&self, _: FileId) -> FileResult<Bytes> {
        FileResult::Err(FileError::NotSource)
    }

    /// Try to access the font with the given index in the font book.
    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.get(index).cloned()
    }

    /// Gets the current system time.
    /// WARNING: This function will only return [None] as it is
    /// is not implemented, nor needed for Rustwell.
    fn today(&self, _: Option<i64>) -> Option<Datetime> {
        None
    }
}

/// Creates a [Source] based on a document.
fn create_source(filename: &str, content: String) -> Source {
    let file_id = create_file_id(filename);
    Source::new(file_id, content)
}

/// Creates a [FileId] based on a filename.
fn create_file_id(filename: &str) -> FileId {
    FileId::new(None, VirtualPath::new(filename))
}

/// Creates a [FontBook] which indexes the returned [Vec<Font>].
fn create_fontbook() -> (LazyHash<FontBook>, Vec<Font>) {
    let mut fonts = Vec::new();
    let mut fontbook = FontBook::new();

    for font_data in FONTS.iter() {
        let font = match Font::new(Bytes::new(font_data), 0) {
            Some(f) => f,
            None => continue,
        };
        fonts.push(font);

        let info = FontInfo::new(font_data, 0).expect("Could not parse font");
        fontbook.push(info);
    }

    (LazyHash::new(fontbook), fonts)
}
