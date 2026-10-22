mod scanner;

use std::borrow::Cow;
use std::iter::Peekable;

use crate::rich_string::RichString;
use crate::screenplay::Dialogue;
use crate::screenplay::DialogueElement;
use crate::screenplay::Element;
use crate::screenplay::Lines;
use crate::screenplay::Screenplay;
use crate::screenplay::SourceRange;
use crate::screenplay::Span;
use crate::screenplay::TitlePage;
use scanner::LogicalLine;
use scanner::Scan;

/// Parses a Fountain source string into a [`Screenplay`] structure.
///
/// Preprocesses the source text by removing
/// boneyards, notes and normalizing tabs to spaces.
///
/// # Examples
///
/// ```
/// use rustwell::parse;
///
/// let input = r#"
/// Title: Example Screenplay
///
/// INT. ROOM – DAY
/// A man stands alone.
/// "#;
///
/// let screenplay = parse(input);
/// assert!(screenplay.elements.len() > 0);
/// ```
#[must_use]
pub fn parse(src: &str) -> Screenplay {
    let Scan {
        lines,
        notes,
        boneyards,
    } = scanner::scan(src);
    let (titlepage, elements) = Parser::new(src, &lines).parse();
    Screenplay {
        titlepage,
        elements,
        notes,
        boneyards,
    }
}

/// The text of a forced heading (`.HEADING`), if `trimmed` is one.
///
/// A lone `.` followed by something non-alphanumeric (like `...`) is not forced.
fn forced_heading_text(trimmed: &str) -> Option<&str> {
    let rest = trimmed.strip_prefix('.')?;
    rest.chars().next()?.is_alphanumeric().then_some(rest)
}

/// If `trimmed` begins with a scene heading prefix (`INT.`, `EXT `, `I/E.` and so on).
fn starts_like_heading(trimmed: &str) -> bool {
    const PREFIXES: [&str; 6] = ["INT", "EXT", "EST", "I/E", "INT./EXT", "INT/EXT"];
    let bytes = trimmed.as_bytes();

    PREFIXES.iter().any(|prefix| {
        let n = prefix.len();
        bytes
            .get(..n)
            .is_some_and(|head| head.eq_ignore_ascii_case(prefix.as_bytes()))
            && bytes.get(n).is_some_and(|&end| end == b' ' || end == b'.')
    })
}

/// Splits a trailing scene number (`#S.1#`) off the heading text.
fn split_heading_number(text: &str) -> (&str, Option<String>) {
    if let Some(start) = text.trim_end().strip_suffix('#')
        && let Some((text, number)) = start.rsplit_once('#')
        && number
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '.')
    {
        return (text.trim_end(), Some(number.to_string()));
    }

    (text, None)
}

/// If `trimmed` looks like a character cue: all caps outside of any extension,
/// with at least one letter.
fn looks_like_character(trimmed: &str) -> bool {
    let head = trimmed.split_once('(').map_or(trimmed, |(head, _)| head);
    head.chars().any(char::is_alphabetic) && !head.chars().any(char::is_lowercase)
}

/// Expands tabs to four spaces, borrowing when there are none.
fn expand_tabs(text: Cow<'_, str>) -> Cow<'_, str> {
    if text.contains('\t') {
        Cow::Owned(text.replace('\t', "    "))
    } else {
        text
    }
}

/// A line of the source as the parser rules see it.
struct LineView<'t> {
    /// The text of the line, with boneyards and notes removed and tabs expanded.
    raw: &'t str,
    /// `raw` with surrounding whitespace removed.
    trimmed: &'t str,
    /// The byte range of the line in the source.
    range: SourceRange,
}

/// The kinds of element made of consecutive lines.
#[derive(Debug, Clone, Copy)]
enum Block {
    Action,
    Lyrics,
    Centered,
    Synopsis,
}

impl Block {
    /// The lines of `element` if it is a block of this kind.
    fn lines_mut(self, element: &mut Element) -> Option<&mut Lines> {
        match (self, element) {
            (Block::Action, Element::Action(lines))
            | (Block::Lyrics, Element::Lyrics(lines))
            | (Block::Centered, Element::CenteredText(lines))
            | (Block::Synopsis, Element::Synopsis(lines)) => Some(lines),
            _ => None,
        }
    }

