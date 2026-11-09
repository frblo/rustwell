//! This module implements the [Screenplay] AST with all the components of a screenplay, to be
//! easily exported to any format.

use core::range::Range;

use crate::rich_string::RichString;

/// A (very flat) abstract syntax tree consisting of the entirety of a screenplay as well as the
/// information for the title page of the screenplay.
///
/// Contains both a [`Option<TitlePage>`] and a [`Vec<Element>`], which are the screenplay components.
/// Notes and boneyards are not part of the elements, they are kept as separate lists
/// positioned in the source.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Default)]
pub struct Screenplay {
    pub titlepage: Option<TitlePage>,
    pub elements: Vec<Span<Element>>,
    /// Every closed `[[ ]]` note in the source, in source order.
    pub notes: Vec<Note>,
    /// Every `/* */` boneyard in the source, in source order.
    pub boneyards: Vec<Boneyard>,
}

/// A byte range into the source.
pub type SourceRange = Range<usize>;

/// Meta information about some part of the screenplay.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Span<T> {
    /// Byte range of this part in the original source.
    pub span: SourceRange,
    pub inner: T,
}

impl<T> Span<T> {
    pub fn new(inner: T, span: SourceRange) -> Self {
        Self { span, inner }
    }
}

/// A `/* */` boneyard located in the source.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Boneyard {
    /// The whole boneyard: `/*`, the content, and `*/` when present (an
    /// unterminated boneyard runs to end of input).
    pub span: SourceRange,
    /// The range of the content between the delimiters.
    pub inner: SourceRange,
    /// The content between the delimiters, as written.
    pub text: String,
}

/// A `[[ ]]` closed note located in the source.
///
/// A `[[` with no matching `]]` before a blank line or end of input is not
/// closed and therefore not a [`Note`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Note {
    /// The whole note: `[[`, the content, and `]]`.
    pub span: SourceRange,
    /// The range of the content between the delimiters.
    pub inner: SourceRange,
    /// The content between the delimiters, as written.
    pub text: String,
}

/// Returns the 1-based line and 1-based column (in characters) of byte `offset` in `src`.
///
/// Offsets past the end of `src` are clamped to the end.
///
/// # Panics
///
/// Panics if `offset` is not on a char boundary.
pub fn line_col(src: &str, offset: usize) -> (usize, usize) {
    let before = &src[..offset.min(src.len())];
    let line = before.matches('\n').count() + 1;
    let line_start = before.rfind('\n').map_or(0, |i| i + 1);
    (line, before[line_start..].chars().count() + 1)
}

/// The components of a [`Screenplay`], like scene headings, action, dialogue, etc.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Element {
    Section {
        depth: u8,
        /// The heading text. The range covers the text only, without the `#`s.
        heading: Span<RichString>,
    },
    Heading {
        /// Written with a leading `.`.
        forced: bool,
        /// The heading text. The range covers the text only, without the leading `.` and the
        /// trailing scene number.
        text: Span<RichString>,
        /// The scene number between the `#`s. The range covers the number, not the `#`s.
        number: Option<Span<String>>,
    },
    Action {
        /// Written with a leading `!`, which only the first line can have.
        forced: bool,
        lines: Lines,
    },
    Dialogue(Dialogue),
    DualDialogue {
        primary: Span<Dialogue>,
        dual: Span<Dialogue>,
    },
    Lyrics(Lines),
    Transition {
        /// Written with a leading `>`.
        forced: bool,
        /// The transition text. The range covers the text only, without the leading `>`.
        text: Span<RichString>,
    },
    CenteredText(Lines),
    Synopsis(Lines),
    PageBreak,
}

/// A run of consecutive source lines, each with its own range.
///
/// A line that is only two spaces (an intentional blank line) is kept as an empty line.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Default)]
pub struct Lines(Vec<Span<RichString>>);

impl Lines {
    /// The lines, in order.
    pub fn iter(&self) -> std::slice::Iter<'_, Span<RichString>> {
        self.0.iter()
    }

    /// The number of lines.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Whether there are no lines.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Adds a line after the last one.
    pub(crate) fn push(&mut self, line: Span<RichString>) {
        self.0.push(line);
    }

    /// The lines joined into one [`RichString`] with a newline between each.
    pub fn join(&self) -> RichString {
        RichString::join(self.iter().map(|line| &line.inner), "\n")
    }
}

impl AsRef<[Span<RichString>]> for Lines {
    fn as_ref(&self) -> &[Span<RichString>] {
        &self.0
    }
}

impl From<Vec<Span<RichString>>> for Lines {
    fn from(lines: Vec<Span<RichString>>) -> Self {
        Lines(lines)
    }
}

impl<'a> IntoIterator for &'a Lines {
    type Item = &'a Span<RichString>;
    type IntoIter = std::slice::Iter<'a, Span<RichString>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Dialogue consisting of a character cue, parentheticals and lines.
/// A single [Dialogue] can have multiple parentheticals and lines.
///
/// NAME (extension)
/// (parenthetical)
/// Line.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Dialogue {
    pub character: Span<Character>,
    pub elements: Vec<Span<DialogueElement>>,
}

