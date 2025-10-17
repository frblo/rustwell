use crate::rich_string::RichString;
use crate::screenplay::Dialogue;
use crate::screenplay::DialogueElement;
use crate::screenplay::Element;
use crate::screenplay::Screenplay;
use crate::screenplay::TitlePage;
use std::iter::Peekable;
use std::str::Lines;

pub fn parse(src: &str) -> Screenplay {
    let cleaned = preprocess_source(src);
    Parser::new(&cleaned).parse()
}

fn preprocess_source(src: &str) -> String {
    let bytes = src.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    let mut in_comment = false;

    while i < bytes.len() {
        let b = bytes[i];

        if !in_comment {
            // Check if at the start of a comment
            if i + 1 < bytes.len() && b == b'/' && bytes[i + 1] == b'*' {
                in_comment = true;
                i += 2;
                continue;
            }

            // As Fountain specifes a tab = 4 spaces
            if b == b'\t' {
                out.extend_from_slice(b"    ");
                i += 1;
                continue;
            }

            // Normal character
            out.push(b);
            i += 1;
        } else {
            // Inside comment: preserve only newlines so that a comment can cover all whitespace
            if b == b'\n' {
                out.push(b'\n');
                i += 1;
                continue;
            }

            // Check if at the end of comment
            if i + 1 < bytes.len() && b == b'*' && bytes[i + 1] == b'/' {
                in_comment = false;
                i += 2;
                continue;
            }

            i += 1;
        }
    }

    String::from_utf8(out).expect("Valid UTF-8 after preprocessing")
}

struct Parser<'a> {
    lines: Peekable<Lines<'a>>,
    state: State,
    elements: Vec<Element>,
    title_page: Option<TitlePage>,
    dialogue_accumulator: Dialogue,
    pending_dual_dialogue: Option<Dialogue>,
}

impl<'a> Parser<'a> {
    fn new(src: &'a str) -> Self {
        Self {
            lines: src.lines().peekable(),
            state: State::Default,
            elements: Vec::new(),
            title_page: None,
            dialogue_accumulator: Dialogue::new(),
            pending_dual_dialogue: None,
        }
    }

    fn parse(mut self) -> Screenplay {
        self.parse_title();
        while let Some(line) = self.lines.next() {
            match self.state {
                State::Default => self.parse_default(line),
                State::InDialogue => self.parse_in_dialogue(line),
                State::AfterNonBlank => self.parse_after_non_blank(line),
            }
        }

        Screenplay {
            titlepage: self.title_page,
            elements: self.elements,
        }
    }

    fn parse_default(&mut self, line: &str) {
        if line.starts_with("===") {
            self.elements.push(Element::PageBreak);
            return;
        }

        if let Some(forced_action) = line.strip_prefix('!') {
            self.elements
                .push(Element::Action(RichString::from(&forced_action)));
            return;
        }

        let mut trimmed = line.trim();

        if self.line_is_heading(trimmed) {
            self.elements.push(Element::Heading {
                slug: RichString::from(trimmed),
                number: None,
            });

            self.lines.next(); // Consume the empty blank line
            return;
        }

        if self.line_is_centered(trimmed) {
            let inner = trimmed
                .strip_prefix('>')
                .expect("Line was centered so should start with >")
                .strip_suffix('<')
                .expect("Line was centered so should start with <");
            self.elements
                .push(Element::CenteredText(RichString::from(inner)));

            self.lines.next(); // Consume the empty blank line
            return;
        }

        if self.line_is_transition(trimmed) {
            self.elements
                .push(Element::Transition(RichString::from(trimmed)));

            self.lines.next(); // Consume the empty blank line
            return;
        }

        if self.line_is_dialogue_start(trimmed) {
            if trimmed.ends_with('^') {
                if let Some(&Element::Dialogue(_)) = self.elements.last() {
                    if let Some(Element::Dialogue(d)) = self.elements.pop() {
                        self.pending_dual_dialogue = Some(d);
                        trimmed = trimmed.strip_suffix('^').unwrap();
                    }
                }
            }
            self.dialogue_accumulator.character = RichString::from(trimmed);
            self.state = State::InDialogue;
            return;
        }

        if trimmed.is_empty() {
            return;
        }

        self.parse_stateless(line);
    }