    /// A new element of this kind holding `lines`.
    fn element(self, lines: Lines) -> Element {
        match self {
            Block::Action => Element::Action(lines),
            Block::Lyrics => Element::Lyrics(lines),
            Block::Centered => Element::CenteredText(lines),
            Block::Synopsis => Element::Synopsis(lines),
        }
    }
}

/// A parser rule. Returns `true` if it claimed the line, having added it to
/// the screenplay, and `false` to leave the line to the next rule.
type Rule = for<'a, 'v> fn(&mut Parser<'a>, &LineView<'v>) -> bool;

/// Builds a `&[Rule]` from the names of [`Parser`] methods.
macro_rules! rules {
    ($($rule:ident),+ $(,)?) => {
        &[$(|parser, line| parser.$rule(line)),+]
    };
}

/// The defaults rules for a line in priority order: the first rule to claim
/// a line wins.
const DEFAULT_RULES: &[Rule] = rules![
    section,
    // `===` is a page break and `=` a synopsis, so page breaks go first.
    page_break,
    synopsis,
    // `!` forces action, so it must beat everything that would otherwise match.
    forced_action,
    centered,
    lyrics,
    // A forced heading (`.`) or an INT/EXT prefix.
    heading,
    // `>` forces a transition, but `> text <` is centered, which comes first.
    transition,
    // All caps with something following, so after the more specific rules.
    dialogue_start,
    // Anything not claimed above, so this must stay last.
    action,
];

/// The rules for a line directly after centered text, lyrics, a synopsis or action.
const IN_BLOCK_RULES: &[Rule] = rules![centered, lyrics, synopsis, action];

/// Internal parser state machine for Fountain.
///
/// Keeps an iterator of the source, a accumulative list of [`Element`]s, and
/// a state. Also tracks a [`TitlePage`] if such exists in the source.
struct Parser<'a> {
    src: &'a str,
    lines: Peekable<std::slice::Iter<'a, LogicalLine>>,
    state: State,
    elements: Vec<Span<Element>>,
    title_page: Option<TitlePage>,
}

impl<'a> Parser<'a> {
    /// Create new parser
    fn new(src: &'a str, lines: &'a [LogicalLine]) -> Self {
        Self {
            src,
            lines: lines.iter().peekable(),
            state: State::Default,
            elements: Vec::new(),
            title_page: None,
        }
    }

    /// Main entry point for parser
    ///
    /// Starts by parsing a potential title. Before moving on to the main loop.
    /// A line with two or more spaces is always treated as intentional empty lines.
    ///
    /// Each line is trimmed once, into a [`LineView`] that the rules read.
    fn parse(mut self) -> (Option<TitlePage>, Vec<Span<Element>>) {
        self.parse_title();
        while let Some(logical) = self.lines.next() {
            let range = logical.range;
            let line = self.text(logical);
            let view = LineView {
                raw: &line,
                trimmed: line.trim(),
                range,
            };

            if view.trimmed.is_empty() && !view.raw.starts_with("  ") {
                self.state = State::Default;
                continue;
            }

            match self.state {
                State::Default => self.apply_rules(DEFAULT_RULES, &view),
                State::InDialogue => {
                    let (curr_dialogue, element_span) = self
                        .get_last_dialogue()
                        .expect("Must exist since we are in dialogue block");
                    element_span.end = view.range.end;

                    if view.trimmed.starts_with('(') && view.trimmed.ends_with(')') {
                        curr_dialogue.elements.push(Span::new(
                            DialogueElement::Parenthetical(RichString::from(view.trimmed)),
                            view.range,
                        ));
                        continue;
                    }

                    curr_dialogue.elements.push(Span::new(
                        DialogueElement::Line(RichString::from(view.trimmed)),
                        view.range,
                    ));
                }
                State::InBlock => self.apply_rules(IN_BLOCK_RULES, &view),
            }
        }

        (self.title_page, self.elements)
    }

