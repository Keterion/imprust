pub fn write_input(mode: &Align, input: &str, width: usize, margin: (usize, usize)) {
    let max_free_space = width - (margin.0  + margin.1);

    match mode {
        Align::Left => {
            for line in input.lines() {
                for _ in 0..margin.0 {
                    print!(" ");
                }
                println!("{}", line);
            }
        },
        Align::Center => {
            for line in input.lines() {
                for _ in 0..(margin.0 + (max_free_space - line.len())/2) {
                    print!(" ");
                }
                print!("{}\n", line);
            }
        },
        Align::Right => {
            for line in input.lines() {
                for _ in 0..(margin.0 + (max_free_space - line.len())) {
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

#[allow(unused)]
pub enum Align {
    Left,
    Center,
    Right,
}