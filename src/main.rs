use std::io::Write;
use std::io::stdin;

mod terminal;
use crate::terminal::slides::*;
use crate::terminal::io::*;

use term_size;
use clearscreen::clear;

type Dims = (usize, usize);

fn main() {

    let term_dims: Dims = term_size::dimensions().expect("Couldn't get terminal dimensions");
    // term_dims.0 -> width
    let margins: Dims = (
        (term_dims.0 as f32 * 0.125 as f32).round() as usize, // the small number is a percentage
        (term_dims.0 as f32 * 0.125 as f32).round() as usize
    );

    let greeting = parse_args();
    let slides: Vec<&str> = greeting.split("*new_slide*").map(|s| s.trim()).collect();
    let mut handler: Handler = Handler::new(slides, term_dims, margins);
    
    let mut s: String = String::new();
    //_ = clear();
    loop {
        print!(">> ");
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
    pub fn new(contents: Vec<&str>, dimensions: Dims, margins: Dims) -> Handler {
        Handler {
            slides: {
                let mut v: Vec<Slide> = vec![];
                for text in contents {
                    v.push( Slide::new(text, &dimensions, &margins) );
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
            "help" => self::Handler::help(),
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
                } else {
                    self.next();
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

    fn help() {
        _ = clear();
        println!("n: next");
        println!("b: back");
        println!("gt [num]: go to [num]");
        println!("none: next");
        println!("help: display this dialog");
    }
}
