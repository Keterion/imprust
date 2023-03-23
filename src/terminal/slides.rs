use super::formatting::*;

pub struct Slide {
    pub contents: String,
    pub dimensions: (usize, usize),
    pub margins: (usize, usize),
    pub text_align: Align,
    outline: bool,
    pub pos: usize,
}

impl Slide {
    pub fn new(contents: &str, dimensions: &(usize, usize), margins: &(usize, usize), align: Align, pos: usize) -> Slide {
        Slide {
            contents: slice_str(contents, dimensions, margins),
            dimensions: dimensions.to_owned(),
            margins: margins.to_owned(),
            text_align: align,
            outline: true,
            pos: pos,
        }
    }
    pub fn display(&self) {
        if self.outline {
            println!("{}", border_text(self));
        } else {
            println!("{}", self.contents);
        }

    }
}