    /// The text of `line` as the parser sees it: boneyards and notes removed and
    /// tabs expanded.
    fn text(&self, line: &'a LogicalLine) -> Cow<'a, str> {
        expand_tabs(line.text(self.src))
    }

    /// Runs `rules` in order on `line`, stopping at the first that claims it.
    fn apply_rules(&mut self, rules: &[Rule], line: &LineView) {
        for rule in rules {
            if rule(self, line) {
                return;
            }
        }
    }

    /// Recognizes a section (`#`). Sections do not produce an element yet.
    fn section(&mut self, line: &LineView) -> bool {
        line.trimmed.starts_with('#')
    }

    fn synopsis(&mut self, line: &LineView) -> bool {
        // `===` is a page break, which only starts a line outside a block.
        if line.trimmed.starts_with("===") {
            return false;
        }
        let Some(inner) = line.trimmed.strip_prefix('=') else {
            return false;
        };

        self.push_block_line(Block::Synopsis, inner, line.range);
        true
    }

    fn page_break(&mut self, line: &LineView) -> bool {
        if !line.trimmed.starts_with("===") {
            return false;
        }

        self.elements
            .push(Span::new(Element::PageBreak, line.range));
        true
    }

    fn forced_action(&mut self, line: &LineView) -> bool {
        let Some(inner) = line.trimmed.strip_prefix('!') else {
            return false;
        };

        self.push_block_line(Block::Action, inner, line.range);
        true
    }

    fn centered(&mut self, line: &LineView) -> bool {
        let Some(inner) = line
            .trimmed
            .strip_prefix('>')
            .and_then(|u| u.strip_suffix('<'))
        else {
            return false;
        };

        self.push_block_line(Block::Centered, inner.trim(), line.range);
        true
    }

    fn lyrics(&mut self, line: &LineView) -> bool {
        let Some(inner) = line.trimmed.strip_prefix('~') else {
            return false;
        };

        self.push_block_line(Block::Lyrics, inner, line.range);
        true
    }

    fn action(&mut self, line: &LineView) -> bool {
        self.push_block_line(Block::Action, line.raw, line.range);
        true
    }

    /// Adds a line to the block element being built, or starts a new one.
    ///
    /// The line continues the last element if that is a block of the same kind and the parser
    /// is in a block, otherwise it begins a new element.
    fn push_block_line(&mut self, block: Block, text: &str, range: SourceRange) {
        let line = Span::new(RichString::from(text), range);

        if self.state == State::InBlock
            && let Some(last) = self.elements.last_mut()
            && let Some(lines) = block.lines_mut(&mut last.inner)
        {
            lines.push(line);
            last.span.end = range.end;
            return;
        }

        self.elements
            .push(Span::new(block.element(vec![line].into()), range));
        self.state = State::InBlock;
    }

    fn heading(&mut self, line: &LineView) -> bool {
        let text = if let Some(forced) = forced_heading_text(line.trimmed) {
            forced
        } else if starts_like_heading(line.trimmed) && self.next_line_is_empty() {
            line.trimmed
        } else {
            return false;
        };

        let (text, number) = split_heading_number(text);
        self.elements.push(Span::new(
            Element::Heading {
                slug: RichString::from(text),
                number,
            },
            line.range,
        ));

        self.lines.next();
        true
    }

    fn get_last_dialogue(&mut self) -> Option<(&mut Dialogue, &mut SourceRange)> {
        let Some(Span {
            span,
            inner: Element::Dialogue(curr_dialogue) | Element::DualDialogue(_, curr_dialogue),
        }) = self.elements.last_mut()
        else {
            return None;
        };

        Some((curr_dialogue, span))
    }

    fn insert_empty_dialogue<'s>(&mut self, inner: &'s str, range: SourceRange) -> &'s str {
        let new_dialogue = Dialogue::new();

        if let Some(stripped) = inner.trim_end().strip_suffix('^')
            && let Some(&Span {
                inner: Element::Dialogue(_),
                ..
            }) = self.elements.last()
            && let Some(Span {
                span,
                inner: Element::Dialogue(d),
            }) = self.elements.pop()
        {
            self.elements.push(Span::new(
                Element::DualDialogue(d, new_dialogue),
                SourceRange::from(span.start..range.end),
            ));
            return stripped.trim_end();
        }

