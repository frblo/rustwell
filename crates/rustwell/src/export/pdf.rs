use std::io::Write;

use krilla::{
    Document,
    color::rgb,
    destination::XyzDestination,
    geom::{PathBuilder, Point, Rect},
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
    screenplay::{Dialogue, DialogueElement, Element, TitlePage},
};

const FONT_SIZE: usize = 12; // standard screenplay size
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

/// A family of fonts with the standard variants.
struct FontFamily {
    pub regular: Font,
    pub bold: Font,
    pub italic: Font,
    pub bold_italic: Font,
}

/// Dimensions of a paper in points (pts).
pub struct PaperSize {
    pub x: usize,
    pub y: usize,
}

/// The size of an `A4` paper in points (pts).
pub const A4: PaperSize = PaperSize { x: 595, y: 842 }; // A4 size in pts
/// The size of a `US letter` paper in points (pts).
pub const LETTER: PaperSize = PaperSize { x: 612, y: 792 }; // Letter size in pts

impl Default for PaperSize {
    fn default() -> Self {
        A4
    }
}

/// The margin at the top of a page. Applicable on every page. In points.
const TOP_MARGIN: usize = 72;
/// The margin at the bottom of a page. Applicable on every page. In points.
const BOTTOM_MARGIN: usize = 72;

/// Left- and right margins, in points, going inwards - meaning left margin is relative to the left
/// side, and the right margin is relative to the right side.
struct Margin {
    pub left: f32,
    pub right: f32,
}

/// Collection of margins for the dialogue components.
struct DialogueMargins {
    pub character: Margin,
    pub parenthetical: Margin,
    pub line: Margin,
}

/// Collection of margins for the dual dialogue components.
struct DualDialogueMargins {
    pub left: DialogueMargins,
    pub right: DialogueMargins,
}

/// Collection of all margins for all different screenplay [`Elements`].
struct Margins {
    pub heading: Margin,
    pub action: Margin,
    pub dialogue: DialogueMargins,
    pub dual_dialogue: DualDialogueMargins,
    pub lyrics: Margin,
    pub transition: Margin,
    pub centered: Margin,
    pub synopsis: Margin,
    pub page_number: Margin,
}

/// The standard margins for all different screenplay [`Elements`].
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
    page_number: Margin {
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
    /// What size (type) of paper (e.g. A4 or US letter)
    pub paper_size: PaperSize,
}

