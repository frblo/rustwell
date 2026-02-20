use std::{io::Write, sync::Arc};

use krilla::{
    Document,
    geom::Point,
    surface::Surface,
    text::{Font, TextDirection},
};

use crate::{Exporter, Screenplay, rich_string::RichString};

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

        let pdf = document
            .finish()
            .map_err(|_| std::io::Error::other("failed to create pdf"))?;
        writer.write_all(&pdf)
    }
}

fn write_line(
    surface: &mut Surface,
    x: f32,
    y: f32,
    content: &RichString,
    mut start_index: usize,
    breakpoint: &BreakPoint,
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

    let mut index = start_index;
    let mut line_index = 0;
    while index < breakpoint.index {
        let (string_element, relative_index) = match content.get_element_from_index(index) {
            Some(res) => res,
            None => todo!(),
        };

        let relative_break_index = if breakpoint.index - index >= string_element.text.len() - index
        {
            string_element.text.len()
        } else {
            breakpoint.index - (index - relative_index)
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
        surface.draw_text(
            Point::from_xy(x + (line_index as f32 * FONT_WIDTH), y),
            font.clone(),
            FONT_SIZE as f32,
            &string_element.text[relative_index..relative_break_index],
            false,
            TextDirection::Auto,
        );

        line_index += relative_break_index - relative_index;
        index = start_index + line_index;
    }
}

fn glyph_span(point_span: usize, font_size: usize) -> usize {
    point_span / font_size
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
    if span < 2 {
        panic!("Character span cannot be smaller than 2");
    }

    let mut brekpoints = Vec::with_capacity(content.len() / span + 1);
    let mut last_space_char = (0, 0);
    let mut line_len = 0;
    for i in 0..content.len() {
        let glyph = match content.get_char(i) {
            Some(g) => g,
            None => break,
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

        if glyph.is_whitespace() {
            last_space_char = (brekpoints.len() + 1, i);
            continue;
        }

        if glyph == '-' {
            last_space_char = (brekpoints.len() + 1, i);
            continue;
        }

        if line_len >= span {
            if brekpoints.len() + 1 != last_space_char.0 {
                brekpoints.push(BreakPoint {
                    index: i,
                    break_type: BreakType::BreakWord,
                });
                line_len = 0;
                continue;
            }

            brekpoints.push(BreakPoint {
                index: last_space_char.1 + 1,
                break_type: BreakType::NewLine,
            });
            line_len = i - last_space_char.1;
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
}