        self.elements
            .push(Span::new(Element::Dialogue(new_dialogue), range));
        inner
    }

    fn dialogue_start(&mut self, line: &LineView) -> bool {
        let character = if let Some(forced) = line.trimmed.strip_prefix('@') {
            forced
        } else if looks_like_character(line.trimmed) && !self.next_line_is_empty() {
            line.trimmed
        } else {
            return false;
        };

        let mut character = self.insert_empty_dialogue(character, line.range);

        let (curr_dialogue, _) = self
            .get_last_dialogue()
            .expect("Just pushed to list, must exist");

        if let Some((head, tail)) = character.split_once('(')
            && let Some((extension, _)) = tail.split_once(')')
        {
            curr_dialogue.extension = Some(RichString::from(extension));
            character = head.trim_end();
        }

        curr_dialogue.character = RichString::from(character);

        self.state = State::InDialogue;
        true
    }

    fn transition(&mut self, line: &LineView) -> bool {
        let text = if let Some(forced) = line.trimmed.strip_prefix('>')
            && !line.trimmed.ends_with('<')
        {
            forced
        } else if line.trimmed.ends_with("TO:")
            && !line.trimmed.chars().any(char::is_lowercase)
            && self.next_line_is_empty()
        {
            line.trimmed
        } else {
            return false;
        };

        self.elements.push(Span::new(
            Element::Transition(RichString::from(text)),
            line.range,
        ));

        self.lines.next();
        true
    }

    fn parse_title(&mut self) {
        let mut tp = TitlePage::new();

        while let Some(&logical) = self.lines.peek() {
            let range = logical.range;
            let line = self.text(logical);
            let Some((key, val)) = line.split_once(':') else {
                break;
            };
            self.lines.next(); // Consume the key line

            let mut values = Vec::new();

            if val.trim().is_empty() {
                values = self.take_indented_block();
            } else {
                values.push(Span::new(RichString::from(val), range));
            }

            match key.trim().to_ascii_uppercase().as_str() {
                "TITLE" => tp.title = values,
                "CREDIT" => tp.credit = values,
                "AUTHOR" | "AUTHORS" => tp.authors = values,
                "SOURCE" => tp.source = values,
                "DRAFT DATE" => tp.draft_date = values,
                "CONTACT" => tp.contact = values,
                _ => (),
            }
        }

        if self.next_line_is_empty() {
            self.lines.next();
        }

        if !tp.title.is_empty()
            || !tp.credit.is_empty()
            || !tp.authors.is_empty()
            || !tp.source.is_empty()
            || !tp.draft_date.is_empty()
            || !tp.contact.is_empty()
        {
            self.title_page = Some(tp);
        }
    }

    fn take_indented_block(&mut self) -> Vec<Span<RichString>> {
        let mut out = Vec::new();
        while let Some(&logical) = self.lines.peek() {
            let next = self.text(logical);
            if next.starts_with("   ") {
                self.lines.next();
                out.push(Span::new(RichString::from(next.trim()), logical.range));
            } else {
                break;
            }
        }
        out
    }

    fn next_line_is_empty(&mut self) -> bool {
        self.lines
            .peek()
            .is_none_or(|line| line.text(self.src).trim().is_empty())
    }
}

#[derive(Debug, PartialEq, Eq)]
/// The different states the state machine can be in.
enum State {
    Default,
    InDialogue,
    InBlock,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tabs_expand_to_four_spaces() {
        assert_eq!(expand_tabs("a\tb".into()), "a    b");
        assert_eq!(expand_tabs("\tc".into()), "    c");
        assert!(matches!(expand_tabs("abc".into()), Cow::Borrowed(_)));
    }

    mod end_to_end_parse {
        use super::*;

        macro_rules! test_screenplay {
            ($name:ident, $input:expr, [$($elem:expr),*]) => {
                #[test]
                fn $name() {
                test_parse($input, [$($elem),*]);
                }
            };
        }

        fn zero_span() -> SourceRange {
            SourceRange::from(0..0)
        }

        fn span<T>(inner: T) -> Span<T> {
            Span::new(inner, zero_span())
        }

        /// Lines with the given text and zeroed ranges.
        fn lines(texts: &[&str]) -> Lines {
            texts
                .iter()
                .map(|text| span((*text).into()))
                .collect::<Vec<_>>()
                .into()
        }

        /// Zeroes the spans of the lines and dialogue elements inside an [`Element`].
        fn without_spans(element: &Element) -> Element {
            let zero_dialogue = |d: &Dialogue| {
                let mut d = d.clone();
                for e in &mut d.elements {
                    e.span = zero_span();
                }
                d
            };
            let zero_lines = |l: &Lines| {
                l.iter()
                    .map(|line| span(line.inner.clone()))
                    .collect::<Vec<_>>()
                    .into()
            };
            match element {
                Element::Dialogue(d) => Element::Dialogue(zero_dialogue(d)),
                Element::DualDialogue(l, r) => {
                    Element::DualDialogue(zero_dialogue(l), zero_dialogue(r))
                }
                Element::Action(l) => Element::Action(zero_lines(l)),
                Element::Lyrics(l) => Element::Lyrics(zero_lines(l)),
                Element::CenteredText(l) => Element::CenteredText(zero_lines(l)),
                Element::Synopsis(l) => Element::Synopsis(zero_lines(l)),
                other => other.clone(),
            }
        }

