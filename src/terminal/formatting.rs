use self::word_formatting::colorize;

use super::slides::Slide;
pub fn space(width: usize, margin: (usize, usize), input: &str, align: &Align, separator: &str) -> String {
    //println!("input: {1}, length: {}", input.len(), input); //debugging
    let extra_whitespace = width - input.len() - (margin.0 + margin.1); // the remaining space in the line, also the box characters removed
    // the extra whitespace is the remaining space in a line after: text, margins, and the extra two characters for the box have been removed
    let mut l: String = String::new();

    match align {
        // the for _ in 1..margin are started at 1 because the box character has to be accounted for
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
            let (margin_left, margin_right) = 
                if extra_whitespace%2 == 0 {
                    (extra_whitespace/2, extra_whitespace/2)
                } else {
                    (extra_whitespace/2, extra_whitespace/2+1)
                };
            // margin left
            for _ in 1..(margin.0 + margin_left) {
                l.push_str(separator);
            }
            // text
            l.push_str(input);
            // margin right
            for _ in 1..(margin.1 + margin_right) {
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
    return l;
}
pub fn box_line(line: &str, box_char: &str) -> String { // adds the vertical lines on the sides of a line
    return format!("{}{}{}", box_char, line, box_char);
}

#[derive(Clone)]
pub enum Align {
    #[allow(unused)]
    Left,
    #[allow(unused)]
    Center,
    #[allow(unused)]
    Right,
}

pub fn border_text(slide: &Slide) -> String {
    let (top_left, top_right, bottom_left, bottom_right) = ("\u{250c}", "\u{2510}", "\u{2514}", &format!("{}", slide.pos));
    let (horizontal_dash, vertical_dash) = ("\u{2500}", "\u{2502}");
    let separator: &str = " ";
    let mut s: String = String::new();
    
    const TAKEN_BY_HORIZONTAL_BORDER: isize = 2;
    const PROMPT: isize = 1;
    let empty_height: usize = {
        let space: isize = slide.dimensions.1 as isize - slide.contents.lines().count() as isize - TAKEN_BY_HORIZONTAL_BORDER - PROMPT;
        if space >= 0 {
            space as usize
        } else {
            0
        }
    };
    let (empty_top, empty_bottom): (usize, usize) =
        if empty_height%2 == 0 {
            (empty_height/2, empty_height/2)
        } else {
            (empty_height/2+1, empty_height/2)
        };
    let empty_line: String = box_line(
        &space(
            slide.dimensions.0, 
            slide.margins, "", 
            &slide.text_align, separator), 
            vertical_dash
        ) + "\n";

    // upper box
    s.push_str(top_left);
    for _ in 2..slide.dimensions.0 {
        s.push_str(horizontal_dash);
    }
    s.push_str(&(top_right.to_owned() + "\n"));

    // empty lines
    for _ in 0..empty_top {
        s.push_str(&empty_line);
    }

    // text
    for line in slide.contents.lines() { // prints the lines and the sides of the box
        s.push_str(&(
            box_line(
                &colorize( // adds coloring to a string
                    &space( // adds margins
                        slide.dimensions.0, 
                        slide.margins, 
                        line,
                        &slide.text_align,
                        separator)
                    ),
                    vertical_dash
                ) + "\n"
            )
        );
        // SOOO, you gotta first add the margins.
        // THEN, you colorize the line if it contains, i.e, a heading
        // THEN, you add the box characters, so those don't get colored
    }

    // empty lines
    for _ in 0..empty_bottom {
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
    let mut sliced_str: String = String::new();
    let (mut words, mut sum): (Vec<&str>, usize);
    let max_len: usize = dimensions.0 - margins.0 - margins.1;
    for line in data.lines() {
        sum = 0;
        // use split_inclusive so that the spacing gets respected -> "  " stays "  "
        words = line.trim().split_inclusive(" ").collect();
        for word in words {
            sum += word.len();
            if sum < max_len { // if the word fits in the line
                sliced_str.push_str(&format!("{}", word));
            } else {
                if word.len() > max_len { // if you have a width of 10 and a word is 15 chars long, it'd break everything
                    let mut current_word_length: usize = word.len();
                    let mut current_word: &str = word;
                    while current_word_length > max_len { // this splits the word and linewraps it
                        let (word_part_1, word_part_2) = current_word.split_at(max_len-1); // -1 because there will be a - character added
                        //println!("Rest of word: '{}-\n' at {} characters with a max space of {} characters", word_part_1, &format!("\n{}-\n", word_part_1).len(), max_len);
                        sliced_str.push_str(&format!("\n{}-\n", word_part_1));
                        current_word_length = word_part_2.len();
                        current_word = word_part_2;
                    }
                    //println!("Rest of word: {} at {} characters, thought at {} characters, with a max space of {} characters", current_word, current_word.len(), current_word_length, max_len);
                    sliced_str.push_str(current_word); // push the rest of the word that fits in one line
                    sum = current_word.len();
                } else {
                    sliced_str.push_str(&format!("\n{}", word));
                    sum = word.len();
                }
            }
        }
        sliced_str.push_str("\n");
    }
    return sliced_str;
}

mod word_formatting {
    use colored::Colorize;
    pub fn colorize(line: &str) -> String {
        match line.trim().split_whitespace().nth(0).unwrap() {
            "#" => line.bright_green().to_string(),
            "##" => line.bright_cyan().to_string(),
            _ => line.to_string(),
        }
    }
}