    fn parse_in_dialogue(&mut self, line: &str) {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            // Line that starts with two or more spaces doesn't break dialogue.
            if !line.starts_with("  ") {
                self.state = State::Default;
                let finished_dialogue =
                    std::mem::replace(&mut self.dialogue_accumulator, Dialogue::new());
                if let Some(d) = self.pending_dual_dialogue.take() {
                    self.elements
                        .push(Element::DualDialogue(d, finished_dialogue));
                } else {
                    self.elements.push(Element::Dialogue(finished_dialogue));
                }
            }
            return;
        }

        if trimmed.starts_with('(') {
            self.dialogue_accumulator
                .elements
                .push(DialogueElement::Parenthetical(RichString::from(trimmed)));
            return;
        }

        self.dialogue_accumulator
            .elements
            .push(DialogueElement::Line(RichString::from(trimmed)));
    }

    fn parse_after_non_blank(&mut self, line: &str) {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            self.state = State::Default;
            return;
        }

        self.parse_stateless(line);
    }

    fn parse_stateless(&mut self, line: &str) {
        self.state = State::AfterNonBlank;

        if line.starts_with('~') {
            self.elements.push(Element::Lyrics(RichString::from(line)));
            return;
        }

        self.elements.push(Element::Action(RichString::from(line)));
    }

    fn parse_title(&mut self) {
        let mut tp = TitlePage::new();

        while let Some(line) = self.lines.peek() {
            let Some((key, val)) = line.split_once(':') else {
                break;
            };
            self.lines.next(); // Consume the key line

            let mut values = Vec::new();

            if !val.trim().is_empty() {
                values.push(RichString::from(val));
            } else {
                values = self.take_indented_block();
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

        // If the title page is followed by a blank line (as it should)
        // then skip it to start rest whit whatever comes next.
        // Technically allows for no blank line before rest of document.
        if let Some(line) = self.lines.peek() {
            if line.is_empty() {
                self.lines.next();
            }
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
        while let Some(next) = self.lines.peek().copied() {
            if next.starts_with("   ") {
                self.lines.next();
                out.push(RichString::from(next.trim()));
            } else {
                break;
            }
        }
        out
    }

    fn line_is_heading(&mut self, line: &str) -> bool {
        let upper_line = line.to_uppercase();

        let mut it = line.chars();
        let forced = matches!(
            (it.next(), it.next()),
            (Some('.'), Some(c)) if c.is_alphanumeric()
        );

        // INT./EXT, INT/EXT covered by INT
        let prefix = ["INT", "EXT", "EST", "I/E"]
            .iter()
            .any(|p| upper_line.starts_with(p));

        (forced || prefix) && self.next_line_is_empty()
    }

    fn line_is_dialogue_start(&mut self, line: &str) -> bool {
        let forced = line.starts_with('@');

        let head = line.split_once('(').map_or(line, |(h, _)| h);
        let has_alpha = head.chars().any(|c| c.is_alphabetic());
        let has_lower = head.chars().any(|c| c.is_lowercase());
        let character_elem = has_alpha && !has_lower;

        (forced || character_elem) && !self.next_line_is_empty()
    }

    fn line_is_transition(&mut self, line: &str) -> bool {
        let forced = line.starts_with('>') && !line.ends_with('<');

        let transition_ending = line.ends_with("TO:");
        let has_lower = line.chars().any(|c| c.is_lowercase());
        let transition_elem = transition_ending && !has_lower;

        (forced || transition_elem) && self.next_line_is_empty()
    }

    fn line_is_centered(&mut self, line: &str) -> bool {
        line.starts_with('>') && line.ends_with('<') && self.next_line_is_empty()
    }

    fn next_line_is_empty(&mut self) -> bool {
        self.lines.peek().map_or(true, |s| s.is_empty())
    }
}

#[derive(Debug, PartialEq, Eq)]
enum State {
    Default,
    InDialogue,
    AfterNonBlank,
}
