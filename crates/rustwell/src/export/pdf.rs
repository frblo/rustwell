use std::{io::Write, sync::Arc};

use krilla::{
    Document,
    color::rgb,
    destination::XyzDestination,
    geom::Point,
    num::NormalizedF32,
    outline::{Outline, OutlineNode},
    page::PageSettings,
    paint::Fill,
    surface::Surface,
    text::Font,
};

use crate::{
    Exporter, Screenplay,
    rich_string::RichString,
    screenplay::{Dialogue, DialogueElement, Element},
};

const FONT_SIZE: usize = 12;
const FONT_WIDTH: f32 = 7.2; // 12 * 0.6 (Courier Prime's aspect ratio)

/// The font bundled together with Rustwell; Courier Prime.
/// Includes the data of the font styles Regular, Bold, Italic
/// and BoldItalic, in stated order.
const FONTS: [&[u8]; 4] = [
    include_bytes!("fonts/CourierPrime-Regular.ttf"),
    include_bytes!("fonts/CourierPrime-Bold.ttf"),
    include_bytes!("fonts/CourierPrime-Italic.ttf"),
    include_bytes!("fonts/CourierPrime-BoldItalic.ttf"),
];

struct Fonts {
    pub regular: Font,
    pub bold: Font,
    pub italic: Font,
    pub bold_italic: Font,
}

pub struct PaperSize {
    pub x: usize,
    pub y: usize,
}

pub const A4: PaperSize = PaperSize { x: 595, y: 842 }; // A4 size in pts
pub const LETTER: PaperSize = PaperSize { x: 612, y: 792 }; // Letter size in pts

const TOP_MARGIN: usize = 72;
const BOTTOM_MARGIN: usize = 72;

struct Margin {
    pub left: f32,
    pub right: f32,
}

struct DialogueMargins {
    pub character: Margin,
    pub parenthetical: Margin,
    pub line: Margin,
}

struct DualDialogueMargins {
    pub left: DialogueMargins,
    pub right: DialogueMargins,
}

struct Margins {
    pub heading: Margin,
    pub action: Margin,
    pub dialogue: DialogueMargins,
    pub dual_dialogue: DualDialogueMargins,
    pub lyrics: Margin,
    pub transition: Margin,
    pub centered: Margin,
    pub synopsis: Margin,
}

const MARGINS: Margins = Margins {
    heading: Margin {
        left: 108.0,
        right: 72.0,
    },
    action: Margin {
        left: 108.0,
        right: 72.0,
    },
    dialogue: DialogueMargins {
        character: Margin {
            left: 252.0,
            right: 108.0,
        },
        parenthetical: Margin {
            left: 223.2,
            right: 180.0,
        },
        line: Margin {
            left: 180.0,
            right: 144.0,
        },
    },
    dual_dialogue: DualDialogueMargins {
        left: DialogueMargins {
            character: Margin {
                left: 198.0,
                right: 288.0,
            },
            parenthetical: Margin {
                left: 162.0,
                right: 324.0,
            },
            line: Margin {
                left: 144.0,
                right: 288.0,
            },
        },
        right: DialogueMargins {
            character: Margin {
                left: 414.0,
                right: 72.0,
            },
            parenthetical: Margin {
                left: 378.0,
                right: 90.0,
            },
            line: Margin {
                left: 360.0,
                right: 72.0,
            },
        },
    },
    lyrics: Margin {
        left: 180.0,
        right: 144.0,
    },
    transition: Margin {
        left: 144.0,
        right: 144.0,
    },
    centered: Margin {
        left: 144.0,
        right: 144.0,
    },
    synopsis: Margin {
        left: 108.0,
        right: 72.0,
    },
};

/// A [`Screenplay`] exporter for `pdf`
///
/// The variables configure the exporter
#[derive(Default)]
pub struct PdfExporter {
    /// Whether to include synopses in the output
    pub synopses: bool,
}

