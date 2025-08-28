use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    match pattern {
        p if p.chars().count() == 1 =>
            return input_line.contains(p),
        p if p.starts_with("\\d") =>
            return input_line.contains(|c:char| c.is_numeric()),
        p if p.starts_with("\\w") =>
            return input_line.contains(|c:char| c.is_numeric() || c.is_alphabetic() || c == '_'),
        p if p.chars().count()>2 && p.starts_with("[") && p.ends_with("]") =>
            return input_line.contains(|c:char| p[1..p.len() - 1].contains(c)),
        p if p.chars().count()>2 && p.starts_with("[^") && p.ends_with("]") =>
            return input_line.contains(|c:char| !p[1..p.len() - 1].contains(c)),
        _ => panic!("Unhandled pattern: {}", pattern),
    }
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!");

    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    // Uncomment this block to pass the first stage
    if match_pattern(&input_line, &pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
