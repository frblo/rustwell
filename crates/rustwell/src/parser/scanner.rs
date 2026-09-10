//! Offset-preserving scan of Fountain source.
//!
//! [`scan`] splits the source into logical lines that borrow byte ranges from
//! the original string instead of rewriting it into owned per-line [`String`]s.
//! Boneyards (`/* */`) and notes (`[[ ]]`) are recorded rather than deleted.

use core::range::Range;
use std::borrow::Cow;

/// A byte range into the source.
pub type Span = Range<usize>;

/// A Fountain source string split into logical lines, with the boneyards and
/// notes that were removed from them kept aside.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scan {
    pub lines: Vec<LogicalLine>,
    pub boneyards: Vec<Boneyard>,
    pub notes: Vec<Note>,
}

/// One logical line of the source.
///
/// Usually one physical line, but a boneyard or note that contains newlines
/// fuses the physical lines it spans into a single logical line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicalLine {
    /// 1-based physical line number where the logical line begins.
    pub start_line: usize,
    /// Byte range of the logical line, excluding the terminating newline.
    pub range: Span,
    /// The above `range` split into visible text and removed fragments.
    /// Concatenating the [`Segment::Text`] pieces gives the line's content.
    pub segments: Vec<Segment>,
}

impl LogicalLine {
    /// The visible content of the line.
    ///
    /// This is done by joinging its [`Segment::Text`] pieces.
    /// Borrows when possible, that is when there is zero or one [`Segment::Text`].
    pub fn text<'s>(&self, src: &'s str) -> Cow<'s, str> {
        let mut texts = self.segments.iter().filter_map(|segment| match segment {
            Segment::Text(span) => Some(&src[*span]),
            Segment::Ignored(_) => None,
        });
        match (texts.next(), texts.next()) {
            (None, _) => Cow::Borrowed(""),
            (Some(only), None) => Cow::Borrowed(only),
            (Some(first), Some(second)) => {
                let mut joined = String::from(first);
                joined.push_str(second);
                joined.extend(texts);
                Cow::Owned(joined)
            }
        }
    }
}

/// A slice of a [`LogicalLine`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Segment {
    /// Visible screenplay text.
    Text(Span),
    /// A removed fragment of the text.
    ///
    /// Either a boneyard or a note, these are also tracked on the
    /// resulting [`Scan`] struct.
    Ignored(Span),
}

/// A `/* */` boneyard located in the source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Boneyard {
    /// The whole boneyard including eventual delimiters.
    pub span: Span,
    /// The content between the delimiters.
    pub inner: Span,
}

/// A `[[ ]]` closed note located in the source.
///
/// A `[[` with no matching `]]` before a blank line or end of input is
/// not closed and therefore not a [`Note`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Note {
    /// The whole note including eventual delimiters.
    pub span: Span,
    /// The content between the delimiters.
    pub inner: Span,
}