        fn test_parse(input: &str, expected: impl IntoIterator<Item = Element>) {
            let parsed = parse(input);
            for (Span { inner: actual, .. }, expected) in parsed.elements.iter().zip(expected) {
                assert_eq!(without_spans(actual), expected);
            }
        }

        test_screenplay!(
            parses_heading_without_number,
            "InT. OUTSIDE - DAY",
            [Element::Heading {
                slug: "InT. OUTSIDE - DAY".into(),
                number: None,
            }]
        );

        test_screenplay!(
            does_not_parse_heading_whitout_dot,
            "Intro music plays.",
            [Element::Action(lines(&["Intro music plays."]))]
        );

        test_screenplay!(
            parses_heading_with_number,
            "INT. OUTSIDE - DAY #S.1#",
            [Element::Heading {
                slug: "INT. OUTSIDE - DAY".into(),
                number: Some("S.1".to_string()),
            }]
        );

        test_screenplay!(
            parses_heading_forced,
            ".OUTSIDE - DAY",
            [Element::Heading {
                slug: "OUTSIDE - DAY".into(),
                number: None,
            }]
        );

        test_screenplay!(
            parses_heading_forced_with_number,
            ".OUTSIDE - DAY #S.1#",
            [Element::Heading {
                slug: "OUTSIDE - DAY".into(),
                number: Some("S.1".to_string()),
            }]
        );

        test_screenplay!(
            parses_action,
            "They look at the test output - it's all failing.",
            [Element::Action(lines(&[
                "They look at the test output - it's all failing."
            ]))]
        );

        test_screenplay!(
            parses_action_forced,
            "!INT. They look at the test output - it's all failing.",
            [Element::Action(lines(&[
                "INT. They look at the test output - it's all failing."
            ]))]
        );

        test_screenplay!(
            parses_dialogue_without_extension,
            r"
CHAR
(sad)
Nooo!
(angry)
I am angry.",
            [Element::Dialogue(Dialogue {
                character: "CHAR".into(),
                extension: None,
                elements: vec![
                    span(DialogueElement::Parenthetical("(sad)".into())),
                    span(DialogueElement::Line("Nooo!".into())),
                    span(DialogueElement::Parenthetical("(angry)".into())),
                    span(DialogueElement::Line("I am angry.".into())),
                ],
            })]
        );

        test_screenplay!(
            parses_dialogue_with_extension,
            r"
CHAR (V.O)
(sad)
Nooo!",
            [Element::Dialogue(Dialogue {
                character: "CHAR".into(),
                extension: Some("V.O".into()),
                elements: vec![
                    span(DialogueElement::Parenthetical("(sad)".into())),
                    span(DialogueElement::Line("Nooo!".into())),
                ],
            })]
        );

        test_screenplay!(
            parses_dialogue_without_extension_forced,
            r"
@char
(sad)
Nooo!
(angry)
I am angry.",
            [Element::Dialogue(Dialogue {
                character: "char".into(),
                extension: None,
                elements: vec![
                    span(DialogueElement::Parenthetical("(sad)".into())),
                    span(DialogueElement::Line("Nooo!".into())),
                    span(DialogueElement::Parenthetical("(angry)".into())),
                    span(DialogueElement::Line("I am angry.".into())),
                ],
            })]
        );

        test_screenplay!(
            parses_dialogue_with_extension_forced,
            r"
@char (V.O)
(sad)
Nooo!",
            [Element::Dialogue(Dialogue {
                character: "char".into(),
                extension: Some("V.O".into()),
                elements: vec![
                    span(DialogueElement::Parenthetical("(sad)".into())),
                    span(DialogueElement::Line("Nooo!".into())),
                ],
            })]
        );

        test_screenplay!(
            parses_dual_dialogue,
            r"
@CHaR
(sad)
Nooo!

CHOR (V.O) ^
YES!",
            [Element::DualDialogue(
                Dialogue {
                    character: "CHaR".into(),
                    extension: None,
                    elements: vec![
                        span(DialogueElement::Parenthetical("(sad)".into())),
                        span(DialogueElement::Line("Nooo!".into())),
                    ],
                },
                Dialogue {
                    character: "CHOR".into(),
                    extension: Some("V.O".into()),
                    elements: vec![span(DialogueElement::Line("YES!".into()))],
                },
            )]
        );

