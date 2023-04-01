pub mod text_formatting {
    use crate::terminal::slides::Line;
    #[derive(Clone)]
    pub enum Align {
        #[allow(unused)]
        Left,
        #[allow(unused)]
        Center,
        #[allow(unused)]
        Right,
    }

    pub fn border_line(width: usize, left_edge: &str, dash: &str, right_edge: &str) -> String {
        let mut border: String = String::new();
        border.push_str(left_edge);
        for _ in 2..width {
            border.push_str(dash);
        }
        border.push_str(right_edge);
        return border;
    }
    pub fn border_text(lines: &Vec<Line>, width: usize, height: usize, border_chars: &Vec<String>) -> String {
        let mut output: String = String::new();
        let filled_lines = 2 + lines.iter().count();
        let free_lines: usize = {
            // this gives the free lines or 0 if there is an overflow
            let free: isize = height as isize - filled_lines as isize;
            if free > 0 {
                height - filled_lines
            } else {
                0
            }
        };
        let free_line_halves: (usize, usize) = if free_lines%2==0 {
            (free_lines/2, free_lines/2)
        } else {
            (free_lines/2 + 1, free_lines/2)
        };
        let free_line: String = {
            let vertical_dash = border_chars.get(5).expect("Couldn't get vertical box character");
            format!("{vertical_dash}{}{vertical_dash}", String::from_utf8(vec![b' '; width-2]).unwrap())
        };

        output.push_str(&border_line(
            width,
            border_chars.get(0).unwrap(),
            border_chars.get(4).unwrap(),
            border_chars.get(1).unwrap(),
        ));
        for _ in 0..free_line_halves.0 {
            output.push_str(&free_line);
        }
        for line in lines {
            output.push_str(&line.format_line(true, true));
        }
        for _ in 0..free_line_halves.1 {
            output.push_str(&free_line);
        }
        output.push_str(&border_line(
            width,
            border_chars.get(2).unwrap(),
            border_chars.get(4).unwrap(),
            border_chars.get(3).unwrap(),
        ));
        return output;
    }

    pub fn slice_str(data: &str, dimensions: &(usize, usize), margins: &(usize, usize)) -> Vec<String> {
        let mut lines: Vec<String> = vec![];
        let mut current_line: String = String::new();
        let (mut words_in_line, mut characters_in_line): (Vec<&str>, usize);
        let max_output_width: usize = dimensions.0 - margins.0 - margins.1;

        for line in data.lines() {
            characters_in_line = 0;
            // use split_inclusive so that the spacing gets respected -> "  " stays "  "
            words_in_line = line.trim().split_inclusive(" ").collect();
            for word in words_in_line {
                characters_in_line += word.len();
                if characters_in_line < max_output_width { // if the word fits in the line
                    current_line.push_str(&format!("{}", word));
                } else {
                    if word.len() > max_output_width { // if you have a width of 10 and a word is 15 chars long, it'd break everything
                        lines.push(current_line.clone()); // word is too large for current line, save current line, go to next line
                        current_line.clear();

                        let mut current_word: &str = word;
                        let mut current_word_length: usize = current_word.len();
                        while current_word_length > max_output_width { // this splits the word and linewraps it
                            let (linebroken_word, rest_of_word) = current_word.split_at(max_output_width-1); // -1 because there will be a - character added
                            //println!("Rest of word: '{}-\n' at {} characters with a max space of {} characters", word_part_1, &format!("\n{}-\n", word_part_1).len(), max_len);
                            lines.push(format!("{linebroken_word}-")); // add part of word to line and go to next line
                            current_line.clear();

                            current_word_length = rest_of_word.len();
                            current_word = rest_of_word;
                        }
                        //println!("Rest of word: {} at {} characters, thought at {} characters, with a max space of {} characters", current_word, current_word.len(), current_word_length, max_len);
                        current_line.push_str(current_word); // push the rest of the word that fits in one line
                        characters_in_line = current_word.len();
                    } else {
                        lines.push(current_line.clone()); // save the line filled to the max
                        current_line.clear();

                        current_line.push_str(&format!("{word}"));
                        characters_in_line = word.len();
                    }
                }
            }
            lines.push(current_line.clone());
            current_line.clear();
        }
        return lines;
    }
}

pub mod text_coloring {
    use colored::Colorize;
    pub fn colorize_headings(line: &str) -> String {
        match line.trim().split_whitespace().nth(0).unwrap() {
            "#" => line.bright_green().to_string(),
            "##" => line.bright_cyan().to_string(),
            "###" => line.bright_blue().to_string(),
            "####" => line.bright_yellow().to_string(),
            "#####" => line.bright_magenta().to_string(),
            "######" => line.bright_black().to_string(),
            _ => line.to_string(),
        }
    }
}