/// [`scan`] splits `src` into logical lines without allocating a rewritten copy,
/// it pulls boneyards and notes aside as it goes.
///
/// NOTE: Tabs are left as-is, later concern.
pub fn scan(src: &str) -> Scan {
    let mut lines = Vec::new();
    let mut boneyards = Vec::new();
    let mut notes = Vec::new();

    let mut physical_line = 1;

    let mut line_start = 0;
    let mut line_start_line = 1;

    let mut segments: Vec<Segment> = Vec::new();

    let mut segement_text_start = 0;

    let mut pos = 0usize;
    loop {
        let rest = &src[pos..];

        let next_note = rest.find("[[").map(|i| pos + i);
        let next_boneyard = rest.find("/*").map(|i| pos + i);
        let next_newline = rest.find('\n').map(|i| pos + i);
        let earliest = [next_note, next_boneyard, next_newline]
            .into_iter()
            .flatten()
            .min();

        match earliest {
            Some(open) if earliest == next_note => match note_close(src, open) {
                Some(close) => {
                    push_text(&mut segments, segement_text_start, open);
                    let span = Span::from(open..close);
                    let inner = Span::from(open + 2..close - 2);
                    physical_line += newlines(&src[span]);
                    segments.push(Segment::Ignored(span));
                    notes.push(Note { span, inner });
                    collect_boneyards(src, inner, &mut boneyards);
                    segement_text_start = close;
                    pos = close;
                }
                // Not a note so the `[[` is literal, keep it in the pending text run.
                None => pos = open + 2,
            },
            Some(open) if earliest == next_boneyard => {
                push_text(&mut segments, segement_text_start, open);
                let close = src[open + 2..].find("*/").map(|i| open + 2 + i);
                let span = Span::from(open..close.map_or(src.len(), |c| c + 2));
                let inner = Span::from(open + 2..close.unwrap_or(src.len()));
                physical_line += newlines(&src[inner]);
                segments.push(Segment::Ignored(span));
                boneyards.push(Boneyard { span, inner });
                segement_text_start = span.end;
                pos = span.end;
            }
            Some(nl) => {
                push_text(&mut segments, segement_text_start, nl);
                lines.push(LogicalLine {
                    start_line: line_start_line,
                    range: Span::from(line_start..nl),
                    segments: std::mem::take(&mut segments),
                });
                physical_line += 1;
                line_start = nl + 1;
                line_start_line = physical_line;
                segement_text_start = nl + 1;
                pos = nl + 1;
            }
            None => {
                push_text(&mut segments, segement_text_start, src.len());
                let line = LogicalLine {
                    start_line: line_start_line,
                    range: Span::from(line_start..src.len()),
                    segments,
                };
                // The final line is dropped when it has no visible text.
                //
                // Can happen because off:
                // A trailing newline, a note-only tail, or empty input.
                if !line.text(src).is_empty() {
                    lines.push(line);
                }
                break;
            }
        }
    }

    Scan {
        lines,
        boneyards,
        notes,
    }
}

/// If the `[[` at `open` starts a closed note, returns the offset just past the
/// closing `]]`. Returns `None` when the `[[` is only literal text, because no
/// `]]` arrives before an empty line or end of input.
///
/// An empty line in this context means: with every `/* */` span deleted,
/// two newlines separated by nothing or a single space.
fn note_close(src: &str, open: usize) -> Option<usize> {
    let mut i = open + 2;

    // Whether a real newline has been seen yet, the `[[`'s own line cannot be
    // the blank line that breaks a note.
    let mut seen_newline = false;
    // The current line since that newline, with boneyard spans deleted: whether
    // it is all spaces, and how many bytes long. "" or " " breaks the note.
    let mut all_spaces = true;
    let mut len = 0usize;

    loop {
        let rest = &src[i..];
        let close = rest.find("]]").map(|k| i + k);
        let boneyard = rest.find("/*").map(|k| i + k);
        let newline = rest.find('\n').map(|k| i + k);
        let first = [close, boneyard, newline].into_iter().flatten().min();

        match first {
            None => return None,
            Some(p) if first == close => return Some(p + 2),
            // Boneyard gets skipped here and collected later.
            Some(p) if first == boneyard => {
                let visible = &src[Span::from(i..p)];
                all_spaces &= visible.bytes().all(|b| b == b' ');
                len += visible.len();
                i = src[p + 2..].find("*/").map_or(src.len(), |k| p + 2 + k + 2);
            }
            Some(p) => {
                let visible = &src[Span::from(i..p)];
                all_spaces &= visible.bytes().all(|b| b == b' ');
                len += visible.len();
                if seen_newline && all_spaces && len <= 1 {
                    return None;
                }
                seen_newline = true;
                i = p + 1;
                all_spaces = true;
                len = 0;
            }
        }
    }
}

/// The number of newlines in a &str.
fn newlines(text: &str) -> usize {
    text.bytes().filter(|&b| b == b'\n').count()
}

/// Pushes a [`Segment::Text`] to the list,
/// guards so that zero size segments aren't appended.
fn push_text(segments: &mut Vec<Segment>, start: usize, end: usize) {
    if start < end {
        segments.push(Segment::Text(Span::from(start..end)));
    }
}

