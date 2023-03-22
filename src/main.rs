//use std::io::stdin;

mod terminal;
use crate::terminal::output::*;
use crate::terminal::slides::*;

use term_size;
fn main() {
    let (term_width, term_height) = term_size::dimensions().expect("Couldn't get terminal dimensions");
    println!("\u{250c}\u{2500}");
    println!("Width: {}, Height: {}\n", term_width, term_height);
    //let mut s: String = String::new();
    let padding = (
        (term_width as f32 * 0.25 as f32) as usize, // the small number is a percentage
        (term_width as f32 * 0.25 as f32) as usize
    );
    let mut slide: Slide = Slide::new( "...", &(term_width, term_height), &padding, Align::Center);
    slide.slice_string("Hello World!");
    slide.display();
    // loop {
    //     _ = stdin().read_line(&mut s).unwrap();
    //     write_input(&Align::Center, &s, term_width, padding);
    //     s.clear();
    // }
}