impl Exporter for PdfExporter {
    /// The `.pdf` extension.
    fn file_extension(&self) -> &'static str {
        "pdf"
    }

    /// Exports a `pdf` file and writes it to the provided writer. The pdf creation can fail if
    /// certain elements do not fit within a single page.
    fn export(&self, screenplay: &Screenplay, writer: &mut dyn Write) -> std::io::Result<()> {
        let mut document = Document::new();

        let fonts = FontFamily {
            regular: Font::new(FONTS[0].into(), 0).unwrap(),
            bold: Font::new(FONTS[1].into(), 0).unwrap(),
            italic: Font::new(FONTS[2].into(), 0).unwrap(),
            bold_italic: Font::new(FONTS[3].into(), 0).unwrap(),
        };

        self.generate_pdf(&mut document, &self.paper_size, screenplay, &fonts)?;

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
    /// Generates a `pdf` document from a [`Screenplay`]. Runs in (more or less) a single pass.
    fn generate_pdf(
        &self,
        document: &mut Document,
        size: &PaperSize,
        screenplay: &Screenplay,
        fonts: &FontFamily,
    ) -> std::io::Result<()> {
        // The index for which element in the screenplay is currently being processed.
        let mut screenplay_element_idx = 0;

        // The index for which page in the document is currently being written.
        let mut page_idx = 0;

        // The maximum number of writable lines which can fit on a page, considering the top and
        // bottom margins.
        let max_lines_per_page = (size.y - (TOP_MARGIN + BOTTOM_MARGIN)) / FONT_SIZE - 1;
        // If an element does not fit within a page, this will be Some(index) pointing to the which
        // breakpoint in the breakpoint list to start at on the next page.
        let mut residual_breakpoint_idx = None;
        let mut residual_dialogue_idx = None;

        let mut residual_dual_dialogue_idx = (None, None);
        let mut residual_dual_breakpoint_idx = (None, None);

        let mut outline = Outline::new();

        if let Some(t) = &screenplay.titlepage {
            page_idx += 1;
            write_titlepage(size, t, max_lines_per_page, document, fonts)?;
        }

        // Page loop, creates a new page and writes everything it can on it.
        while screenplay_element_idx < screenplay.elements.len() {
            let mut page = document
                .start_page_with(PageSettings::from_wh(size.x as f32, size.y as f32).unwrap());
            let mut surface = page.surface();
            let mut line_idx = 0;

            // Writes the page number.
            if (screenplay.titlepage.is_none() && page_idx > 0)
                || (screenplay.titlepage.is_some() && page_idx > 1)
            {
                let residual_page_number = write_element_custom_top_margin(
                    size,
                    &format!(
                        "{}.",
                        if screenplay.titlepage.is_some() {
                            page_idx
                        } else {
                            page_idx + 1
                        }
                    )
                    .into(),
                    &MARGINS.page_number,
                    &mut 0,
                    &mut 0,
                    1,
                    &mut surface,
                    fonts,
                    Alignment::RightToLeft,
                    36,
                )?;

                // Page number cannot be larger than what fits on a single line on the page.
                if residual_page_number.is_some() {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "There cannot be more pages than the number which fits on a page.",
                    ));
                }
            }

            // Element loop, iterates through the screenplay elements.
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
                        // If we're in a dialogue element, we need to preserve
                        // `residual_breakpoint_idx`.
                        if !matches!(element, Element::Dialogue(_)) {
                            residual_breakpoint_idx = None;
                        }
                        i
                    }
                    None => 0,
                };

                /// Macro for the most common usage of write_element(...), as most types of
                /// [`Element`]s call this function with almost identical parameters.
                macro_rules! write_element {
                    ($content:expr, $margin:expr, $text_direction:expr) => {
                        residual_breakpoint_idx = write_element(
                            size,
                            $content,
                            $margin,
                            &mut breakpoint_idx,
                            &mut line_idx,
                            max_lines_per_page,
                            &mut surface,
                            fonts,
                            $text_direction,
                        )?
                    };
                }

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
                                rich_number,
                                &left_number_margin,
                                &mut 0,
                                &mut initial_line_index.clone(),
                                max_lines_per_page,
                                &mut surface,
                                fonts,
                                Alignment::LeftToRight,
                            )?;

                            write_element(
                                size,
                                rich_number,
                                &right_number_margin,
                                &mut 0,
                                &mut initial_line_index.clone(),
                                max_lines_per_page,
                                &mut surface,
                                fonts,
                                Alignment::RightToLeft,
                            )?;
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
                        write_element!(slug, &MARGINS.heading, Alignment::LeftToRight);
                    }
                    Element::Action(s) => {
                        write_element!(s, &MARGINS.action, Alignment::LeftToRight);
                    }
                    Element::Dialogue(dialogue) => {
                        let premature_exit = write_dialogue(
                            dialogue,
                            &mut residual_dialogue_idx,
                            &mut residual_breakpoint_idx,
                            size,
                            max_lines_per_page,
                            &mut line_idx,
                            &mut surface,
                            fonts,
                            &MARGINS.dialogue,
                        )?;
                        if residual_dialogue_idx.is_some() || premature_exit {
                            break;
                        }
                    }
                    Element::DualDialogue(dialogue0, dialogue1) => {
                        let mut initial_line_index = line_idx;
                        let mut premature_exit = false;
                        // Ensures dual dialogue is only written it either both have not been
                        // written, or if this specifically is to be continued on a new page.
                        if (residual_dual_dialogue_idx.0.is_none()
                            && residual_dual_dialogue_idx.1.is_none())
                            || residual_dual_dialogue_idx.0.is_some()
                        {
                            premature_exit = premature_exit
                                || write_dialogue(
                                    dialogue0,
                                    &mut residual_dual_dialogue_idx.0,
                                    &mut residual_dual_breakpoint_idx.0,
                                    size,
                                    max_lines_per_page,
                                    &mut line_idx,
                                    &mut surface,
                                    fonts,
                                    &MARGINS.dual_dialogue.left,
                                )?;
                        }
                        if (residual_dual_dialogue_idx.1.is_none()
                            && residual_dual_dialogue_idx.0.is_none())
                            || residual_dual_dialogue_idx.1.is_some()
                        {
                            premature_exit = premature_exit
                                || write_dialogue(
                                    dialogue1,
                                    &mut residual_dual_dialogue_idx.1,
                                    &mut residual_dual_breakpoint_idx.1,
                                    size,
                                    max_lines_per_page,
                                    &mut initial_line_index,
                                    &mut surface,
                                    fonts,
                                    &MARGINS.dual_dialogue.right,
                                )?;
                        }
                        line_idx = line_idx.max(initial_line_index);
                        if residual_dual_dialogue_idx.0.is_some()
                            || residual_dual_dialogue_idx.1.is_some()
                            || premature_exit
                        {
                            break;
                        }
                    }
                    Element::Lyrics(s) => {
                        write_element!(s, &MARGINS.lyrics, Alignment::RightToLeft);
                    }
                    Element::Transition(s) => {
                        write_element!(s, &MARGINS.transition, Alignment::RightToLeft);
                    }
                    Element::CenteredText(s) => {
                        write_element!(s, &MARGINS.centered, Alignment::Centered);
                    }
                    Element::Synopsis(s) => {
                        if self.synopses {
                            surface.set_fill(Some(Fill {
                                paint: rgb::Color::new(143, 143, 143).into(),
                                opacity: NormalizedF32::new(0.5).unwrap(),
                                rule: Default::default(),
                            }));
                            write_element!(s, &MARGINS.synopsis, Alignment::LeftToRight);
                            surface.set_fill(None);
                        }
                    }
                    Element::PageBreak => {
                        screenplay_element_idx += 1;
                        break;
                    }
                }

                line_idx += 1;

                if residual_breakpoint_idx.is_some() {
                    continue;
                }

                screenplay_element_idx += 1;
            }

            surface.finish();
            page.finish();
            page_idx += 1;
        }
        document.set_outline(outline);

        Ok(())
    }
}