        test_screenplay!(
            parses_lyrics,
            "~Hey ho let's go",
            [Element::Lyrics(lines(&["Hey ho let's go"]))]
        );

        test_screenplay!(
            parses_transition,
            "\nCUT TO:\n",
            [Element::Transition("CUT TO:".into())]
        );

        test_screenplay!(
            parses_transition_forced,
            ">Camera does a spin",
            [Element::Transition("Camera does a spin".into())]
        );

        test_screenplay!(
            parses_centered,
            "> The end    <",
            [Element::CenteredText(lines(&["The end"]))]
        );

        test_screenplay!(parses_pagebreak_with_3_equals, "===", [Element::PageBreak]);

        test_screenplay!(
            parses_pagebreak_with_8_equals,
            "========",
            [Element::PageBreak]
        );

        test_screenplay!(
            parses_synopsis,
            "=In this scene everyone gets cake.",
            [Element::Synopsis(lines(&[
                "In this scene everyone gets cake."
            ]))]
        );

        test_screenplay!(
            does_not_parse_section,
            r"
# Act 1

INT. HOUSE

## Montage

House is empty.",
            [
                Element::Heading {
                    slug: "INT. HOUSE".into(),
                    number: None,
                },
                Element::Action(lines(&["House is empty."]))
            ]
        );

        test_screenplay!(
            filters_out_boneyard,
            r"
INT. HOUSE

/* This is a boneyard
                and should not be parsed
, you understand?*/

House is empty.",
            [
                Element::Heading {
                    slug: "INT. HOUSE".into(),
                    number: None,
                },
                Element::Action(lines(&["House is empty."]))
            ]
        );

        test_screenplay!(
            filters_out_boneyard_inlined,
            "The house is /*extremely full*/empty.",
            [Element::Action(lines(&["The house is empty."]))]
        );

