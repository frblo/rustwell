use crate::rich_string::RichString;

const FONT_SIZE: usize = 12;
const FONT_WIDTH: f32 = 7.2;
const A4: (usize, usize) = (595, 842);

fn symbol_span(point_span: usize, font_size: usize) -> usize {
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