/// Writes a diologue [`Element`] to the `pdf` document. If a dialogue spans multiple pages it will
/// write the character name with the extension `(cont'd)` on each new page. Returns a
/// [Option<bool>] which is true if the whole dialogue element did not fit on the same page.
fn write_dialogue(
    dialogue: &Dialogue,
    residual_dialogue: &mut Option<usize>,
    residual_index: &mut Option<usize>,
    size: &PaperSize,
    max_lines: usize,
    line_index: &mut usize,
    surface: &mut Surface,
    fonts: &FontFamily,
    dialogue_margins: &DialogueMargins,
) -> std::io::Result<bool> {
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

    if name_lines_count >= max_lines {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Character name cannot be longer than a whole page.",
        ));
    }

    if *line_index + name_lines_count + 1 >= max_lines {
        return Ok(true);
    }

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
    )?;

    let mut dialogue_index = residual_dialogue.unwrap_or(0);
    while dialogue_index < dialogue.elements.len() {
        if *line_index >= max_lines {
            *residual_dialogue = Some(dialogue_index);
            // If a dialogue continues on the next page, writes `(MORE)` at the bottom of the
            // current page, inside the bottom margin.
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
            )?;

            return Ok(true);
        }
        let mut breakpoint_index = match *residual_index {
            Some(i) => {
                *residual_index = None;
                i
            }
            None => 0,
        };

        let (content, margin) = match &dialogue.elements[dialogue_index] {
            DialogueElement::Parenthetical(s) => (s, &dialogue_margins.parenthetical),
            DialogueElement::Line(s) => (s, &dialogue_margins.line),
        };

        *residual_index = write_element(
            size,
            content,
            margin,
            &mut breakpoint_index,
            line_index,
            max_lines,
            surface,
            fonts,
            Alignment::LeftToRight,
        )?;

        if residual_index.is_some() {
            continue;
        }

        dialogue_index += 1;
    }

    *residual_dialogue = None;
    Ok(false)
}

/// Writes a [`Element`] to the `pdf` document. If it cannot fit the whole element on the page it
/// will return [`Some`] with an index to which [`BreakPoint`] the function made it to. Calls
/// [`write_element_custom_top_margin`] with the standard top margin.
fn write_element(
    size: &PaperSize,
    content: &RichString,
    margin: &Margin,
    breakpoint_index: &mut usize,
    line_index: &mut usize,
    max_lines: usize,
    surface: &mut Surface,
    fonts: &FontFamily,
    text_direction: Alignment,
) -> std::io::Result<Option<usize>> {
    write_element_custom_top_margin(
        size,
        content,
        margin,
        breakpoint_index,
        line_index,
        max_lines,
        surface,
        fonts,
        text_direction,
        TOP_MARGIN,
    )
}