        test_screenplay!(
            filters_out_boneyard_unended,
            r"
INT. HOUSE

/* This is a boneyard
                and should not be parsed
, you understand?

House is empty.",
            [Element::Heading {
                slug: "INT. HOUSE".into(),
                number: None,
            }]
        );

        test_screenplay!(
            filters_out_note_multiline,
            r"
INT. HOUSE

[[ This is a note
                and should not be parsed
, you understand?]]

House is empty.",
            [
                Element::Heading {
                    slug: "INT. HOUSE".into(),
                    number: None,
                },
                Element::Action(lines(&["House is empty."]))
            ]
        );

        test_screenplay!(
            filters_out_note_inlined,
            "The house is [[should it be full?]]empty.",
            [Element::Action(lines(&["The house is empty."]))]
        );

        test_screenplay!(
            filters_out_note_inlined_multiline,
            r"
INT. HOUSE

The house [[ This is a note
                and should not be parsed
, you understand?]]is empty.",
            [
                Element::Heading {
                    slug: "INT. HOUSE".into(),
                    number: None,
                },
                Element::Action(lines(&["The house is empty."]))
            ]
        );

        test_screenplay!(
            filters_out_note_multiline_empty_newline,
            "INT. HOUSE\n\nThe house [[This is a note\n  \nand should not be parsed\n, you understand?]]is empty.",
            [
                Element::Heading {
                    slug: "INT. HOUSE".into(),
                    number: None,
                },
                Element::Action(lines(&["The house is empty."]))
            ]
        );

        test_screenplay!(
            not_filters_out_unended_note_multiline,
            r"
INT. HOUSE

The house [[wow

no",
            [
                Element::Heading {
                    slug: "INT. HOUSE".into(),
                    number: None,
                },
                Element::Action(lines(&["The house [[wow"])),
                Element::Action(lines(&["no"]))
            ]
        );

        test_screenplay!(
            not_filters_out_unended_note,
            "This is [[ not right",
            [Element::Action(lines(&["This is [[ not right"]))]
        );

        test_screenplay!(
            parenthetical_must_be_closed,
            "NAME\n(This is dialogue",
            [Element::Dialogue(Dialogue {
                character: "NAME".into(),
                extension: None,
                elements: vec![span(DialogueElement::Line("(This is dialogue".into()))]
            })]
        );

        test_screenplay!(
            dialogue_with_whiteline_in_middle,
            "NAME\nThis dialogue should visibly have a line below it.\n  \nVisually separating it from this line, due to the two spaces at the start of the previous line.",
            [Element::Dialogue(Dialogue {
                character: "NAME".into(),
                extension: None,
                elements: vec![
                    span(DialogueElement::Line("This dialogue should visibly have a line below it.".into())),
                    span(DialogueElement::Line("".into())),
                    span(DialogueElement::Line("Visually separating it from this line, due to the two spaces at the start of the previous line.".into())),
                ]
            })]
        );
    }

    /// Tests the byte ranges the parser attaches to what it parses.
    ///
    /// Every test checks a range by slicing the source with it and comparing
    /// the slice, so the expected values read as text instead of offsets.
    /// A range always refers to the original source, so boneyards and notes are
    /// included in the ranges they sit inside.
    mod spans {
        use super::*;

        /// The source slice covered by each top-level element, in order.
        fn element_sources(src: &str) -> Vec<&str> {
            parse(src).elements.iter().map(|e| &src[e.span]).collect()
        }

        /// The source slice covered by each parenthetical and line of dialogue,
        /// grouped per speaker. A dual dialogue gives two groups.
        fn dialogue_part_sources(src: &str) -> Vec<Vec<&str>> {
            let screenplay = parse(src);
            let speakers = screenplay.elements.iter().flat_map(|e| match &e.inner {
                Element::Dialogue(d) => vec![d],
                Element::DualDialogue(left, right) => vec![left, right],
                _ => vec![],
            });
            speakers.map(|d| sources(src, &d.elements)).collect()
        }

        /// The source slice covered by each of `values`.
        fn sources<'s, T>(src: &'s str, values: &[Span<T>]) -> Vec<&'s str> {
            values.iter().map(|v| &src[v.span]).collect()
        }

        #[test]
        fn action_covers_its_line() {
            assert_eq!(element_sources("Some action."), ["Some action."]);
        }

        #[test]
        fn consecutive_action_lines_form_one_element() {
            let src = "One.\nTwo.\nThree.";
            assert_eq!(element_sources(src), [src]);
        }

        #[test]
        fn each_line_of_a_block_has_its_own_range() {
            let src = "One.\nTwo.\nThree.";
            let screenplay = parse(src);
            let [
                Span {
                    inner: Element::Action(lines),
                    ..
                },
            ] = &screenplay.elements[..]
            else {
                panic!("expected a single action element");
            };
            assert_eq!(sources(src, lines.as_ref()), ["One.", "Two.", "Three."]);
        }

        #[test]
        fn lines_of_every_block_kind_are_kept_apart() {
            let cases: &[(&str, &[&str])] = &[
                ("~one\n~two", &["~one", "~two"]),
                ("> one <\n> two <", &["> one <", "> two <"]),
                ("!one\ntwo", &["!one", "two"]),
                ("=one\n=two", &["=one", "=two"]),
            ];
            for (src, expected) in cases {
                let screenplay = parse(src);
                let [Span { inner, .. }] = &screenplay.elements[..] else {
                    panic!("expected a single element for {src:?}");
                };
                let (Element::Action(lines)
                | Element::Lyrics(lines)
                | Element::CenteredText(lines)
                | Element::Synopsis(lines)) = inner
                else {
                    panic!("expected a block element for {src:?}");
                };
                assert_eq!(&sources(src, lines.as_ref()), expected, "input: {src:?}");
            }
        }

        #[test]
        fn dialogue_lines_are_kept_apart_and_a_blank_line_is_empty() {
            let src = "CHAR\nOne.\n  \nTwo.";
            assert_eq!(dialogue_part_sources(src), [["One.", "  ", "Two."]]);
        }

        #[test]
        fn blank_lines_belong_to_no_element() {
            let src = "INT. ROOM - DAY\n\nSome action.\n\n===\n\nMore action.";
            assert_eq!(
                element_sources(src),
                ["INT. ROOM - DAY", "Some action.", "===", "More action."]
            );
        }

        #[test]
        fn boneyard_is_recorded_with_its_offsets_and_is_not_an_element() {
            let src = "One.\n\n/* a\nmulti-line\nboneyard */\nTwo.";
            let screenplay = parse(src);
            assert_eq!(element_sources(src), ["One.", "Two."]);

            let [boneyard] = &screenplay.boneyards[..] else {
                panic!("expected exactly one boneyard");
            };
            assert_eq!(&src[boneyard.span], "/* a\nmulti-line\nboneyard */");
            assert_eq!(&src[boneyard.inner], " a\nmulti-line\nboneyard ");
            assert_eq!(boneyard.text, " a\nmulti-line\nboneyard ");
        }

