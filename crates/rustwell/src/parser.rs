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

struct Parser<'a> {
    lines: Peekable<Lines<'a>>,
    state: State,
    elements: Vec<Element>,
    title_page: Option<TitlePage>,
}

impl<'a> Parser<'a> {
    fn new(src: &'a str) -> Self {
        Self {
            lines: src.lines().peekable(),
            state: State::Default,
            elements: Vec::new(),
            title_page: None,
        }
    }

    fn parse(mut self) -> Screenplay {
        self.parse_title();
        while let Some(line) = self.lines.next() {
            let trimmed = line.trim();

            if trimmed.is_empty() && !line.starts_with("  ") {
                self.state = State::Default;
                continue;
            }

            match self.state {
                State::Default => {
                    let trimmed = line.trim();

                    if self.try_page_break(trimmed)
                        || self.try_forced_action(trimmed)
                        || self.try_centered(trimmed)
                        || self.try_lyrics(trimmed)
                        || self.try_heading(trimmed)
                        || self.try_transition(trimmed)
                        || self.try_dialouge_start(trimmed)
                        || self.try_action(line)
                    {
                        continue;
                    }
                }
                State::InDialogue => {
                    let curr_dialouge = match self.elements.last_mut() {
                        Some(Element::Dialogue(d)) => d,
                        Some(Element::DualDialogue(_, d)) => d,
                        _ => unreachable!("If we are here previous must be one of these two"),
                    };

                    if trimmed.starts_with('(') {
                        curr_dialouge
                            .elements
                            .push(DialogueElement::Parenthetical(RichString::from(trimmed)));
                        continue;
                    }

                    curr_dialouge
                        .elements
                        .push(DialogueElement::Line(RichString::from(trimmed)));
                }
                State::InBlock => {
                    if self.try_centered(trimmed)
                        || self.try_lyrics(trimmed)
                        || self.try_action(line)
                    {
                        continue;
                    }
                }
            }
        }

        Screenplay {
            titlepage: self.title_page,
            elements: self.elements,
        }
    }

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

    fn try_page_break(&mut self, line: &str) -> bool {
        self.try_(
            line,
            |_, s| s.trim_start().starts_with("===").then_some(s),
            |this, _| this.elements.push(Element::PageBreak),
        )
    }

    fn try_forced_action(&mut self, line: &str) -> bool {
        self.try_(
            line,
            |_, s| s.trim_start().strip_prefix('!'),
            |this, inner| {
                this.elements.push(Element::Action(RichString::from(inner)));
                this.state = State::InBlock;
            },
        )
    }

    fn try_centered(&mut self, line: &str) -> bool {
        self.try_(
            line,
            |_, s| s.trim().strip_prefix('>').and_then(|u| u.strip_suffix('<')),
            |this, inner| {
                if this.state == State::InBlock {
                    if let Some(Element::CenteredText(rs)) = this.elements.last_mut() {
                        rs.push_str("\n");
                        rs.push_str(inner);
                        return;
                    }
                }

                let rs = RichString::from(inner);
                this.elements.push(Element::CenteredText(rs));

                this.state = State::InBlock;
            },
        )
    }

    fn try_lyrics(&mut self, line: &str) -> bool {
        self.try_(
            line,
            |_, s| s.trim_start().strip_prefix('~'),
            |this, inner| {
                if this.state == State::InBlock {
                    if let Some(Element::Lyrics(rs)) = this.elements.last_mut() {
                        rs.push_str("\n");
                        rs.push_str(inner);
                        return;
                    }
                }

                let rs = RichString::from(inner);
                this.elements.push(Element::Lyrics(rs));

                this.state = State::InBlock;
            },
        )
    }

    fn try_action(&mut self, line: &str) -> bool {
        self.try_(
            line,
            |_, line| Some(line),
            |this, inner| {
                if this.state == State::InBlock {
                    if let Some(Element::Action(rs)) = this.elements.last_mut() {
                        rs.push_str("\n");
                        rs.push_str(inner);
                        return;
                    }
                }

                let rs = RichString::from(inner);
                this.elements.push(Element::Action(rs));

                this.state = State::InBlock;
            },
        )
    }

    #[inline]
    fn starts_with_ascii_ci(s: &str, pat: &str) -> bool {
        let n = pat.len();
        s.as_bytes()
            .get(..n)
            .map_or(false, |head| head.eq_ignore_ascii_case(pat.as_bytes()))
    }

    fn try_heading(&mut self, line: &str) -> bool {
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

                // INT./EXT, INT/EXT covered by INT
                let pats = ["INT", "EXT", "EST", "I/E"];

                (pats
                    .iter()
                    .any(|p| Parser::starts_with_ascii_ci(trimmed, p))
                    && this.next_line_is_empty())
                .then_some(trimmed)
            },
            |this, inner| {
                this.elements.push(Element::Heading {
                    slug: RichString::from(inner),
                    number: None,
                });

                this.lines.next();
            },
        )
    }

    fn try_dialouge_start(&mut self, line: &str) -> bool {
        self.try_(
            line,
            |this, line| {
                let trimmed = line.trim_start();
                if let Some(inner) = trimmed.strip_prefix('@') {
                    return Some(inner);
                }

                let head = trimmed.split_once('(').map_or(trimmed, |(h, _)| h);
                let has_alpha = head.chars().any(|c| c.is_alphabetic());
                let has_lower = head.chars().any(|c| c.is_lowercase());
                (has_alpha && !has_lower && !this.next_line_is_empty()).then_some(trimmed)
            },
            |this, inner| {
                if let Some(inner) = inner.strip_suffix('^') {
                    if let Some(&Element::Dialogue(_)) = this.elements.last() {
                        if let Some(Element::Dialogue(d)) = this.elements.pop() {
                            let mut new_dialogue = Dialogue::new();
                            new_dialogue.character = RichString::from(inner);
                            this.elements.push(Element::DualDialogue(d, new_dialogue));
                        }
                    }
                } else {
                    let mut dialogue = Dialogue::new();
                    dialogue.character = RichString::from(inner);
                    this.elements.push(Element::Dialogue(dialogue));
                }

                this.state = State::InDialogue;
            },
        )
    }

    fn try_transition(&mut self, line: &str) -> bool {
        self.try_(
            line,
            |this, line| {
                if let Some(inner) = line.trim_start().strip_suffix('>') {
                    if !line.trim_end().ends_with('<') {
                        return Some(inner);
                    }
                }

                let transition_ending = line.ends_with("TO:");
                let has_lower = line.chars().any(|c| c.is_lowercase());
                let transition_elem = transition_ending && !has_lower;

                (transition_elem && this.next_line_is_empty()).then_some(line)
            },
            |this, inner| {
                this.elements
                    .push(Element::Transition(RichString::from(inner)));

                this.lines.next();
            },
        )
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

    fn next_line_is_empty(&mut self) -> bool {
        self.lines.peek().map_or(true, |s| s.trim().is_empty())
    }
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

#[derive(Debug, PartialEq, Eq)]
enum State {
    Default,
    InDialogue,
    InBlock,
}