/// The character cue line of a [`Dialogue`].
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Character {
    /// Written with a leading `@`.
    pub forced: bool,
    /// The name. The range covers the name only, without the `@`, the extensions and the `^`.
    pub name: Span<RichString>,
    /// Every parenthesized extension, in order: `BOB (V.O.) (CONT'D)` has two.
    /// The range covers the text inside the parentheses.
    ///
    /// Typically only one will be display but we collect all for continuity.
    pub extensions: Vec<Span<RichString>>,
    /// The cue ends with `^`, which asks for the speech to be shown next to the one before it.
    ///
    /// True for the second half of an [`Element::DualDialogue`]. A `^` with nothing to pair with
    /// (the first speaker, or a third in a row) is kept as a plain [`Element::Dialogue`].
    pub caret: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum DialogueElement {
    /// A line wrapped in parentheses.
    Parenthetical(RichString),
    /// One source line of speech. Consecutive lines are consecutive elements, and a line that is
    /// only two spaces (an intentional blank line) is an empty line.
    Line(RichString),
}

/// The title page of the screenplay.
///
/// Fountain spec defines some common entries, these have functions to access
/// them. But other `Key: value` entries are also stored.
///
/// A key that is repeated gives repeated entries, which [`TitlePage::values`] 
/// reads as one block of values. This means `Title: A` followed by `Title: B`
/// means the same as `Title:` with `A` and `B` indented below.
///
/// Keys are looked up by name, so any key can be read without the parser having to know it.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct TitlePage {
    pub entries: Vec<TitleEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TitleEntry {
    /// The key as written, `Draft date` rather than `draft_date`,
    /// with the range of the key only.
    pub key: Span<String>,
    /// One value for `Key: value`, one per indented line for a block.
    pub values: Vec<Span<RichString>>,
}

impl TitleEntry {
    /// If this entry's key is `key`.
    ///
    /// Matching ignores ASCII case and surrounding whitespace, treats runs of 
    /// whitespace inside the key as one space, and counts `Author` and
    /// `Authors` as the same key.
    pub fn is_key(&self, key: &str) -> bool {
        normalize_key(&self.key.inner) == normalize_key(key)
    }
}

impl TitlePage {
    /// Whether the page has any entry the exporters display,
    /// which are the ones defined in the Fountian spec.
    pub fn has_displayed_entries(&self) -> bool {
        self.title()
            .chain(self.credit())
            .chain(self.authors())
            .chain(self.source())
            .chain(self.draft_date())
            .chain(self.contact())
            .next()
            .is_some()
    }

    /// The entries whose key is `key`, in source order.
    /// See [`TitleEntry::is_key`] for matching.
    pub fn entries(&self, key: &str) -> impl Iterator<Item = &TitleEntry> {
        let key = normalize_key(key);
        self.entries
            .iter()
            .filter(move |entry| normalize_key(&entry.key.inner) == key)
    }

    /// The values of every entry whose key is `key`, in source order.
    pub fn values(&self, key: &str) -> impl Iterator<Item = &Span<RichString>> {
        self.entries(key).flat_map(|entry| entry.values.iter())
    }

    pub fn title(&self) -> impl Iterator<Item = &Span<RichString>> {
        self.values("title")
    }

    pub fn credit(&self) -> impl Iterator<Item = &Span<RichString>> {
        self.values("credit")
    }

    /// The values of `Author` and `Authors`.
    pub fn authors(&self) -> impl Iterator<Item = &Span<RichString>> {
        self.values("author")
    }

    pub fn source(&self) -> impl Iterator<Item = &Span<RichString>> {
        self.values("source")
    }

    pub fn draft_date(&self) -> impl Iterator<Item = &Span<RichString>> {
        self.values("draft date")
    }

    pub fn contact(&self) -> impl Iterator<Item = &Span<RichString>> {
        self.values("contact")
    }
}

/// The form of a title page key that keys are compared in.
fn normalize_key(key: &str) -> String {
    let words: Vec<String> = key
        .split_whitespace()
        .map(|word| word.to_ascii_lowercase())
        .collect();
    let key = words.join(" ");
    if key == "authors" {
        "author".to_string()
    } else {
        key
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_col_is_one_based() {
        assert_eq!(line_col("abc\ndef", 0), (1, 1));
        assert_eq!(line_col("abc\ndef", 2), (1, 3));
        assert_eq!(line_col("abc\ndef", 4), (2, 1));
        assert_eq!(line_col("abc\ndef", 7), (2, 4));
    }

    #[test]
    fn line_col_counts_columns_in_characters() {
        // `å` is two bytes, so byte offset 3 is the third character.
        assert_eq!(line_col("ååb", 4), (1, 3));
    }

    #[test]
    fn line_col_clamps_past_the_end() {
        assert_eq!(line_col("a\nb", 100), (2, 2));
    }
}