impl Exporter for PdfExporter {
    fn file_extension(&self) -> &'static str {
        "pdf"
    }

    /// Exports a `pdf` file and writes it to the provided writer.
    fn export(&self, screenplay: &Screenplay, writer: &mut dyn Write) -> std::io::Result<()> {
        let regular_data: Arc<dyn AsRef<[u8]> + Send + Sync> = Arc::new(FONTS[0]);
        let bold_data: Arc<dyn AsRef<[u8]> + Send + Sync> = Arc::new(FONTS[1]);
        let italic_data: Arc<dyn AsRef<[u8]> + Send + Sync> = Arc::new(FONTS[2]);
        let bold_italic_data: Arc<dyn AsRef<[u8]> + Send + Sync> = Arc::new(FONTS[3]);
        let mut document = Document::new();

        let fonts = Fonts {
            regular: Font::new(regular_data.into(), 0).unwrap(),
            bold: Font::new(bold_data.into(), 0).unwrap(),
            italic: Font::new(italic_data.into(), 0).unwrap(),
            bold_italic: Font::new(bold_italic_data.into(), 0).unwrap(),
        };

        self.generate_pdf(&mut document, &A4, screenplay, &fonts);

        let pdf = document
            .finish()
            .map_err(|_| std::io::Error::other("failed to create pdf"))?;
        writer.write_all(&pdf)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Alignment {
    LeftToRight,
    RightToLeft,
    Centered,
}

impl PdfExporter {
    fn generate_pdf(
        &self,
        document: &mut Document,
        size: &PaperSize,
        screenplay: &Screenplay,
        fonts: &Fonts,
    ) {
        let mut screenplay_element_idx = 0;

        let mut page_idx = 0;

        let max_lines_per_page = (size.y - (TOP_MARGIN + BOTTOM_MARGIN)) / FONT_SIZE - 1;
        let mut residual_breakpoint_idx = None;
        let mut residual_dialogue_idx = None;

        let mut residual_dual_dialogue_idx = (None, None);
        let mut residual_dual_breakpoint_idx = (None, None);

        let mut outline = Outline::new();

        while screenplay_element_idx < screenplay.elements.len() {
            let mut page = document
                .start_page_with(PageSettings::from_wh(size.x as f32, size.y as f32).unwrap());
            let mut surface = page.surface();
            let mut line_idx = 0;

            loop {
                if line_idx >= max_lines_per_page {
                    break;
                }

                if screenplay_element_idx >= screenplay.elements.len() {
                    break;
                }

                let element = &screenplay.elements[screenplay_element_idx];
                let mut breakpoint_idx = match residual_breakpoint_idx {
                    Some(i) => {
                        residual_breakpoint_idx = None;
                        i
                    }
                    None => 0,
                };

                let mut we = |content, margin, text_direction| {
                    residual_breakpoint_idx = write_element(
                        size,
                        content,
                        margin,
                        &mut breakpoint_idx,
                        &mut line_idx,
                        max_lines_per_page,
                        &mut surface,
                        fonts,
                        text_direction,
                    )
                };

                match &element {
                    Element::Heading { slug, number } => {
                        if number.is_some() {
                            let initial_line_index = line_idx;

                            let left_number_margin = Margin {
                                left: 54.0,
                                right: size.x as f32 - MARGINS.heading.left + 18.0,
                            };
                            let right_number_margin = Margin {
                                left: size.x as f32 - MARGINS.heading.right + 18.0,
                                right: 18.0,
                            };

                            let rich_number = &number.as_ref().unwrap().into();

                            write_element(
                                size,
                                &rich_number,
                                &left_number_margin,
                                &mut 0,
                                &mut initial_line_index.clone(),
                                max_lines_per_page,
                                &mut surface,
                                fonts,
                                Alignment::LeftToRight,
                            );

                            write_element(
                                size,
                                &rich_number,
                                &right_number_margin,
                                &mut 0,
                                &mut initial_line_index.clone(),
                                max_lines_per_page,
                                &mut surface,
                                fonts,
                                Alignment::RightToLeft,
                            );
                        }
                        outline.push_child(OutlineNode::new(
                            slug.to_string(),
                            XyzDestination::new(
                                page_idx,
                                Point {
                                    x: MARGINS.heading.left,
                                    y: (TOP_MARGIN + (line_idx * FONT_SIZE) - FONT_SIZE) as f32,
                                },
                            ),
                        ));
                        residual_breakpoint_idx = write_element(
                            size,
                            slug,
                            &MARGINS.heading,
                            &mut breakpoint_idx,
                            &mut line_idx,
                            max_lines_per_page,
                            &mut surface,
                            fonts,
                            Alignment::LeftToRight,
                        )
                    }
                    Element::Action(s) => we(s, &MARGINS.action, Alignment::LeftToRight),
                    Element::Dialogue(dialogue) => {
                        write_dialogue(
                            dialogue,
                            &mut residual_dialogue_idx,
                            &mut residual_breakpoint_idx,
                            size,
                            max_lines_per_page,
                            &mut line_idx,
                            &mut surface,
                            fonts,
                            &MARGINS.dialogue,
                        );
                        if residual_dialogue_idx.is_some() {
                            break;
                        }
                    }
                    Element::DualDialogue(dialogue0, dialogue1) => {
                        let mut initial_line_index = line_idx;
                        if (residual_dual_dialogue_idx.0.is_none()
                            && residual_dual_dialogue_idx.1.is_none())
                            || residual_dual_dialogue_idx.0.is_some()
                        {
                            write_dialogue(
                                dialogue0,
                                &mut residual_dual_dialogue_idx.0,
                                &mut residual_dual_breakpoint_idx.0,
                                size,
                                max_lines_per_page,
                                &mut line_idx,
                                &mut surface,
                                fonts,
                                &MARGINS.dual_dialogue.left,
                            );
                        }
                        if (residual_dual_dialogue_idx.1.is_none()
                            && residual_dual_dialogue_idx.0.is_none())
                            || residual_dual_dialogue_idx.1.is_some()
                        {
                            write_dialogue(
                                dialogue1,
                                &mut residual_dual_dialogue_idx.1,
                                &mut residual_dual_breakpoint_idx.1,
                                size,
                                max_lines_per_page,
                                &mut initial_line_index,
                                &mut surface,
                                fonts,
                                &MARGINS.dual_dialogue.right,
                            );
                        }
                        line_idx = line_idx.max(initial_line_index);
                        if residual_dual_dialogue_idx.0.is_some()
                            || residual_dual_dialogue_idx.1.is_some()
                        {
                            break;
                        }
                    }
                    Element::Lyrics(s) => we(s, &MARGINS.lyrics, Alignment::RightToLeft),
                    Element::Transition(s) => we(s, &MARGINS.transition, Alignment::RightToLeft),
                    Element::CenteredText(s) => we(s, &MARGINS.centered, Alignment::Centered),
                    Element::Synopsis(s) => {
                        if self.synopses {
                            surface.set_fill(Some(Fill {
                                paint: rgb::Color::new(143, 143, 143).into(),
                                opacity: NormalizedF32::new(0.5).unwrap(),
                                rule: Default::default(),
                            }));
                            write_element(
                                size,
                                s,
                                &MARGINS.synopsis,
                                &mut breakpoint_idx,
                                &mut line_idx,
                                max_lines_per_page,
                                &mut surface,
                                fonts,
                                Alignment::LeftToRight,
                            );
                            surface.set_fill(None);
                        }
                    }
                    Element::PageBreak => {
                        screenplay_element_idx += 1;
                        break;
                    }
                }

                line_idx += 1;
                screenplay_element_idx += 1;
            }

            surface.finish();
            page.finish();
            page_idx += 1;
        }
        document.set_outline(outline);
    }
}

fn write_dialogue(
    dialogue: &Dialogue,
    residual_dialogue: &mut Option<usize>,
    residual_index: &mut Option<usize>,
    size: &PaperSize,
    max_lines: usize,
    line_index: &mut usize,
    surface: &mut Surface,
    fonts: &Fonts,
    dialogue_margins: &DialogueMargins,
) {
    let mut character_name = dialogue.character.clone();
    match (*residual_dialogue, &dialogue.extension) {
        (Some(_), _) => {
            character_name.append(" (cont'd)".into());
        }
        (None, Some(ext)) => {
            character_name.append(" (".into());
            character_name.append(ext.clone());
            character_name.append(")".into());
        }
        _ => (),
    };
    let span = glyph_span(
        size,
        dialogue_margins.character.left,
        dialogue_margins.character.right,
    );
    let name_lines_count = break_points(&character_name, span).len() + 1;
    if *line_index + name_lines_count + 1 >= max_lines {
        return;
    }
    assert!(name_lines_count < max_lines);

    write_element(
        size,
        &character_name,
        &dialogue_margins.character,
        &mut 0,
        line_index,
        max_lines,
        surface,
        fonts,
        Alignment::LeftToRight,
    );

    let mut dialogue_index = residual_dialogue.unwrap_or(0);
    while dialogue_index < dialogue.elements.len() {
        if *line_index + 1 >= max_lines {
            *residual_dialogue = Some(dialogue_index);
            write_element(
                size,
                &"(MORE)".into(),
                &dialogue_margins.character,
                &mut 0,
                line_index,
                max_lines + 1,
                surface,
                fonts,
                Alignment::LeftToRight,
            );

            return;
        }
        let mut breakpoint_index = match *residual_index {
            Some(i) => {
                *residual_index = None;
                i
            }
            None => 0,
        };

        match &dialogue.elements[dialogue_index] {
            DialogueElement::Parenthetical(s) => {
                *residual_index = write_element(
                    size,
                    &s,
                    &dialogue_margins.parenthetical,
                    &mut breakpoint_index,
                    line_index,
                    max_lines,
                    surface,
                    fonts,
                    Alignment::LeftToRight,
                )
            }
            DialogueElement::Line(s) => {
                *residual_index = write_element(
                    size,
                    &s,
                    &dialogue_margins.line,
                    &mut breakpoint_index,
                    line_index,
                    max_lines,
                    surface,
                    fonts,
                    Alignment::LeftToRight,
                )
            }
        }

        if residual_index.is_some() {
            continue;
        }

        dialogue_index += 1;
    }

    *residual_dialogue = None;
}

fn write_element(
    size: &PaperSize,
    content: &RichString,
    margin: &Margin,
    breakpoint_index: &mut usize,
    line_index: &mut usize,
    max_lines: usize,
    surface: &mut Surface,
    fonts: &Fonts,
    text_direction: Alignment,
) -> Option<usize> {
    let left_margin = margin.left;
    let right_margin = margin.right;
    let span = glyph_span(size, left_margin, right_margin);
    let breakpoints = break_points(content, span);
    while *breakpoint_index <= breakpoints.len() {
        if *line_index >= max_lines {
            return Some(*breakpoint_index);
        }

        let start_index = if *breakpoint_index == 0 {
            0
        } else {
            breakpoints[*breakpoint_index - 1].index
        };
        write_line(
            surface,
            left_margin,
            (FONT_SIZE * *line_index + TOP_MARGIN) as f32,
            content,
            start_index,
            breakpoints.get(*breakpoint_index),
            fonts,
            text_direction,
            size,
            margin,
        );
        *breakpoint_index += 1;
        *line_index += 1;
    }
    None
}

fn write_line(
    surface: &mut Surface,
    mut x: f32,
    y: f32,
    content: &RichString,
    mut start_index: usize,
    breakpoint: Option<&BreakPoint>,
    fonts: &Fonts,
    text_direction: Alignment,
    size: &PaperSize,
    margin: &Margin,
) {
    match content.get_char(start_index) {
        Some(c) => {
            if c == '\n' {
                start_index += 1
            }
        }
        None => todo!(),
    }

    let breakpoint_index = match breakpoint {
        Some(b) => b.index,
        None => content.len(),
    };

    match text_direction {
        Alignment::LeftToRight => (),
        Alignment::RightToLeft => {
            let line_length = breakpoint_index - start_index;
            let line_span = line_length as f32 * FONT_WIDTH;
            x += size.x as f32 - (margin.left + margin.right) - line_span;
        }
        Alignment::Centered => {
            let line_length = breakpoint_index - start_index;
            let line_span = (line_length / 2) as f32 * FONT_WIDTH;
            x = (size.x / 2) as f32 - line_span;
        }
    }

    let mut glyph_index = 0;
    while start_index < breakpoint_index {
        let (string_element, relative_index) = match content.get_element_from_index(start_index) {
            Some(res) => res,
            None => todo!(),
        };

        let element_length = string_element.text.chars().count();

        let relative_break_index =
            if breakpoint_index - start_index >= element_length - relative_index {
                element_length
            } else {
                breakpoint_index - (start_index - relative_index)
            };
        let font = match (string_element.is_bold(), string_element.is_italic()) {
            (false, false) => &fonts.regular,
            (true, false) => &fonts.bold,
            (false, true) => &fonts.italic,
            (true, true) => &fonts.bold_italic,
        };
        let mut char_indices = string_element.text.char_indices();
        let start_byte_index = char_indices.nth(relative_index).unwrap().0;
        let end_byte_index = match char_indices.nth(relative_break_index - relative_index - 1) {
            Some((i, _)) => i,
            None => string_element.text.len(),
        };

        surface.draw_text(
            Point::from_xy(x + (glyph_index as f32 * FONT_WIDTH), y),
            font.clone(),
            FONT_SIZE as f32,
            &string_element.text[start_byte_index..end_byte_index],
            false,
            krilla::text::TextDirection::LeftToRight,
        );

        glyph_index += relative_break_index - relative_index;
        start_index += relative_break_index - relative_index;
    }
}

fn glyph_span(size: &PaperSize, left_margin: f32, right_margin: f32) -> usize {
    ((size.x as f32 - (left_margin + right_margin)) / FONT_WIDTH) as usize
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum BreakType {
    NewLine,
    BreakWord,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct BreakPoint {
    pub index: usize,
    pub break_type: BreakType,
}

fn break_points(content: &RichString, span: usize) -> Vec<BreakPoint> {
    debug_assert!(span >= 2);

    let mut brekpoints = Vec::with_capacity(content.len() / span + 1);
    let mut last_whitespace_char = (0, 0);
    let mut line_len = 0;
    for (i, glyph) in content.iter().enumerate() {
        line_len += 1;
        if glyph == '\n' {
            brekpoints.push(BreakPoint {
                index: i,
                break_type: BreakType::NewLine,
            });
            line_len = 0;
            continue;
        }

        if glyph.is_whitespace() || glyph == '-' {
            last_whitespace_char = (brekpoints.len() + 1, i);
            continue;
        }

        if line_len >= span {
            if brekpoints.len() + 1 != last_whitespace_char.0 {
                brekpoints.push(BreakPoint {
                    index: i,
                    break_type: BreakType::BreakWord,
                });
                line_len = 0;
                continue;
            }

            brekpoints.push(BreakPoint {
                index: last_whitespace_char.1 + 1,
                break_type: BreakType::NewLine,
            });
            line_len = i - last_whitespace_char.1;
        }
    }
    brekpoints
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn breaks_simple() {
        let mut rs = RichString::new();
        rs.push_str("hello world");

        let breakpoints = break_points(&rs, 6);
        let correct = vec![BreakPoint {
            index: 6,
            break_type: BreakType::NewLine,
        }];

        assert_eq!(breakpoints, correct);
    }

    #[test]
    fn breaks_simple_with_newline() {
        let mut rs = RichString::new();
        rs.push_str("hello\nworld");

        let breakpoints = break_points(&rs, 100);
        let correct = vec![BreakPoint {
            index: 5,
            break_type: BreakType::NewLine,
        }];

        assert_eq!(breakpoints, correct);
    }

    #[test]
    fn breaks_simple_breakword() {
        let mut rs = RichString::new();
        rs.push_str("helloworld");

        let breakpoints = break_points(&rs, 6);
        let correct = vec![BreakPoint {
            index: 5,
            break_type: BreakType::BreakWord,
        }];

        assert_eq!(breakpoints, correct);
    }

    #[test]
    fn breaks_simple_utilizing_hyphen() {
        let mut rs = RichString::new();
        rs.push_str("hello-world");

        let breakpoints = break_points(&rs, 7);
        let correct = vec![BreakPoint {
            index: 6,
            break_type: BreakType::NewLine,
        }];

        assert_eq!(breakpoints, correct);
    }

    #[test]
    fn breaks_rich() {
        let mut rs = RichString::new();
        rs.push_str("he**ll**o wor*ld*");

        let breakpoints = break_points(&rs, 6);
        let correct = vec![BreakPoint {
            index: 6,
            break_type: BreakType::NewLine,
        }];

        assert_eq!(breakpoints, correct);
    }

    #[test]
    fn breaks_rich_longer() {
        let mut rs = RichString::new();
        rs.push_str("Bosse går till **affären** och köper lite mjölk, vilket han tycker är väldigt gott att äta.");

        let breakpoints = break_points(&rs, 60);
        let correct = vec![BreakPoint {
            index: 56,
            break_type: BreakType::NewLine,
        }];

        assert_eq!(breakpoints, correct);
    }
}
