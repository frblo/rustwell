use std::{collections::HashMap, io::Write};

use typst::{
    self, Library, LibraryExt,
    diag::{FileError, FileResult},
    foundations::{Bytes, Datetime},
    layout::PagedDocument,
    syntax::{FileId, Source, VirtualPath},
    text::{Font, FontBook, FontInfo},
    utils::LazyHash,
};

use crate::{
    Exporter,
    rich_string::{self, RichString},
    screenplay::{DialogueElement, Element, Screenplay, Span},
};

/// The contents of the [typst] template `template.typ` found in the
/// export module.
const TEMPLATE: &str = include_str!("template.typ");

/// A [`Screenplay`] exporter for `typst`
///
/// The [`Exporter`] implementation for [`Exporter::export`] exports [`typst`]
/// source code.
///
/// The variables configure the exporter
#[derive(Default)]
pub struct TypstExporter {
    /// If synopses should be included in the output
    pub synopses: bool,
}

impl Exporter for TypstExporter {
    fn file_extension(&self) -> &'static str {
        "typ"
    }

    /// Exports [`typst`] source code
    fn export(&self, screenplay: &Screenplay, writer: &mut dyn Write) -> std::io::Result<()> {
        self.export_typst(screenplay, writer)
    }
}

impl TypstExporter {
    /// Exports the provided [Screenplay] as a pure [typst] document that can be
    /// manually compiled with any [typst]-compiler. The document will not be very
    /// readable nor be provided with comments explaining anything. This is mainly included
    /// for debugging.
    pub fn export_typst(
        &self,
        screenplay: &Screenplay,
        mut writer: impl Write,
    ) -> std::io::Result<()> {
        let content = self.format_as_typst(screenplay);
        write!(writer, "{content}")
    }

    /// Generates a [PagedDocument], which is a layouted [typst] document which can then
    /// be exported and written with any [typst] exporter, like [typst_pdf].
    pub fn compile_document(&self, screenplay: &Screenplay) -> std::io::Result<PagedDocument> {
        let (fontbook, fonts) = create_fontbook();
        let content = self.format_as_typst(screenplay);
        let worldplay = WorldPlay::new(content, &fontbook, &fonts);
        let pd: Result<PagedDocument, _> = typst::compile(&worldplay).output;
        match pd {
            Ok(p) => Ok(p),
            Err(_) => Err(std::io::Error::other("failed to compile typst document")),
        }
    }

    /// Formats the [Screenplay] as a [typst] document, meaning it essentially gets
    /// converted into [typst]-compilable code.
    fn format_as_typst(&self, screenplay: &Screenplay) -> String {
        let formatted_elements = screenplay
            .elements
            .iter()
            .map(
                |Span {
                     start_line: _,
                     end_line: _,
                     inner: e,
                 }| self.export_element(e),
            )
            .collect::<Vec<String>>();
        let titlepage = self.export_titlepage(screenplay);
        format!("{TEMPLATE}\n{titlepage}\n{}", formatted_elements.join("\n"))
    }

    /// Exports the [crate::screenplay::TitlePage] in the provided [Screenplay] to [typst] code.
    /// This function also provides the necessary `#show: screenplay.with(...)` that
    /// handles the page layout for the whole screenplay.
    fn export_titlepage(&self, screenplay: &Screenplay) -> String {
        if let Some(titlepage) = &screenplay.titlepage {
            let title = self.format_titlepage_element(&titlepage.title);
            let credit = self.format_titlepage_element(&titlepage.credit);
            let authors = self.format_titlepage_element(&titlepage.authors);
            let source = self.format_titlepage_element(&titlepage.source);
            let draft_date = self.format_titlepage_element(&titlepage.draft_date);
            let contact = self.format_titlepage_element(&titlepage.contact);
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
    fn export_element(&self, element: &Element) -> String {
        match element {
            Element::Heading { slug, number } => {
                if let Some(num) = number {
                    format!(
                        r#"#scene(number: "{}")[{}]"#,
                        self.replace_escaping(num),
                        self.format_rich_string(slug)
                    )
                } else {
                    format!("#scene[{}]", self.format_rich_string(slug))
                }
            }
            Element::Action(s) => self.format_rich_string(s),
            Element::Dialogue(dialogue) => format!(
                "#dialogue(paren: {})[{}][{}]",
                self.format_character_extension(&dialogue.extension),
                self.format_rich_string(&dialogue.character),
                self.format_dialogue(&dialogue.elements),
            ),
            Element::DualDialogue(dialogue1, dialogue2) => format!(
                "#dual_dialogue(paren1: {}, paren2: {})[{}][{}][{}][{}]",
                self.format_character_extension(&dialogue1.extension),
                self.format_character_extension(&dialogue2.extension),
                self.format_rich_string(&dialogue1.character),
                self.format_dialogue(&dialogue1.elements),
                self.format_rich_string(&dialogue2.character),
                self.format_dialogue(&dialogue2.elements),
            ),
            Element::Lyrics(s) => format!("#lyrics[{}]", self.format_rich_string(s)),
            Element::Transition(s) => format!("#transition[{}]", self.format_rich_string(s)),
            Element::CenteredText(s) => format!("#centered[{}]", self.format_rich_string(s)),
            Element::Synopsis(s) => {
                if self.synopses {
                    format!("#synopsis[{}]", self.format_rich_string(s))
                } else {
                    "".to_string()
                }
            }
            Element::PageBreak => "#pagebreak()".to_string(),
        }
    }

    /// Formats the dialogue into [typst] code.
    fn format_dialogue(&self, dialogue: &[DialogueElement]) -> String {
        dialogue
            .iter()
            .map(|d| self.format_dialogue_element(d))
            .collect::<Vec<String>>()
            .join(" ")
    }

    /// Formats the character extension (`(V.O)`, for example) that is
    /// next to a character's name in a dialogue.
    fn format_character_extension(&self, opt_ext: &Option<RichString>) -> String {
        if let Some(ext) = opt_ext {
            format!("[{}]", self.format_rich_string(ext))
        } else {
            "none".to_string()
        }
    }

    /// Formats a [DialogueElement] into a [typst] code.
    fn format_dialogue_element(&self, element: &DialogueElement) -> String {
        match element {
            DialogueElement::Parenthetical(s) => {
                format!("#parenthetical[{}]", self.format_rich_string(s))
            }
            DialogueElement::Line(s) => self.format_rich_string(s),
        }
    }

    /// Formats a [RichString] into a [typst]-[String].
    fn format_rich_string(&self, str: &RichString) -> String {
        str.elements
            .iter()
            .map(|e| self.format_rich_element(e))
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
    fn format_rich_element(&self, element: &rich_string::Element) -> String {
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
            self.replace_escaping(&element.text)
        );
        if element.is_underline() {
            out = format!("#underline[{}]", out);
        }

        out
    }

    /// This function also iterates over each string twice to replace all escaping
    /// characters `\` and `"` with `\\` and `\*` respectively.
    fn replace_escaping(&self, s: &str) -> String {
        s.replace("\\", "\\\\").replace("\"", "\\\"")
    }

    /// Formats a single [crate::screenplay::TitlePage] element into [typst] code.
    /// If no value has been declared it will return `"none"`.
    fn format_titlepage_element(&self, element: &[RichString]) -> String {
        if element.is_empty() {
            return "none".to_string();
        }
        format!(
            "[{}]",
            element
                .iter()
                .map(|e| self.format_rich_string(e))
                .collect::<Vec<String>>()
                .join("\\ ")
        )
    }
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
