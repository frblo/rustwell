use crate::rich_string::RichString;
use crate::screenplay::Dialogue;
use crate::screenplay::DialogueElement;
use crate::screenplay::Element;
use crate::screenplay::Screenplay;
use crate::screenplay::Span;
use crate::screenplay::TitlePage;
use std::iter::Enumerate;
use std::iter::Peekable;
use std::str::Lines;

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
    let cleaned = preprocess_source(src);
    Parser::new(&cleaned).parse()
}

/// Internal parser state machine for Fountain.
///
/// Keeps an iterator of the source, a accumulative list of [`Element`]s, and
/// a state. Also tracks a [`TitlePage`] if such exists in the source.
struct Parser<'a> {
    lines: Peekable<Enumerate<Lines<'a>>>,
    state: State,
    elements: Vec<Span<Element>>,
    title_page: Option<TitlePage>,
}

impl<'a> Parser<'a> {
    /// Create new parser
    ///
    /// Expects `src` to have been preprocessed.
    fn new(src: &'a str) -> Self {
        Self {
            lines: src.lines().enumerate().peekable(),
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
    fn parse(mut self) -> Screenplay {
        self.parse_title();
        while let Some((i, line)) = self.lines.next() {
            let trimmed = line.trim();

            if trimmed.is_empty() && !line.starts_with("  ") {
                self.state = State::Default;
                continue;
            }

            match self.state {
                State::Default => {
                    // The first one returning true will break
                    if self.try_section(trimmed, i)
                        || self.try_page_break(trimmed, i)
                        || self.try_synopsis(trimmed, i)
                        || self.try_forced_action(trimmed, i)
                        || self.try_centered(trimmed, i)
                        || self.try_lyrics(trimmed, i)
                        || self.try_heading(trimmed, i)
                        || self.try_transition(trimmed, i)
                        || self.try_dialogue_start(trimmed, i)
                        || self.try_action(line, i)
                    {}
                }
                State::InDialogue => {
                    let (curr_dialogue, end_line) = self
                        .get_last_dialogue()
                        .expect("Must exist since we are in dialogue block");
                    *end_line = i;

                    if trimmed.starts_with('(') {
                        curr_dialogue
                            .elements
                            .push(DialogueElement::Parenthetical(RichString::from(trimmed)));
                        continue;
                    }

                    curr_dialogue
                        .elements
                        .push(DialogueElement::Line(RichString::from(trimmed)));
                }
                State::InBlock => {
                    if self.try_centered(trimmed, i)
                        || self.try_lyrics(trimmed, i)
                        || self.try_action(line, i)
                    {}
                }
            }
        }

        Screenplay::new(self.title_page, self.elements)
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

    fn try_section(&mut self, line: &str, line_idx: usize) -> bool {
        self.try_(
            line,
            |_, s| s.trim_start().starts_with('#').then_some(s),
            |_, _| {},
        )
    }

    fn try_synopsis(&mut self, line: &str, line_idx: usize) -> bool {
        self.try_(
            line,
            |_, s| s.trim_start().strip_prefix('='),
            |this, inner| {
                if this.state == State::InBlock
                    && let Some(Span {
                        start_line: _,
                        end_line,
                        inner: Element::Synopsis(rs),
                    }) = this.elements.last_mut()
                {
                    rs.push_str("\n");
                    rs.push_str(inner);
                    *end_line = line_idx;
                    return;
                }

                let rs = RichString::from(inner);
                this.elements
                    .push(Span::new(Element::Synopsis(rs), line_idx));

                this.state = State::InBlock;
            },
        )
    }

    fn try_page_break(&mut self, line: &str, line_idx: usize) -> bool {
        self.try_(
            line,
            |_, s| s.trim_start().starts_with("===").then_some(s),
            |this, _| this.elements.push(Span::new(Element::PageBreak, line_idx)),
        )
    }

    fn try_forced_action(&mut self, line: &str, line_idx: usize) -> bool {
        self.try_(
            line,
            |_, s| s.trim_start().strip_prefix('!'),
            |this, inner| {
                this.elements.push(Span::new(
                    Element::Action(RichString::from(inner)),
                    line_idx,
                ));
                this.state = State::InBlock;
            },
        )
    }

    fn try_centered(&mut self, line: &str, line_idx: usize) -> bool {
        self.try_(
            line,
            |_, s| s.trim().strip_prefix('>').and_then(|u| u.strip_suffix('<')),
            |this, inner| {
                let inner = inner.trim();
                if this.state == State::InBlock
                    && let Some(Span {
                        start_line: _,
                        end_line,
                        inner: Element::CenteredText(rs),
                    }) = this.elements.last_mut()
                {
                    rs.push_str("\n");
                    rs.push_str(inner);
                    *end_line = line_idx;
                    return;
                }

                let rs = RichString::from(inner);
                this.elements
                    .push(Span::new(Element::CenteredText(rs), line_idx));

                this.state = State::InBlock;
            },
        )
    }

    fn try_lyrics(&mut self, line: &str, line_idx: usize) -> bool {
        self.try_(
            line,
            |_, s| s.trim_start().strip_prefix('~'),
            |this, inner| {
                if this.state == State::InBlock
                    && let Some(Span {
                        start_line: _,
                        end_line,
                        inner: Element::Lyrics(rs),
                    }) = this.elements.last_mut()
                {
                    rs.push_str("\n");
                    rs.push_str(inner);
                    *end_line = line_idx;
                    return;
                }

                let rs = RichString::from(inner);
                this.elements.push(Span::new(Element::Lyrics(rs), line_idx));

                this.state = State::InBlock;
            },
        )
    }

    fn try_action(&mut self, line: &str, line_idx: usize) -> bool {
        self.try_(
            line,
            |_, line| Some(line),
            |this, inner| {
                if this.state == State::InBlock
                    && let Some(Span {
                        start_line: _,
                        end_line,
                        inner: Element::Action(rs),
                    }) = this.elements.last_mut()
                {
                    rs.push_str("\n");
                    rs.push_str(inner);
                    *end_line = line_idx;
                    return;
                }

                let rs = RichString::from(inner);
                this.elements.push(Span::new(Element::Action(rs), line_idx));

                this.state = State::InBlock;
            },
        )
    }

    fn try_heading(&mut self, line: &str, line_idx: usize) -> bool {
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
                    line_idx,
                ));

                this.lines.next();
            },
        )
    }

    fn get_last_dialogue(&mut self) -> Option<(&mut Dialogue, &mut usize)> {
        let Some(Span {
            start_line: _,
            end_line,
            inner: Element::Dialogue(curr_dialogue) | Element::DualDialogue(_, curr_dialogue),
        }) = self.elements.last_mut()
        else {
            return None;
        };

        Some((curr_dialogue, end_line))
    }

    fn insert_empty_dialogue<'s>(&mut self, inner: &'s str, line_idx: usize) -> &'s str {
        let new_dialogue = Dialogue::new();

        if let Some(stripped) = inner.trim_end().strip_suffix('^')
            && let Some(&Span {
                start_line: _,
                end_line: _,
                inner: Element::Dialogue(_),
            }) = self.elements.last()
            && let Some(Span {
                start_line,
                end_line: _,
                inner: Element::Dialogue(d),
            }) = self.elements.pop()
        {
            self.elements.push(Span::new(
                Element::DualDialogue(d, new_dialogue),
                start_line,
            ));
            return stripped;
        }

        self.elements
            .push(Span::new(Element::Dialogue(new_dialogue), line_idx));
        inner
    }

    fn try_dialogue_start(&mut self, line: &str, line_idx: usize) -> bool {
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
                let mut inner = this.insert_empty_dialogue(inner, line_idx);

                let (curr_dialogue, end_line) = this
                    .get_last_dialogue()
                    .expect("Just pushed to list, must exist");

                if let Some((head, tail)) = inner.split_once('(')
                    && let Some((extension, _)) = tail.split_once(')')
                {
                    curr_dialogue.extension = Some(RichString::from(extension));
                    inner = head.trim_end();
                }

                curr_dialogue.character = RichString::from(inner);
                *end_line = line_idx;

                this.state = State::InDialogue;
            },
        )
    }

    fn try_transition(&mut self, line: &str, line_idx: usize) -> bool {
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
                    line_idx,
                ));

                this.lines.next();
            },
        )
    }

    fn parse_title(&mut self) {
        let mut tp = TitlePage::new();

        while let Some((_, line)) = self.lines.peek() {
            let Some((key, val)) = line.split_once(':') else {
                break;
            };
            self.lines.next(); // Consume the key line

            let mut values = Vec::new();

            if val.trim().is_empty() {
                values = self.take_indented_block();
            } else {
                values.push(RichString::from(val));
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

    fn take_indented_block(&mut self) -> Vec<RichString> {
        let mut out = Vec::new();
        while let Some((_, next)) = self.lines.peek().copied() {
            if next.starts_with("   ") {
                self.lines.next();
                out.push(RichString::from(next.trim()));
            } else {
                break;
            }
        }
        out
    }

    fn next_line_is_empty(&mut self) -> bool {
        self.lines.peek().is_none_or(|(_, s)| s.trim().is_empty())
    }
}

/// Removes boneyards, notes and normalizes tabs to four spaces
fn preprocess_source(src: &str) -> String {
    let without_boneyards = remove_boneyards(src);
    remove_notes(&without_boneyards)
}

fn remove_boneyards(src: &str) -> String {
    let mut out = String::with_capacity(src.len());
    let mut rest = src;

    while let Some(start) = rest.find("/*") {
        out.push_str(&rest[..start].replace('\t', "    "));

        let boneyard_content = &rest[start + 2..];
        let (boneyard, after) = match boneyard_content.find("*/") {
            Some(end) => (&boneyard_content[..end], &boneyard_content[end + 2..]),
            None => (boneyard_content, ""),
        };
        if let Some(_) = boneyard.find('\n') {
            out.push('\n');
        }

        rest = after;
    }

    out.push_str(&rest.replace('\t', "    "));
    out
}

fn remove_notes(src: &str) -> String {
    let mut out = String::with_capacity(src.len());
    let mut rest = src;

    while let Some(start) = rest.find("[[") {
        out.push_str(&rest[..start]);

        let note_content = &rest[start + 2..];
        match note_content.find("]]") {
            Some(close) => {
                let inner = &note_content[..close];
                match find_note_break(inner) {
                    Some((break_pos, pat)) => {
                        // Treat as unclosed: dump [[ and content up to and including break
                        out.push_str("[[");
                        out.push_str(&inner[..break_pos]);
                        out.push_str(pat);
                        rest = &note_content[break_pos + pat.len()..];
                    }
                    None => {
                        rest = &note_content[close + 2..];
                    }
                }
            }
            None => {
                out.push_str("[[");
                rest = note_content;
            }
        }
    }

    out.push_str(rest);
    out
}

fn find_note_break(s: &str) -> Option<(usize, &str)> {
    let a = s.find("\n\n").map(|i| (i, "\n\n"));
    let b = s.find("\n \n").map(|i| (i, "\n \n"));
    match (a, b) {
        (Some(a), Some(b)) => Some(if a.0 <= b.0 { a } else { b }),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
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

    macro_rules! test_screenplay {
        ($name:ident, $input:expr, [$($elem:expr),*]) => {
            #[test]
            fn $name() {
            test_parse($input, [$($elem),*]);
            }
        };
    }

    fn test_parse<'a>(input: &str, expected: impl IntoIterator<Item = Element>) {
        let parsed = parse(input);
        for (
            Span {
                start_line: _,
                end_line: _,
                inner: actual,
            },
            expected,
        ) in parsed.elements.iter().zip(expected)
        {
            assert_eq!(actual, &expected);
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
        does_not_parse_heading_whitout_dot,
        "Intro music plays.",
        [Element::Action("Intro music plays.".into())]
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
                DialogueElement::Parenthetical("(sad)".into()),
                DialogueElement::Line("Nooo!".into()),
                DialogueElement::Parenthetical("(angry)".into()),
                DialogueElement::Line("I am angry.".into()),
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
                DialogueElement::Parenthetical("(sad)".into()),
                DialogueElement::Line("Nooo!".into()),
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
                DialogueElement::Parenthetical("(sad)".into()),
                DialogueElement::Line("Nooo!".into()),
                DialogueElement::Parenthetical("(angry)".into()),
                DialogueElement::Line("I am angry.".into()),
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
                DialogueElement::Parenthetical("(sad)".into()),
                DialogueElement::Line("Nooo!".into()),
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
                    DialogueElement::Parenthetical("(sad)".into()),
                    DialogueElement::Line("Nooo!".into()),
                ],
            },
            Dialogue {
                character: "CHOR".into(),
                extension: Some("V.O".into()),
                elements: vec![DialogueElement::Line("YES!".into())],
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
}
