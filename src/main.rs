use std::io::stdin;

mod terminal;
use crate::terminal::formatting::*;
use crate::terminal::slides::*;
use crate::terminal::io::*;

use term_size;
//use clearscreen::clear;
fn main() {
    let (term_width, term_height) = term_size::dimensions().expect("Couldn't get terminal dimensions");
    let padding = (
        (term_width as f32 * 0.25 as f32).round() as usize, // the small number is a percentage
        (term_width as f32 * 0.25 as f32).round() as usize
    );

    let greeting = read_file("Greeting.md");
    //println!("{}", greeting);

    let slide: Slide = Slide::new( &greeting, &(term_width, term_height), &padding, Align::Center);
    let mut s: String = String::new();
    //println!("Beginning loop");
    slide.display();
    _ = stdin().read_line(&mut s).unwrap();
    //slide.slice_str(&s);

    //do horizontal centering (always so no special thingy there)
}
