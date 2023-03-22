use super::output::*;

pub struct Slide {
    contents: String,
    dimensions: (usize, usize),
    margins: (usize, usize),
    text_align: Align,
    outline: bool,
}

impl Slide {
    pub fn new(contents: &str, dimensions: &(usize, usize), margins: &(usize, usize), align: Align) -> Slide {
        Slide {
            contents: contents.to_owned(),
            dimensions: dimensions.to_owned(),
            margins: margins.to_owned(),
            text_align: align,
            outline: true,
        }
    }
    pub fn slice_string(&mut self, data: &str) {
        // width = self.dimensions.0
        self.contents = validate_length(self.dimensions.0, data).to_owned();
    }
    pub fn display(&self) {
        write_input(&self.text_align, &self.contents, self.dimensions.0, self.margins);
        if self.outline {
            println!("{}", self.border_text(&self.contents));
        } else {
            println!("{}", self.contents);
        }

    }
    fn border_text(&self, text: &str) -> String {
        let (top_left, top_right, bottom_left, bottom_right) = ("\u{250c}", "\u{2510}", "\u{2514}", "\u{2518}");
        let (horizontal_dash, vertical_dash) = ("\u{2500}", "\u{2502}");
        let mut s: String = String::new();
        let remaining_usable_width = self.dimensions.0 - (self.margins.0 + self.margins.1 + 2);
        // upper box
        s.push_str(top_left);
        for _ in 2..self.dimensions.0 {
            s.push_str(horizontal_dash);
        }
        s.push_str(&(top_right.to_owned() + "\n"));


        for line in text.lines() { // prints the lines and the sides of the box
            s.push_str(vertical_dash);
            for _ in 0..self.margins.0-1 {
                s.push_str(" ");
            }
            s.push_str(line);
            for _ in 0..self.margins.0-1 {
                s.push_str(" ");
            }
            s.push_str(&(vertical_dash.to_owned() + "\n"));
        }

        // lower box
        s.push_str(bottom_left);
        for _ in 2..self.dimensions.0 {
            s.push_str(horizontal_dash);
        }
        s.push_str(bottom_right);
        return s;
    }
}