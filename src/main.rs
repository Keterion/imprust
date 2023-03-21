use std::io::stdin;

use term_size;
fn main() {
    let (term_width, term_height) = term_size::dimensions().expect("Couldn't get terminal dimensions");
    println!("Width: {}, Height: {}\n", term_width, term_height);
    let mut s: String = String::new();
    let padding = (
        (term_width as f32 * 0.25 as f32) as usize, // the small number is a percentage
        (term_width as f32 * 0.25 as f32) as usize
    );
    loop {
    _ = stdin().read_line(&mut s).unwrap();
    write_input(&Align::Center, &s, term_width, padding);
    s.clear();
    }
    // write_input(&Align::Center, "Hello World - Center", term_width, padding);
    // write_input(&Align::Right, "Hello World - Right", term_width, padding);

    // next thing on the agenda is an image to ascii converter
    
}

pub fn write_input(mode: &Align, input: &str, width: usize, padding: (usize, usize)) {
    let max_free_space = width - (padding.0  + padding.1);
    let split = validate_length(max_free_space, input);

    match mode {
        Align::Left => {
            for line in split.lines() {
                for _ in 0..padding.0 {
                    print!(" ");
                }
                println!("{}", line);
            }
        },
        Align::Center => {
            for line in split.lines() {
                for _ in 0..(padding.0 + (max_free_space - line.len())/2) {
                    print!(" ");
                }
                print!("{}\n", line);
            }
        },
        Align::Right => {
            for line in split.lines() {
                for _ in 0..(padding.0 + (max_free_space - line.len())) {
                    print!(" ");
                }
                print!("{}\n", line);
            }
        }
    }
}

pub fn validate_length(max_width: usize, input: &str) -> String {
    let mut split_str: String = String::new();
    let mut remaining = input;
    while remaining.len() > max_width {
        let (part_1, part_2): (&str, &str) = remaining.split_at(max_width).to_owned();
        split_str.push_str(&(part_1.to_owned() + "\n"));
        remaining = &mut part_2.clone();
    }
    split_str.push_str(remaining);
    return split_str;
}

pub enum Align {
    Left,
    Center,
    Right,
}