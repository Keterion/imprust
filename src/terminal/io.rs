use std::{fs, env};

pub fn read_file(name: &str) -> String {
    fs::read_to_string(name).expect(&format!("Couldn't read {}", name))
}

pub fn parse_args() -> String {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            return read_file(&args[1])
        },
        _ => {
        panic!("idk what the fuck you are specifying but its hella wrong, maybe you dont have a filename, check that")},
    }
}
