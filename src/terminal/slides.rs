use super::formatting::{text_formatting::*, markdown_formatting};

pub struct Slide {
    pub dimensions: (usize, usize),
    pub margins: (usize, usize),
    pub text_align: Align,
    outline: bool,
    lines: Vec<Line>,
    bordered_text: String,
    pub border_chars: Vec<String>,
}

impl Slide {
    pub fn new(mut contents: &str, dimensions: &(usize, usize), margins: &(usize, usize)) -> Slide {
        let (align, found): (Align, bool) = {
            let first_line = contents.lines().nth(0).unwrap();
            match first_line {
                "**align_left**" => (Align::Left, true),
                "**align_center**" => (Align::Center, true),
                "**align_right**" => (Align::Right, true),
                _ => (Align::Left, false),
            }
        };
        if found {
            contents = contents.split_once("\n").unwrap().1;
        }
        let lines: Vec<Line> = slice_str(contents, dimensions, margins).iter().map(|line|{
            Line::new(&line, margins.to_owned(), dimensions.0, &align)
        }).collect();
        // upper left, upper right, lower left, lower right, horizontal dash, vertical dash
        let border_chars: Vec<String> = vec!["┌", "┐", "└", "┘", "─", "│"].iter().map(|char| char.to_owned().to_owned()).collect();
        Slide {
            dimensions: dimensions.to_owned(),
            margins: margins.to_owned(),
            text_align: align.to_owned(),
            outline: true,
            bordered_text: border_text(&lines, dimensions.0, dimensions.1-1, &border_chars),
            lines: lines,
            border_chars: border_chars,
        }
    }
    pub fn display(&self) {
        if self.outline {
            println!("{}", self.bordered_text);
        } else {
            for line in &self.lines {
                println!("{}", line.contents);
            }
        }

    }
}

pub struct Line {
    contents: String,
    space_fill: (usize, usize),
    box_character: String,
}

impl Line {
    pub fn new(contents: &str, margins: (usize, usize), terminal_width: usize, align: &Align) -> Line {
        let content_length = contents.len();
        let margin_length = margins.0 + margins.1;
        let total_filled_length = content_length + margin_length;
        let missing_line_space = terminal_width - total_filled_length;
        //println!("Width: {terminal_width}\nContent length: {content_length}\nMargin length: {margin_length}\nTotal: {total_filled_length}\nMissing spaces: {missing_line_space}");
        let space_filling: (usize, usize) = match align {
            Align::Left => {
                (margins.0, margins.1 + missing_line_space)
            },
            Align::Center => {
                if missing_line_space%2==0 {
                    (missing_line_space/2 + margins.0, missing_line_space/2 + margins.1)
                } else {
                    (missing_line_space/2 + 1 + margins.0, missing_line_space/2 + margins.1)
                }
            },
            Align::Right => {
                (margins.0 + missing_line_space, margins.1)
            }
        };
        Line {
            contents: contents.to_owned(),
            space_fill: space_filling,
            box_character: "\u{2502}".to_owned(),
        }
    }
    pub fn format_line(&self, boxed: bool, highlighting: bool) -> String {
        let mut spaced_line: String = String::new();
        let box_character_length: usize = 1;
        //println!("{} length: {}", self.box_character, box_character_length);
        let left_space = if boxed {
            //println!("{}", self.space_fill.0);
            self.box_character.to_owned() + &String::from_utf8(vec![b' '; self.space_fill.0 - box_character_length]).unwrap()
        } else {
            String::from_utf8(vec![b' '; self.space_fill.0]).unwrap()
        };
        let right_space = if boxed {
            //println!("{}", self.space_fill.1);
            String::from_utf8(vec![b' '; self.space_fill.1 - box_character_length]).unwrap() + &self.box_character
        } else {
            String::from_utf8(vec![b' '; self.space_fill.1]).unwrap()
        };
        spaced_line.push_str(&left_space);
        spaced_line.push_str(
            &if highlighting{
                markdown_formatting::colorize_headings(&self.contents)
            } else {
                self.contents.clone()
            }
        );
        spaced_line.push_str(&right_space);
        return spaced_line;
    }
}