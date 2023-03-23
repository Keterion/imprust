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
        (term_dims.0 as f32 * 0.25 as f32).round() as usize, // the small number is a percentage
        (term_dims.0 as f32 * 0.25 as f32).round() as usize
    );

    let greeting = read_file("Greeting.md");
    let mut slides: Vec<&str> = greeting.split("---").map(|s| s.trim()).collect();
    let mut handler: handler = handler::new(slides, term_dims, padding, Align::Center);
    
    //println!("{}", greeting);

    let slide: Slide = Slide::new( &greeting, &(term_dims.0, term_dims.1), &padding, &Align::Center, 1);
    let mut s: String = String::new();
    //println!("Beginning loop");
    // _ = clear();
    // slide.display();
    // println!("{s}");
    //slide.slice_str(&s);
    _ = clear();
    loop {
        print!("n: next | b: back | gt [num]: goto num >> ");
        _ = std::io::stdout().flush();
        _ = stdin().read_line(&mut s).unwrap();
        handler.parse_command(&s);
        s.clear();
    }

    
}

struct handler {
    slides: Vec<Slide>,
    currPos: usize,
}

impl handler {
    pub fn new(contents: Vec<&str>, dimensions: Dims, margins: Dims, align: Align) -> handler {
        handler {
            slides: {
                let mut v: Vec<Slide> = vec![];
                let mut i = 0;
                for text in contents {
                    v.push( Slide::new(text, &dimensions, &margins, &align, i) );
                    i += 1;
                }
                v
            },
            currPos: 0,
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
        if (self.currPos + 1) > (self.slides.len()-1) {
            self.display_current();
            return;
        }
        self.currPos += 1;
        self.display_current();
    }
    fn back(&mut self) {
        if (self.currPos as isize - 1) < 0 {
            self.display_current();
            return;
        }
        self.currPos -= 1;
        self.display_current();
    }
    fn go_to(&mut self, nPos: usize) {
        if nPos > (self.slides.len()-1) {
            self.display_current();
            return;
        }
        self.currPos = nPos;
        self.display_current();
    }
    fn display_current(&self) {
        _ = clear();
        self.slides.get(self.currPos).unwrap().display();
    }
}