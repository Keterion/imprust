use super::slides::Slide;
pub fn space(width: usize, margin: (usize, usize), input: &str, align: &Align, box_char: &str, separator: &str) -> String {
    let extra_whitespace = width - input.len() - (margin.0 + margin.1); // the remaining space in the line, also the box characters removed
    // the extra whitespace is the remaining space in a line after: text, margins, and the extra two characters for the box have been removed
    let mut l: String = String::new();
    l.push_str(box_char); // pushes the vertical box character
    if !validate_string_len(width, input, margin) {
        panic!("String is too large, can't space it");
    }
    match align {
        Align::Left => {
            for _ in 1..margin.0 {
                l.push_str(separator);
            }
            l.push_str(input);
            for _ in 1..(margin.1 + extra_whitespace) {
                l.push_str(separator);
            }
        },
        Align::Center => {
            for _ in 1..(margin.0 + extra_whitespace/2) {
                l.push_str(separator);
            }
            l.push_str(input);
            for _ in 1..(margin.1 + extra_whitespace/2) {
                l.push_str(separator);
            }
        },
        Align::Right => {
            for _ in 1..(margin.0 + extra_whitespace) {
                l.push_str(separator);
            }
            l.push_str(input);
            for _ in 1..margin.1 {
                l.push_str(separator);
            }
        }
    }
    if input.len() % 2 != 0 {
        l.push_str(separator); // you gotta do this because there's one character missing without it
    }
    l.push_str(box_char); // pushes the vertical box character
    // the for _ in 1..margin are started at 1 because the box character has to be accounted for
    return l;
}

fn validate_string_len(width: usize, input: &str, margin: (usize, usize)) -> bool {
    let usable = width - margin.0 - margin.1;
    if input.len() > usable {
        false
    } else {
        true
    }
}

#[allow(unused)]
pub enum Align {
    Left,
    Center,
    Right,
}

pub fn border_text(slide: &Slide) -> String {
    let (top_left, top_right, bottom_left, bottom_right) = ("\u{250c}", "\u{2510}", "\u{2514}", &format!("{}", slide.pos));
    let (horizontal_dash, vertical_dash) = ("\u{2500}", "\u{2502}");
    let separator: &str = " ";
    let mut s: String = String::new();
    
    let empty_height: usize = slide.dimensions.1 - slide.contents.lines().count();
    let empty_line: String = space(slide.dimensions.0, slide.margins, "", &slide.text_align, vertical_dash, separator) + "\n";
    // upper box
    s.push_str(top_left);
    for _ in 2..slide.dimensions.0 {
        s.push_str(horizontal_dash);
    }
    s.push_str(&(top_right.to_owned() + "\n"));

    for _ in 1..empty_height/2 {
        s.push_str(&empty_line);
    }

    // text
    for line in slide.contents.lines() { // prints the lines and the sides of the box
        s.push_str(&(space(slide.dimensions.0, slide.margins, line, &slide.text_align, vertical_dash, separator) + "\n"));
    }

    for _ in 1..empty_height/2 {
        s.push_str(&empty_line);
    }

    // lower box
    s.push_str(bottom_left);
    for _ in 2..slide.dimensions.0 {
        s.push_str(horizontal_dash);
    }
    s.push_str(bottom_right);

    return s;
}

pub fn slice_str(data: &str, dimensions: &(usize, usize), margins: &(usize, usize)) -> String {
    // width = self.dimensions.0
    let mut split_str: String = String::new();
    let remaining = data.clone().lines();
    for mut line in remaining {
        while line.len() > (dimensions.0 - margins.0 - margins.1) {
            let (part_1, part_2): (&str, &str) = line.split_at(dimensions.0 - margins.0 - margins.1).to_owned();
            split_str.push_str(&(part_1.to_owned() + "\n"));
            line = &mut part_2.clone();
        }
        split_str.push_str(line);
       split_str.push_str("\n");
    }
    return split_str;
}