        #[test]
        fn note_inside_a_line_is_in_the_element_range_but_not_its_content() {
            let src = "Hello [[a note]] there.";
            let screenplay = parse(src);
            assert_eq!(element_sources(src), [src]);
            let Element::Action(lines) = &screenplay.elements[0].inner else {
                panic!("expected action");
            };
            assert_eq!(lines.join().to_plain_string(), "Hello  there.");

            let [note] = &screenplay.notes[..] else {
                panic!("expected exactly one note");
            };
            assert_eq!(&src[note.span], "[[a note]]");
            assert_eq!(&src[note.inner], "a note");
            assert_eq!(note.text, "a note");
        }

        #[test]
        fn dialogue_runs_from_the_character_to_its_last_line() {
            let src = "\nCHAR\n(sad)\nNooo!\n(angry)\nI am angry.";
            assert_eq!(
                element_sources(src),
                ["CHAR\n(sad)\nNooo!\n(angry)\nI am angry."]
            );
        }

        #[test]
        fn dialogue_parentheticals_and_lines_have_their_own_ranges() {
            let src = "\nCHAR\n(sad)\nNooo!\n(angry)\nI am angry.";
            assert_eq!(
                dialogue_part_sources(src),
                [["(sad)", "Nooo!", "(angry)", "I am angry."]]
            );
        }

        #[test]
        fn consecutive_dialogue_lines_are_separate_parts() {
            let src = "\nCHAR\nFirst line.\nSecond line.";
            assert_eq!(
                dialogue_part_sources(src),
                [["First line.", "Second line."]]
            );
        }

        #[test]
        fn dual_dialogue_covers_both_speakers() {
            let src = "\nALICE\nHi there.\n\nBOB ^\nHello.";
            assert_eq!(element_sources(src), ["ALICE\nHi there.\n\nBOB ^\nHello."]);
            assert_eq!(
                dialogue_part_sources(src),
                [vec!["Hi there."], vec!["Hello."]]
            );
        }

        #[test]
        fn title_page_value_covers_its_whole_line_including_the_key() {
            let src = "Title: My Movie\nCredit: Written by\nAuthor: A. Writer\n\nAction.";
            let title_page = parse(src).titlepage.expect("expected a title page");
            assert_eq!(sources(src, &title_page.title), ["Title: My Movie"]);
            assert_eq!(sources(src, &title_page.credit), ["Credit: Written by"]);
            assert_eq!(sources(src, &title_page.authors), ["Author: A. Writer"]);
            assert_eq!(element_sources(src), ["Action."]);
        }

        #[test]
        fn indented_title_page_values_each_cover_one_line() {
            let src = "Title:\n   My Movie\n   The Sequel\n\nAction.";
            let title_page = parse(src).titlepage.expect("expected a title page");
            assert_eq!(
                sources(src, &title_page.title),
                ["   My Movie", "   The Sequel"]
            );
            assert_eq!(element_sources(src), ["Action."]);
        }

        #[test]
        fn every_kind_of_element_covers_its_source_lines() {
            let cases: &[(&str, &[&str])] = &[
                ("INT. ROOM - DAY", &["INT. ROOM - DAY"]),
                (".FORCED HEADING", &[".FORCED HEADING"]),
                ("Plain action.", &["Plain action."]),
                ("!INT. Forced action.", &["!INT. Forced action."]),
                ("\nCUT TO:\n\nAfter.", &["CUT TO:", "After."]),
                (
                    "\n> Forced transition\n\nAfter.",
                    &["> Forced transition", "After."],
                ),
                ("> THE END <", &["> THE END <"]),
                ("> one <\n> two <", &["> one <\n> two <"]),
                ("~A sung line", &["~A sung line"]),
                ("~verse one\n~verse two", &["~verse one\n~verse two"]),
                ("= A synopsis", &["= A synopsis"]),
                ("===", &["==="]),
                ("# A section produces no element", &[]),
            ];
            for (src, expected) in cases {
                assert_eq!(&element_sources(src), expected, "input: {src:?}");
            }
        }
    }
}