/// Writes a [`Element`] to the `pdf` document. If it cannot fit the whole element on the page it
/// will return [`Some`] with an index to which [`BreakPoint`] the function made it to. Allows for
/// using a non-default top margin (for page numbers).
fn write_element_custom_top_margin(
    size: &PaperSize,
    content: &RichString,
    margin: &Margin,
    breakpoint_index: &mut usize,
    line_index: &mut usize,
    max_lines: usize,
    surface: &mut Surface,
    fonts: &FontFamily,
    text_direction: Alignment,
    top_margin: usize,
) -> std::io::Result<Option<usize>> {
    let left_margin = margin.left;
    let right_margin = margin.right;
    let span = glyph_span(size, left_margin, right_margin);
    let breakpoints = break_points(content, span);
    while *breakpoint_index <= breakpoints.len() {
        if *line_index >= max_lines {
            return Ok(Some(*breakpoint_index));
        }

        let start_index = if *breakpoint_index == 0 {
            0
        } else {
            breakpoints[*breakpoint_index - 1].index
        };
        write_line(
            surface,
            left_margin,
            (FONT_SIZE * *line_index + top_margin) as f32,
            content,
            start_index,
            breakpoints.get(*breakpoint_index),
            fonts,
            text_direction,
            size,
            margin,
        )?;
        *breakpoint_index += 1;
        *line_index += 1;
    }
    Ok(None)
}

/// Writes a single line (not to be confused with [`DialogueElement::Line`]) onto the page.
fn write_line(
    surface: &mut Surface,
    mut x: f32,
    y: f32,
    content: &RichString,
    mut start_index: usize, // the "global" index to the rich string
    breakpoint: Option<&BreakPoint>,
    fonts: &FontFamily,
    text_direction: Alignment,
    size: &PaperSize,
    margin: &Margin,
) -> std::io::Result<()> {
    match content.get_char(start_index) {
        // Newlines are handled as part of the layout engine, so it should never write ut a newline
        // character.
        Some(c) => {
            if c == '\n' {
                start_index += 1
            }
        }
        None => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Could not get character from source.",
            ));
        }
    }

    let (breakpoint_index, break_word) = match breakpoint {
        Some(b) => (b.index, b.break_type == BreakType::BreakWord),
        None => (content.len(), false),
    };

    // If we use a nonstandard [`Alignment`] we must move the `x` value of which we start writing
    // the line on.
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

    // Keeps track of the position on the line which the writer is currently at.
    let mut glyph_index = 0;
    // Writes until reaching the breakpoint position.
    while start_index < breakpoint_index {
        let (string_element, relative_index) = match content.get_element_from_index(start_index) {
            Some(res) => res,
            None => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Could not get rich string element.",
                ));
            }
        };

        let element_length = string_element.text.chars().count();

        // The local index in the rich string element at which to break the line.
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

        let glyphs_written = relative_break_index - relative_index;

        // Draws a line under the characters (to create an underline effect).
        if string_element.is_underline() {
            let underline = {
                let mut pb = PathBuilder::new();
                let r = Rect::from_xywh(
                    x + (glyph_index as f32 * FONT_WIDTH),
                    y + 0.5,
                    glyphs_written as f32 * FONT_WIDTH,
                    0.75,
                )
                .unwrap();
                pb.push_rect(r);
                pb.close();
                pb.finish().unwrap()
            };
            surface.draw_path(&underline);
        }

        glyph_index += glyphs_written;
        start_index += glyphs_written;
    }

    // Adds a `-` at the end of a line if the linebreak took place inside a word.
    if break_word {
        surface.draw_text(
            Point::from_xy(x + (glyph_index as f32 * FONT_WIDTH), y),
            fonts.regular.clone(),
            FONT_SIZE as f32,
            "-",
            false,
            krilla::text::TextDirection::LeftToRight,
        );
    }

    Ok(())
}

