use crate::rich_string;
use crate::rich_string::RichString;
use crate::screenplay::Element;
use crate::screenplay::Screenplay;
use crate::screenplay::TitlePage;
use std::iter::Peekable;
use std::str::Lines;

pub fn parse(src: &str) -> Screenplay {
    Parser::new(src).parse()
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

        Screenplay {
            titlepage: self.title_page,
            elements: self.elements,
        }
    }

    fn parse_title(&mut self) {
        let mut tp = TitlePage::new();

        loop {
            let Some(line) = self.lines.peek() else {
                break;
            };

            let Some((key, val)) = line.split_once(':') else {
                break;
            };

            let mut values = Vec::new();

            if !val.trim().is_empty() {
                values.push(parse_rich_string(val));
                self.lines.next();
            } else {
                loop {
                    self.lines.next();
                    let value = self.lines.peek().unwrap_or(&"");
                    if value.starts_with("   ") {
                        values.push(parse_rich_string(value.trim()));
                    } else {
                        break;
                    }
                }
            }

            match key {
                "Title" => tp.title = values,
                "Credit" => tp.credit = values,
                "Author" | "Authors" => tp.authors = values,
                "Source" => tp.source = values,
                "Draft date" | "Draft Date" => tp.draft_date = values,
                "Contact" => tp.contact = values,
                _ => (),
            }
        }

        if !tp.title.is_empty()
            || !tp.credit.is_empty()
            || !tp.authors.is_empty()
            || !tp.source.is_empty()
            || !tp.draft_date.is_empty()
            || tp.contact.is_empty()
        {
            self.title_page = Some(tp);
        }
    }
}

fn parse_rich_string(text: &str) -> RichString {
    let mut elements = Vec::new();
    elements.push(rich_string::Element::new(text.to_owned()));

    RichString { elements }
}

#[derive(Debug, PartialEq, Eq)]
enum State {
    Default,
    InAction,
    InDialouge,
}
