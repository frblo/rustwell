use std::{collections::HashMap, io::Write};

use typst::{
    self, Library,
    diag::{FileError, FileResult},
    foundations::{Bytes, Datetime},
    syntax::{FileId, Source, VirtualPath},
    text::{Font, FontBook, FontInfo},
    utils::LazyHash,
};
use typst_pdf::PdfOptions;

use crate::{
    rich_string::{self, RichString},
    screenplay::{DialogueElement, Element, Screenplay},
};

const TEMPLATE: &str = include_str!("template.typ");

const FONTS: [&[u8]; 4] = [
    include_bytes!("fonts/CourierPrime-Regular.ttf"),
    include_bytes!("fonts/CourierPrime-Bold.ttf"),
    include_bytes!("fonts/CourierPrime-Italic.ttf"),
    include_bytes!("fonts/CourierPrime-BoldItalic.ttf"),
];

pub fn export_pdf(screenplay: &Screenplay, mut writer: impl Write) {
    let (fontbook, fonts) = create_fontbook();
    let content = format_as_typst(screenplay);
    let worldplay = WorldPlay::new(content, &fontbook, &fonts);
    let typed_doc = typst::compile(&worldplay)
        .output
        .expect("Error compiling pdf output");

    let pdf = typst_pdf::pdf(&typed_doc, &PdfOptions::default()).expect("Error generating pdf");
    writer.write_all(&pdf).expect("Error writing PDF.");
}

fn format_as_typst(screenplay: &Screenplay) -> String {
    let formatted_elements = screenplay
        .elements
        .iter()
        .map(export_element)
        .collect::<Vec<String>>();
    let titlepage = export_titlepage(screenplay);
    format!("{TEMPLATE}\n{titlepage}\n{}", formatted_elements.join("\n"))
}

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

fn export_element(element: &Element) -> String {
    match element {
        Element::Heading { slug, number } => format!("#scene[{}]", format_rich_string(slug)),
        Element::Action(s) => format_rich_string(s),
        Element::Dialogue(dialogue) => format!(
            "#dialogue[{}][{}]",
            format_rich_string(&dialogue.character),
            format_dialogue(&dialogue.elements),
        ),
        Element::DualDialogue(dialogue1, dialogue2) => format!(
            "#dual_dialogue[{}][{}][{}][{}]",
            format_rich_string(&dialogue1.character),
            format_dialogue(&dialogue1.elements),
            format_rich_string(&dialogue2.character),
            format_dialogue(&dialogue2.elements),
        ),
        Element::Lyrics(s) => format!("#lyrics[{}]", format_rich_string(s)),
        Element::Transition(s) => format!("#transition[{}]", format_rich_string(s)),
        Element::CenteredText(s) => format!("#centered[{}]", format_rich_string(s)),
        Element::Note(s) => "".to_string(),
        Element::PageBreak => "#pagebreak()".to_string(),
    }
}

fn format_dialogue(dialogue: &Vec<DialogueElement>) -> String {
    dialogue
        .iter()
        .map(format_dialogue_element)
        .collect::<Vec<String>>()
        .join(" ")
}

/// Formats a [DialogueElement] into a `html`-[String].
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
        element.text.replace("\\", "\\\\").replace("\"", "\\\"")
    );
    if element.is_underline() {
        out = format!("#underline[{}]", out);
    }

    out
}

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

struct WorldPlay<'a> {
    library: LazyHash<Library>,
    book: &'a LazyHash<FontBook>,
    source: HashMap<FileId, FileEntry>,
    fonts: &'a Vec<Font>,
}

/// `MAIN` contains the "filename" of the main file, which in typst **has** to be `/main.typ`.
const MAIN: &str = "/main.typ";

impl<'a> WorldPlay<'a> {
    fn new(content: String, book: &'a LazyHash<FontBook>, fonts: &'a Vec<Font>) -> Self {
        let mut sources = HashMap::new();
        let main = create_source(MAIN, content.clone());
        let main_entry = FileEntry::new(content.into(), Some(main.clone()));
        sources.insert(main.id(), main_entry);

        Self {
            library: LazyHash::new(Library::default()),
            book,
            fonts,
            source: sources,
        }
    }
}

/// A [File] that will be stored in the HashMap.
#[derive(Clone, Debug)]
struct FileEntry {
    bytes: Bytes,
    source: Option<Source>,
}

impl FileEntry {
    /// Creates a new [FileEntry], who would have thought?
    fn new(bytes: Vec<u8>, source: Option<Source>) -> Self {
        Self {
            bytes: Bytes::new(bytes),
            source,
        }
    }

    /// Gets the required file from a [FileId], preferably already from the [Source], but if it
    /// has not already been added to the file pool of the [World] it will try to get it and then
    /// add it to the file pool.
    fn source(&mut self, id: FileId) -> FileResult<Source> {
        let source = if let Some(source) = &self.source {
            source
        } else {
            let contents = std::str::from_utf8(&self.bytes).map_err(|_| FileError::InvalidUtf8)?;
            let contents = contents.trim_start_matches('\u{feff}');
            let source = Source::new(id, contents.into());
            self.source.insert(source)
        };
        Ok(source.clone())
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
            Some(d) => {
                let mut d = d.clone();
                d.source(id)
            }
            None => FileResult::Err(FileError::NotSource),
        }
    }

    /// Try to access the specified file.
    fn file(&self, _: FileId) -> FileResult<Bytes> {
        FileResult::Err(FileError::NotSource)
    }

    /// Try to access the font with the given index in the font book.
    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.get(index).cloned()
    }

    /// This function returns [None], Typst's `datetime` function will
    /// return an error as a result. Good thing it's not used.
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

/// Creates a [FontBook] which indexes the [Vec<Font>].
pub fn create_fontbook() -> (LazyHash<FontBook>, Vec<Font>) {
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
