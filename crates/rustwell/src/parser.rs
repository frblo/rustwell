mod scanner;

use std::borrow::Cow;
use std::iter::Peekable;

use crate::rich_string::RichString;
use crate::screenplay::Dialogue;
use crate::screenplay::DialogueElement;
use crate::screenplay::Element;
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

/// Expands tabs to four spaces, borrowing when there are none.
fn expand_tabs(text: Cow<'_, str>) -> Cow<'_, str> {
    if text.contains('\t') {
        Cow::Owned(text.replace('\t', "    "))
    } else {
        text
    }
}

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
    /// Might seem like trimming is used a lot. The intention is that the
    /// try functions work without having trimmed. Cost is extremely low when
    /// calling trim on a already trimmed [&str].
    fn parse(mut self) -> (Option<TitlePage>, Vec<Span<Element>>) {
        self.parse_title();
        while let Some(logical) = self.lines.next() {
            let range = logical.range;
            let line = self.text(logical);
            let line = &*line;
            let trimmed = line.trim();

            if trimmed.is_empty() && !line.starts_with("  ") {
                self.state = State::Default;
                continue;
            }

            match self.state {
                State::Default => {
                    // The first one returning true will break
                    if self.try_section(trimmed)
                        || self.try_page_break(trimmed, range)
                        || self.try_synopsis(trimmed, range)
                        || self.try_forced_action(trimmed, range)
                        || self.try_centered(trimmed, range)
                        || self.try_lyrics(trimmed, range)
                        || self.try_heading(trimmed, range)
                        || self.try_transition(trimmed, range)
                        || self.try_dialogue_start(trimmed, range)
                        || self.try_action(line, range)
                    {}
                }
                State::InDialogue => {
                    let (curr_dialogue, element_span) = self
                        .get_last_dialogue()
                        .expect("Must exist since we are in dialogue block");
                    element_span.end = range.end;

                    if trimmed.starts_with('(') && trimmed.ends_with(')') {
                        curr_dialogue.elements.push(Span::new(
                            DialogueElement::Parenthetical(RichString::from(trimmed)),
                            range,
                        ));
                        continue;
                    }

                    if let Some(Span {
                        inner: DialogueElement::Line(rs),
                        span,
                    }) = curr_dialogue.elements.last_mut()
                    {
                        rs.push_str("\n");
                        rs.push_str(trimmed);
                        span.end = range.end;
                    } else {
                        curr_dialogue.elements.push(Span::new(
                            DialogueElement::Line(RichString::from(trimmed)),
                            range,
                        ));
                    }
                }
                State::InBlock => {
                    if self.try_centered(trimmed, range)
                        || self.try_lyrics(trimmed, range)
                        || self.try_action(line, range)
                    {}
                }
            }
        }

        (self.title_page, self.elements)
    }

    /// The text of `line` as the parser sees it: boneyards and notes removed and
    /// tabs expanded.
    fn text(&self, line: &'a LogicalLine) -> Cow<'a, str> {
        expand_tabs(line.text(self.src))
    }

    /// `try_` is a helper function taking a predicate and a handle function
    /// and is used to define different parts of the state machine.
    fn try_<'s, P, H>(&mut self, line: &'s str, predicate: P, handle: H) -> bool
    where
        P: FnOnce(&mut Self, &'s str) -> Option<&'s str>,
        H: FnOnce(&mut Self, &'s str),
    {
        let Some(new_line) = predicate(self, line) else {
            return false;
        };

        handle(self, new_line);
        true
    }

    fn try_section(&mut self, line: &str) -> bool {
        self.try_(
            line,
            |_, s| s.trim_start().starts_with('#').then_some(s),
            |_, _| {},
        )
    }

    fn try_synopsis(&mut self, line: &str, range: SourceRange) -> bool {
        self.try_(
            line,
            |_, s| s.trim_start().strip_prefix('='),
            |this, inner| {
                if this.state == State::InBlock
                    && let Some(Span {
                        span,
                        inner: Element::Synopsis(rs),
                    }) = this.elements.last_mut()
                {
                    rs.push_str("\n");
                    rs.push_str(inner);
                    span.end = range.end;
                    return;
                }

                let rs = RichString::from(inner);
                this.elements.push(Span::new(Element::Synopsis(rs), range));

                this.state = State::InBlock;
            },
        )
    }

    fn try_page_break(&mut self, line: &str, range: SourceRange) -> bool {
        self.try_(
            line,
            |_, s| s.trim_start().starts_with("===").then_some(s),
            |this, _| this.elements.push(Span::new(Element::PageBreak, range)),
        )
    }

    fn try_forced_action(&mut self, line: &str, range: SourceRange) -> bool {
        self.try_(
            line,
            |_, s| s.trim_start().strip_prefix('!'),
            |this, inner| {
                this.elements
                    .push(Span::new(Element::Action(RichString::from(inner)), range));
                this.state = State::InBlock;
            },
        )
    }

    fn try_centered(&mut self, line: &str, range: SourceRange) -> bool {
        self.try_(
            line,
            |_, s| s.trim().strip_prefix('>').and_then(|u| u.strip_suffix('<')),
            |this, inner| {
                let inner = inner.trim();
                if this.state == State::InBlock
                    && let Some(Span {
                        span,
                        inner: Element::CenteredText(rs),
                    }) = this.elements.last_mut()
                {
                    rs.push_str("\n");
                    rs.push_str(inner);
                    span.end = range.end;
                    return;
                }

                let rs = RichString::from(inner);
                this.elements
                    .push(Span::new(Element::CenteredText(rs), range));

                this.state = State::InBlock;
            },
        )
    }

    fn try_lyrics(&mut self, line: &str, range: SourceRange) -> bool {
        self.try_(
            line,
            |_, s| s.trim_start().strip_prefix('~'),
            |this, inner| {
                if this.state == State::InBlock
                    && let Some(Span {
                        span,
                        inner: Element::Lyrics(rs),
                    }) = this.elements.last_mut()
                {
                    rs.push_str("\n");
                    rs.push_str(inner);
                    span.end = range.end;
                    return;
                }

                let rs = RichString::from(inner);
                this.elements.push(Span::new(Element::Lyrics(rs), range));

                this.state = State::InBlock;
            },
        )
    }

    fn try_action(&mut self, line: &str, range: SourceRange) -> bool {
        self.try_(
            line,
            |_, line| Some(line),
            |this, inner| {
                if this.state == State::InBlock
                    && let Some(Span {
                        span,
                        inner: Element::Action(rs),
                    }) = this.elements.last_mut()
                {
                    rs.push_str("\n");
                    rs.push_str(inner);
                    span.end = range.end;
                    return;
                }

                let rs = RichString::from(inner);
                this.elements.push(Span::new(Element::Action(rs), range));

                this.state = State::InBlock;
            },
        )
    }

    fn try_heading(&mut self, line: &str, range: SourceRange) -> bool {
        self.try_(
            line,
            |this, line| {
                let trimmed = line.trim_start();
                let mut it = trimmed.chars();
                if matches!(
                    (it.next(), it.next()),
                    (Some('.'), Some(c)) if c.is_alphanumeric()
                ) {
                    return Some(
                        trimmed
                            .strip_prefix('.')
                            .expect("Already checked that it exists"),
                    );
                }

                let pats = ["INT", "EXT", "EST", "I/E", "INT./EXT", "INT/EXT"];
                let bytes = trimmed.as_bytes();

                (pats.iter().any(|p| {
                    let n = p.len();
                    bytes
                        .get(..n)
                        .is_some_and(|head| head.eq_ignore_ascii_case(p.as_bytes()))
                        && bytes.get(n).is_some_and(|&end| end == b' ' || end == b'.')
                }) && this.next_line_is_empty())
                .then_some(trimmed)
            },
            |this, inner| {
                let mut number = None;
                let mut inner = inner;
                if let Some(start) = inner.trim_end().strip_suffix('#')
                    && let Some((new_inner, numbering)) = start.rsplit_once('#')
                    && numbering
                        .chars()
                        .all(|c| c.is_alphanumeric() || c == '-' || c == '.')
                {
                    number = Some(numbering.to_string());
                    inner = new_inner.trim_end();
                }

                this.elements.push(Span::new(
                    Element::Heading {
                        slug: RichString::from(inner),
                        number,
                    },
                    range,
                ));

                this.lines.next();
            },
        )
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
            return stripped;
        }

        self.elements
            .push(Span::new(Element::Dialogue(new_dialogue), range));
        inner
    }

    fn try_dialogue_start(&mut self, line: &str, range: SourceRange) -> bool {
        self.try_(
            line,
            |this, line| {
                let trimmed = line.trim_start();
                if let Some(inner) = trimmed.strip_prefix('@') {
                    return Some(inner);
                }

                let head = trimmed.split_once('(').map_or(trimmed, |(h, _)| h);
                let has_alpha = head.chars().any(char::is_alphabetic);
                let has_lower = head.chars().any(char::is_lowercase);
                (has_alpha && !has_lower && !this.next_line_is_empty()).then_some(trimmed)
            },
            |this, inner| {
                let mut inner = this.insert_empty_dialogue(inner, range);

                let (curr_dialogue, _) = this
                    .get_last_dialogue()
                    .expect("Just pushed to list, must exist");

                if let Some((head, tail)) = inner.split_once('(')
                    && let Some((extension, _)) = tail.split_once(')')
                {
                    curr_dialogue.extension = Some(RichString::from(extension));
                    inner = head.trim_end();
                }

                curr_dialogue.character = RichString::from(inner);

                this.state = State::InDialogue;
            },
        )
    }

    fn try_transition(&mut self, line: &str, range: SourceRange) -> bool {
        self.try_(
            line,
            |this, line| {
                if let Some(inner) = line.trim_start().strip_prefix('>')
                    && !line.trim_end().ends_with('<')
                {
                    return Some(inner);
                }

                let transition_ending = line.ends_with("TO:");
                let has_lower = line.chars().any(char::is_lowercase);
                let transition_elem = transition_ending && !has_lower;

                (transition_elem && this.next_line_is_empty()).then_some(line)
            },
            |this, inner| {
                this.elements.push(Span::new(
                    Element::Transition(RichString::from(inner)),
                    range,
                ));

                this.lines.next();
            },
        )
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

        fn span<T>(inner: T) -> Span<T> {
            Span::new(inner, SourceRange::from(0..0))
        }

        /// Zeroes the spans of the dialogue elements, so that an [`Element`] can be
        /// compared by shape alone.
        fn without_spans(element: &Element) -> Element {
            let zero = |d: &Dialogue| {
                let mut d = d.clone();
                for e in &mut d.elements {
                    e.span = SourceRange::from(0..0);
                }
                d
            };
            match element {
                Element::Dialogue(d) => Element::Dialogue(zero(d)),
                Element::DualDialogue(l, r) => Element::DualDialogue(zero(l), zero(r)),
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
            [Element::Action("Intro music plays.".into())]
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
            [Element::Action(
                "They look at the test output - it's all failing.".into()
            )]
        );

        test_screenplay!(
            parses_action_forced,
            "!INT. They look at the test output - it's all failing.",
            [Element::Action(
                "INT. They look at the test output - it's all failing.".into(),
            )]
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
            [Element::Lyrics("Hey ho let's go".into())]
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
            [Element::CenteredText("The end".into())]
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
            [Element::Synopsis(
                "In this scene everyone gets cake.".into(),
            )]
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
                Element::Action("House is empty.".into())
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
                Element::Action("House is empty.".into())
            ]
        );

        test_screenplay!(
            filters_out_boneyard_inlined,
            "The house is /*extremely full*/empty.",
            [Element::Action("The house is empty.".into())]
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
                Element::Action("House is empty.".into())
            ]
        );

        test_screenplay!(
            filters_out_note_inlined,
            "The house is [[should it be full?]]empty.",
            [Element::Action("The house is empty.".into())]
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
                Element::Action("The house is empty.".into())
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
                Element::Action("The house is empty.".into())
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
                Element::Action("The house [[wow".into()),
                Element::Action("no".into())
            ]
        );

        test_screenplay!(
            not_filters_out_unended_note,
            "This is [[ not right",
            [Element::Action("This is [[ not right".into())]
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
                elements: vec![span(DialogueElement::Line("This dialogue should visibly have a line below it.\n\nVisually separating it from this line, due to the two spaces at the start of the previous line.".into()))]
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

            let [boneyard] = screenplay.boneyards[..] else {
                panic!("expected exactly one boneyard");
            };
            assert_eq!(&src[boneyard.span], "/* a\nmulti-line\nboneyard */");
            assert_eq!(&src[boneyard.inner], " a\nmulti-line\nboneyard ");
        }

        #[test]
        fn note_inside_a_line_is_in_the_element_range_but_not_its_content() {
            let src = "Hello [[a note]] there.";
            let screenplay = parse(src);
            assert_eq!(element_sources(src), [src]);
            assert_eq!(
                screenplay.elements[0].inner,
                Element::Action("Hello  there.".into())
            );

            let [note] = screenplay.notes[..] else {
                panic!("expected exactly one note");
            };
            assert_eq!(&src[note.span], "[[a note]]");
            assert_eq!(&src[note.inner], "a note");
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
        fn consecutive_dialogue_lines_form_one_part() {
            let src = "\nCHAR\nFirst line.\nSecond line.";
            assert_eq!(dialogue_part_sources(src), [["First line.\nSecond line."]]);
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