/// Margins for the [`TitlePage`] elements.
struct TitlePageMargins {
    pub title: Margin,
    pub credit: Margin,
    pub authors: Margin,
    pub source: Margin,
    pub draft_date: Margin,
    pub contact: Margin,
}

/// Standard margins for the [`TitlePage`] elements.
const TITLE_PAGE_MARGINS: TitlePageMargins = TitlePageMargins {
    title: Margin {
        left: 72.0,
        right: 72.0,
    },
    credit: Margin {
        left: 72.0,
        right: 72.0,
    },
    authors: Margin {
        left: 72.0,
        right: 72.0,
    },
    source: Margin {
        left: 72.0,
        right: 72.0,
    },
    draft_date: Margin {
        left: 315.0,
        right: 72.0,
    },
    contact: Margin {
        left: 72.0,
        right: 315.0,
    },
};

/// Writes the [`TitlePage`] to the `pdf` document. Fails if everything does not fit on one page,
/// but allows for overlapping contact information and draft dates with the rest of the elements.
fn write_titlepage(
    size: &PaperSize,
    titlepage: &TitlePage,
    max_lines: usize,
    document: &mut Document,
    fonts: &FontFamily,
) -> std::io::Result<()> {
    let mut page =
        document.start_page_with(PageSettings::from_wh(size.x as f32, size.y as f32).unwrap());
    let mut surface = page.surface();

    let mut line_idx = max_lines / 3;

    /// Writes the [`TitlePage`] elements using the [`write_element`] function. Will add newlines
    /// between each type of element, as per (some) standards.
    macro_rules! write_title_element {
        // For the elements which are centered an written below each other.
        ($element:ident) => {
            if !titlepage.$element.is_empty() {
                for s in &titlepage.$element {
                    let residual = write_element(
                        size,
                        s,
                        &TITLE_PAGE_MARGINS.$element,
                        &mut 0,
                        &mut line_idx,
                        max_lines,
                        &mut surface,
                        fonts,
                        Alignment::Centered,
                    )?;

                    if residual.is_some() {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "Title page cannot be longer than a single page.",
                        ));
                    }
                }
                line_idx += 1;
            }
        };
        // For elements which have specific alignments and are written from the bottom up.
        ($element:ident, $alignment:expr) => {
            if !titlepage.$element.is_empty() {
                let mut total_lines = titlepage.$element.len();
                for s in &titlepage.$element {
                    total_lines += break_points(
                        s,
                        glyph_span(
                            size,
                            TITLE_PAGE_MARGINS.$element.left,
                            TITLE_PAGE_MARGINS.$element.right,
                        ),
                    )
                    .len();

                    if total_lines > max_lines {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "Title page cannot be longer than a single page.",
                        ));
                    }
                }
                line_idx = max_lines - total_lines;

                for s in &titlepage.$element {
                    write_element(
                        size,
                        s,
                        &TITLE_PAGE_MARGINS.$element,
                        &mut 0,
                        &mut line_idx,
                        max_lines,
                        &mut surface,
                        fonts,
                        $alignment,
                    )?;
                }
            }
        };
    }

    write_title_element!(title);
    write_title_element!(credit);
    write_title_element!(authors);
    write_title_element!(source);

    write_title_element!(contact, Alignment::LeftToRight);
    write_title_element!(draft_date, Alignment::RightToLeft);

    surface.finish();
    page.finish();
    Ok(())
}

/// Calculates the span in a margin. Returns how many characters with the standard font width will
/// fit between the left and right margin.
fn glyph_span(size: &PaperSize, left_margin: f32, right_margin: f32) -> usize {
    ((size.x as f32 - (left_margin + right_margin)) / FONT_WIDTH) as usize
}

/// The different ways to break a line into multiple lines.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum BreakType {
    /// Standard type of line break, by just adding a newline somewhere.
    NewLine,
    /// Break the line by adding a newline inside a word.
    BreakWord,
}

/// Where in the string a line break should occur.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct BreakPoint {
    pub index: usize,
    /// If it should add a newline after a word, or inside a word.
    pub break_type: BreakType,
}

/// Given a [`RichString`] and a number of allowed glyphs on a single line, calculates where
/// eventual [`BreakPoint`]s should be placed in the string.
///
/// Greedily places [`BreakPoint`]s. Will only place one inside a word if the word does not fit on
/// a line by itself. Respects newline characters in the string.
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
