use std::fs::File;
use std::io::{self, Write};

pub fn edit(filename: &str) {
    println!("Editing '{}'", filename);
    println!("Type your text below.");
    println!("Type ':wq' on a new line to save and exit.");

    let mut lines = Vec::new();

    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim_end();

        if input == ":wq" {
            break;
        }

        lines.push(input.to_string());
    }

    let mut file = File::create(filename).unwrap();

    for line in lines {
        writeln!(file, "{}", line).unwrap();
    }

    println!("File '{}' saved.", filename);
}