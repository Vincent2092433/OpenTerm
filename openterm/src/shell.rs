use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

use crate::commands;

pub fn start() {
    println!("==================================");
    println!("        OpenTerm v0.3.0");
    println!("==================================");
    println!("Cross-platform Developer Terminal");
    println!("Type 'help' for commands.\n");

    let mut rl = DefaultEditor::new().unwrap();

    loop {
        let line = rl.readline("OpenTerm$ ");

        match line {
            Ok(input) => {
                let input = input.trim();

                if input.is_empty() {
                    continue;
                }

                let _ = rl.add_history_entry(input);

                if !commands::execute(input) {
                    break;
                }
            }

            Err(ReadlineError::Interrupted) => {
                println!("Use 'exit' to quit.");
            }

            Err(ReadlineError::Eof) => {
                println!("Goodbye!");
                break;
            }

            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}