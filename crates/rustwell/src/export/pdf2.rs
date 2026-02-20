use std::{io::Write, sync::Arc};

use krilla::{
    Document,
    geom::Point,
    page::PageSettings,
    surface::Surface,
    text::{Font, TextDirection},
};

use crate::{Exporter, Screenplay, rich_string::RichString, screenplay::Element};

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

struct PaperSize {
    pub x: usize,
    pub y: usize,
}

const A4: PaperSize = PaperSize { x: 595, y: 842 }; // A4 size in pts
const TOP_MARGIN: usize = 72;
const BOTTOM_MARGIN: usize = 72;

/// A [`Screenplay`] exporter for `pdf`
///
/// The variables configure the exporter
#[derive(Default)]
pub struct Pdf2Exporter {
    /// Whether to include synopses in the output
    pub synopses: bool,
}

impl Exporter for Pdf2Exporter {
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

        generate_pdf(&mut document, &A4, screenplay, &fonts);

        let pdf = document
            .finish()
            .map_err(|_| std::io::Error::other("failed to create pdf"))?;
        writer.write_all(&pdf)
    }
}

fn generate_pdf(document: &mut Document, size: &PaperSize, screenplay: &Screenplay, fonts: &Fonts) {
    let mut screenplay_index = 0;

    let max_lines = (size.y - (TOP_MARGIN + BOTTOM_MARGIN)) / FONT_SIZE - 1;
    let mut residual_index = None;

    while screenplay_index < screenplay.elements.len() {
        let mut page =
            document.start_page_with(PageSettings::from_wh(size.x as f32, size.y as f32).unwrap());
        let mut surface = page.surface();
        let mut line_index = 0;

        loop {
            if line_index >= max_lines {
                break;
            }

            if screenplay_index >= screenplay.elements.len() {
                break;
            }

            let element = &screenplay.elements[screenplay_index];
            let mut breakpoint_index = match residual_index {
                Some(i) => {
                    residual_index = None;
                    i
                }
                None => 0,
            };

            match &element {
                Element::Action(s) => write_element(
                    size,
                    s,
                    108.0,
                    72.0,
                    &mut breakpoint_index,
                    &mut line_index,
                    max_lines,
                    &mut surface,
                    fonts,
                ),
                _ => unimplemented!(),
            }

            line_index += 1;
            screenplay_index += 1;
        }

        surface.finish();
        page.finish();
    }
}

fn write_element(
    size: &PaperSize,
    content: &RichString,
    left_margin: f32,
    right_margin: f32,
    breakpoint_index: &mut usize,
    line_index: &mut usize,
    max_lines: usize,
    surface: &mut Surface,
    fonts: &Fonts,
) {
    let span = glyph_span(size, left_margin, right_margin);
    let breakpoints = break_points(content, span);
    while *breakpoint_index <= breakpoints.len() {
        if *line_index >= max_lines {
            break;
        }

        let start_index = if *breakpoint_index == 0 {
            0
        } else {
            breakpoints[*breakpoint_index - 1].index
        };
        write_line(
            surface,
            108.0,
            (FONT_SIZE * *line_index + TOP_MARGIN) as f32,
            content,
            start_index,
            breakpoints.get(*breakpoint_index),
            fonts,
        );
        *breakpoint_index += 1;
        *line_index += 1;
    }
}

fn write_line(
    surface: &mut Surface,
    x: f32,
    y: f32,
    content: &RichString,
    mut start_index: usize,
    breakpoint: Option<&BreakPoint>,
    fonts: &Fonts,
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

    let mut glyph_index = 0;
    while start_index < breakpoint_index {
        let (string_element, relative_index) = match content.get_element_from_index(start_index) {
            Some(res) => res,
            None => todo!(),
        };

        let relative_break_index = if breakpoint_index - start_index
            >= string_element.text.chars().count() - relative_index
        {
            string_element.text.chars().count()
        } else {
            breakpoint_index - (start_index - relative_index)
        };
        let font = match (
            string_element.is_bold(),
            string_element.is_italic(),
            string_element.is_underline(),
        ) {
            (false, false, _) => &fonts.regular,
            (true, false, _) => &fonts.bold,
            (false, true, _) => &fonts.italic,
            (true, true, _) => &fonts.bold_italic,
        };
        let mut char_indices = string_element.text.char_indices();
        let start_byte_index = char_indices.nth(relative_index).unwrap().0;
        let end_byte_index = char_indices
            .nth(relative_break_index - relative_index - 2)
            .unwrap()
            .0;

        dbg!(&string_element.text);
        dbg!(&start_index);
        dbg!(&breakpoint_index);
        // dbg!(&end_byte_index);
        surface.draw_text(
            Point::from_xy(x + (glyph_index as f32 * FONT_WIDTH), y),
            font.clone(),
            FONT_SIZE as f32,
            &string_element.text[start_byte_index..=end_byte_index],
            false,
            TextDirection::Auto,
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
    assert!(span >= 2);

    let mut brekpoints = Vec::with_capacity(content.len() / span + 1);
    let mut last_whitespace_char = (0, 0);
    let mut line_len = 0;
    for i in 0..content.len() {
        let glyph = match content.get_char(i) {
            Some(g) => g,
            None => panic!(),
        };

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
