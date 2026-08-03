use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::env;

use crate::commands;


pub fn start() {

    let theme = crate::config::get_value("theme");


    match theme.as_str() {

        "cyber" => {

            println!("==================================");
            println!("        ⚡ OpenTerm CYBER ⚡");
            println!("==================================");

        }


        "dark" => {

            println!("==================================");
            println!("          OpenTerm DARK");
            println!("==================================");

        }


        _ => {

            println!("==================================");
            println!("          OpenTerm v0.7.0");
            println!("==================================");

        }

    }


    println!("Cross-platform Developer Terminal");
    println!("Type 'help' for commands.\n");



    let mut rl = DefaultEditor::new()
        .unwrap();



    loop {


        let cwd = env::current_dir()
            .unwrap_or_default()
            .display()
            .to_string();



        let username = crate::config::get_value("name");



        let prompt = if username.is_empty() {


            format!("OpenTerm:{}$ ", cwd)


        } else {


            format!("{}@OpenTerm:{}$ ", username, cwd)


        };




        match rl.readline(&prompt) {



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

                println!("Press Ctrl+D or type 'exit' to quit.");

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