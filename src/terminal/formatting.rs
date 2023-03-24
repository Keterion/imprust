use super::slides::Slide;
pub fn space(width: usize, margin: (usize, usize), input: &str, align: &Align, box_char: &str, separator: &str) -> String {
    //println!("input: {1}, length: {}", input.len(), input);
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
    // if input.len() % 2 != 0 {
    //     l.push_str(separator); // you gotta do this because there's one character missing without it
    // }
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
#[derive(Clone)]
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
    let empty_line: String = space(slide.dimensions.0, slide.margins, "", &slide.text_align, vertical_dash, separator) + "\n";

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
        s.push_str(&(space(slide.dimensions.0, slide.margins, line, &slide.text_align, vertical_dash, separator) + "\n"));
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
