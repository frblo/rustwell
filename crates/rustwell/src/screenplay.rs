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
        heading: RichString,
    },
    Heading {
        slug: RichString,
        number: Option<String>,
    },
    Action(Lines),
    Dialogue(Dialogue),
    DualDialogue(Dialogue, Dialogue),
    Lyrics(Lines),
    Transition(RichString),
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

/// Dialogue consisting of a character name, an extension, parentheticals and lines.
/// A single [Dialogue] can have multiple parentheticals and lines.
///
/// NAME (extension)
/// (parenthetical)
/// Line.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Dialogue {
    pub character: RichString,
    pub extension: Option<RichString>,
    pub elements: Vec<Span<DialogueElement>>,
}

impl Dialogue {
    /// Creates an empty [Dialogue].
    pub fn new() -> Self {
        Self {
            character: RichString::new(),
            extension: None,
            elements: Vec::new(),
        }
    }
}

impl Default for Dialogue {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum DialogueElement {
    /// A line wrapped in parentheses.
    Parenthetical(RichString),
    /// One source line of speech. Consecutive lines are consecutive elements, and a line that is
    /// only two spaces (an intentional blank line) is an empty line.
    Line(RichString),
}

/// The information for a title page. Each field may be empty as none are strictly required
/// according to the fountain specification.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct TitlePage {
    pub title: Vec<Span<RichString>>,
    pub credit: Vec<Span<RichString>>,
    pub authors: Vec<Span<RichString>>,
    pub source: Vec<Span<RichString>>,
    pub draft_date: Vec<Span<RichString>>,
    pub contact: Vec<Span<RichString>>,
}

impl TitlePage {
    /// Creates a new empty [`TitlePage`].
    pub fn new() -> Self {
        Self {
            title: Vec::new(),
            credit: Vec::new(),
            authors: Vec::new(),
            source: Vec::new(),
            draft_date: Vec::new(),
            contact: Vec::new(),
        }
    }
}

impl Default for TitlePage {
    fn default() -> Self {
        Self::new()
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