/// Collects all boneyard spans inside a range.
///
/// Doesn't create any segments since this range is already processed.
fn collect_boneyards(src: &str, range: Span, out: &mut Vec<Boneyard>) {
    let mut i = range.start;
    while let Some(rel) = src[Span::from(i..range.end)].find("/*") {
        let open = i + rel;
        let close = src[Span::from(open + 2..range.end)]
            .find("*/")
            .map(|k| open + 2 + k);
        let span = Span::from(open..close.map_or(range.end, |c| c + 2));
        out.push(Boneyard {
            span,
            inner: Span::from(open + 2..close.unwrap_or(range.end)),
        });
        i = span.end;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lines(src: &str) -> Vec<(usize, String)> {
        scan(src)
            .lines
            .iter()
            .map(|l| (l.start_line, l.text(src).into_owned()))
            .collect()
    }

    fn assert_lines(src: &str, expected: &[(usize, &str)]) {
        let expected: Vec<(usize, String)> = expected
            .iter()
            .map(|(n, s)| (*n, (*s).to_string()))
            .collect();
        assert_eq!(lines(src), expected);
    }

    #[test]
    fn splits_on_newlines() {
        assert_lines("Hello\nWorld", &[(1, "Hello"), (2, "World")]);
    }

    #[test]
    fn empty_input_has_no_lines() {
        assert!(scan("").lines.is_empty());
    }

    #[test]
    fn trailing_newline_adds_no_line() {
        assert_lines("Hello\n", &[(1, "Hello")]);
    }

    #[test]
    fn blank_lines_are_kept() {
        assert_lines(
            "\n\na\n\nb",
            &[(1, ""), (2, ""), (3, "a"), (4, ""), (5, "b")],
        );
    }

    #[test]
    fn tabs_are_not_expanded() {
        assert_lines("a\tb", &[(1, "a\tb")]);
    }

    #[test]
    fn line_ranges_are_byte_offsets() {
        // "café " is 6 bytes, the em dash is 3.
        let s = scan("café —\nx");
        assert_eq!(s.lines[0].range, Span::from(0..9));
        assert_eq!(s.lines[1].range, Span::from(10..11));
        assert!(s.boneyards.is_empty());
        assert!(s.notes.is_empty());
    }

    #[test]
    fn boneyard_mid_line_joins_the_surrounding_text() {
        assert_lines("Hello /* removed */ World", &[(1, "Hello  World")]);
        let s = scan("Hello /* removed */ World");
        assert_eq!(
            s.boneyards,
            [Boneyard {
                span: Span::from(6..19),
                inner: Span::from(8..17),
            }]
        );
        assert_eq!(
            s.lines[0].segments,
            [
                Segment::Text(Span::from(0..6)),
                Segment::Ignored(Span::from(6..19)),
                Segment::Text(Span::from(19..25)),
            ]
        );
    }

    #[test]
    fn boneyard_on_its_own_line_leaves_an_empty_line() {
        assert_lines(
            "Hello\n/* removed */\nWorld",
            &[(1, "Hello"), (2, ""), (3, "World")],
        );
    }

    #[test]
    fn multiline_boneyard_fuses_the_lines_it_spans() {
        assert_lines(
            "Hello\n/* multi\nline\nboneyard */\nWorld",
            &[(1, "Hello"), (2, ""), (5, "World")],
        );
    }

    #[test]
    fn multiline_boneyard_mid_line() {
        assert_lines("Hello /* multi\nline */ World", &[(1, "Hello  World")]);
    }

    #[test]
    fn blank_source_lines_inside_a_boneyard_do_not_count_as_lines() {
        assert_lines(
            "Line1\n/*\n\n\n*/\nLine2",
            &[(1, "Line1"), (2, ""), (6, "Line2")],
        );
    }

    #[test]
    fn unterminated_boneyard_runs_to_end_of_input() {
        assert_lines("foo /* bar", &[(1, "foo ")]);
        let s = scan("foo /* bar");
        assert_eq!(
            s.boneyards,
            [Boneyard {
                span: Span::from(4..10),
                inner: Span::from(6..10),
            }]
        );
    }

    #[test]
    fn closed_note_is_removed_and_recorded() {
        assert_lines("Hello [[a note]] World", &[(1, "Hello  World")]);
        let s = scan("Hello [[a note]] World");
        assert_eq!(
            s.notes,
            [Note {
                span: Span::from(6..16),
                inner: Span::from(8..14),
            }]
        );
    }

    #[test]
    fn multiline_note_fuses_the_lines_it_spans() {
        assert_lines("Hello [[a\nnote]] World", &[(1, "Hello  World")]);
    }

    #[test]
    fn source_lines_are_preserved_after_a_multiline_note() {
        assert_lines(
            "Line1\n[[a\nmultiline\nnote]]\nLine2",
            &[(1, "Line1"), (2, ""), (5, "Line2")],
        );
    }

    #[test]
    fn a_blank_line_breaks_an_unclosed_note_which_reverts_to_text() {
        assert_lines(
            "Hello [[a note\n\nWorld",
            &[(1, "Hello [[a note"), (2, ""), (3, "World")],
        );
        assert!(scan("Hello [[a note\n\nWorld").notes.is_empty());
    }

    #[test]
    fn a_single_space_line_also_breaks_an_unclosed_note() {
        assert_lines(
            "Hello [[a note\n \nWorld",
            &[(1, "Hello [[a note"), (2, " "), (3, "World")],
        );
    }

    #[test]
    fn a_line_with_a_letter_does_not_break_an_unclosed_note() {
        assert_lines(
            "Hello [[a note,\na\nWorld",
            &[(1, "Hello [[a note,"), (2, "a"), (3, "World")],
        );
    }

    #[test]
    fn note_unclosed_at_end_of_input_reverts_to_text() {
        assert_lines("Hello [[a note", &[(1, "Hello [[a note")]);
        assert!(scan("Hello [[a note").notes.is_empty());
    }

    #[test]
    fn boneyards_inside_a_closed_note_are_still_recorded() {
        let src = "Hello [[a /* b1 */ note /* b2 */ here]] World";
        assert_lines(src, &[(1, "Hello  World")]);
        let s = scan(src);
        assert_eq!(s.notes.len(), 1);
        assert_eq!(s.boneyards.len(), 2);
    }

    #[test]
    fn a_boneyard_only_line_inside_a_note_breaks_it() {
        assert_lines(
            "Hello [[a note\n/* boneyard */\nWorld",
            &[(1, "Hello [[a note"), (2, ""), (3, "World")],
        );
    }

    #[test]
    fn a_note_that_opens_at_end_of_line_can_close_on_the_next() {
        // Regression test so that when nothing follows `[[` on its line,
        // that doesn't count as a blank line which ends the note.
        let src = "Some action [[\nTODO fix ]]\nMore action";
        assert_lines(src, &[(1, "Some action "), (3, "More action")]);
        assert_eq!(scan(src).notes.len(), 1);
    }

    #[test]
    fn two_spaces_around_a_boneyard_do_not_break_the_note() {
        // With the boneyard deleted the middle line is "  " (two spaces), which
        // is not a blank line, so the note stays open and closes at `]]`.
        let src = "a [[X\n  /* c */\nY]] z";
        assert_lines(src, &[(1, "a  z")]);
        assert_eq!(scan(src).notes.len(), 1);
    }

    #[test]
    fn boneyard_then_note_on_consecutive_lines() {
        assert_lines(
            "/* boneyard */\n[[a note]]\nWorld",
            &[(1, ""), (2, ""), (3, "World")],
        );
    }

    #[test]
    fn a_bare_double_bracket_is_literal_text() {
        assert_lines("This is [[ not right", &[(1, "This is [[ not right")]);
    }
}
