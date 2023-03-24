use std::io::Write;
use std::io::{stdin};

mod terminal;
use crate::terminal::formatting::*;
use crate::terminal::slides::*;
use crate::terminal::io::*;

use term_size;
use clearscreen::clear;

type Dims = (usize, usize);
fn main() {

    let term_dims: Dims = term_size::dimensions().expect("Couldn't get terminal dimensions");
    // term_dims.0 -> width
    let padding: Dims = (
        (term_dims.0 as f32 * 0.125 as f32).round() as usize, // the small number is a percentage
        (term_dims.0 as f32 * 0.125 as f32).round() as usize
    );

    let greeting = read_file("Greeting.md");
    let slides: Vec<&str> = greeting.split("*new_slide*").map(|s| s.trim()).collect();
    let mut handler: Handler = Handler::new(slides, term_dims, padding, Align::Center);
    
    let mut s: String = String::new();
    _ = clear();
    loop {
        print!("n: next | b: back | gt [num]: goto num >> ");
        _ = std::io::stdout().flush();
        _ = stdin().read_line(&mut s).unwrap();
        handler.parse_command(&s);
        s.clear();
    }
}

struct Handler {
    slides: Vec<Slide>,
    curr_pos: usize,
}

impl Handler {
    pub fn new(contents: Vec<&str>, dimensions: Dims, margins: Dims, align: Align) -> Handler {
        Handler {
            slides: {
                let mut v: Vec<Slide> = vec![];
                let mut i = 0;
                for text in contents {
                    v.push( Slide::new(text, &dimensions, &margins, &align, i) );
                    i += 1;
                }
                v
            },
            curr_pos: 0,
        }
    }
    pub fn parse_command(&mut self, command: &str) {
        match command.trim() {
            "n" => self.next(),
            "b" => self.back(),
            _ => {
                if command.contains("gt") {
                    let pos: usize = command.split(" ")
                        .nth(1)
                        .unwrap()
                        .trim()
                        .parse()
                        .expect(&format!("Not a positive integer: {}", command.split(" ").nth(1).unwrap())
                    );
                    self.go_to(pos);
                }
            }

        }
    }
    fn next(&mut self) {
        if (self.curr_pos + 1) > (self.slides.len()-1) {
            self.display_current();
            return;
        }
        self.curr_pos += 1;
        self.display_current();
    }
    fn back(&mut self) {
        if (self.curr_pos as isize - 1) < 0 {
            self.display_current();
            return;
        }
        self.curr_pos -= 1;
        self.display_current();
    }
    fn go_to(&mut self, n_pos: usize) {
        if n_pos > (self.slides.len()-1) {
            self.display_current();
            return;
        }
        self.curr_pos = n_pos;
        self.display_current();
    }
    fn display_current(&self) {
        _ = clear();
        self.slides.get(self.curr_pos).unwrap().display();
    }
}