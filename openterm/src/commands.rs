use crate::filesystem;
use std::process::Command;

pub fn execute(input: &str) -> bool {
    let args: Vec<&str> = input.split_whitespace().collect();

    if args.is_empty() {
        return true;
    }

    match args[0] {
        "help" => {
            println!("================ OpenTerm Commands ================");
            println!("help                 Show this help menu");
            println!("pwd                  Show current directory");
            println!("ls                   List files");
            println!("cd <directory>       Change directory");
            println!("mkdir <directory>    Create a directory");
            println!("touch <file>         Create a file");
            println!("cat <file>           Display file contents");
            println!("rm <file>            Delete a file");
            println!("echo <text>          Print text");
            println!("clear                Clear the screen");
            println!("exit                 Exit OpenTerm");
            println!("===================================================");
        }

        "pwd" => filesystem::pwd(),

        "ls" => filesystem::ls(),

        "cd" => {
            if args.len() > 1 {
                filesystem::cd(args[1]);
            } else {
                println!("Usage: cd <directory>");
            }
        }

        "mkdir" => {
            if args.len() > 1 {
                filesystem::mkdir(args[1]);
            } else {
                println!("Usage: mkdir <directory>");
            }
        }

        "touch" => {
            if args.len() > 1 {
                filesystem::touch(args[1]);
            } else {
                println!("Usage: touch <file>");
            }
        }

        "cat" => {
            if args.len() > 1 {
                filesystem::cat(args[1]);
            } else {
                println!("Usage: cat <file>");
            }
        }

        "rm" => {
            if args.len() > 1 {
                filesystem::rm(args[1]);
            } else {
                println!("Usage: rm <file>");
            }
        }

        "echo" => {
            if args.len() > 1 {
                println!("{}", args[1..].join(" "));
            } else {
                println!();
            }
        }

        "clear" => {
            print!("\x1B[2J\x1B[1;1H");
        }

        "exit" => {
            println!("Goodbye!");
            return false;
        }

        _ => {
            let mut command = Command::new(args[0]);

            if args.len() > 1 {
                command.args(&args[1..]);
            }

            match command.status() {
                Ok(status) => {
                    if !status.success() {
                        println!("Command exited with status: {}", status);
                    }
                }

                Err(_) => {
                    println!("Unknown command: {}", args[0]);
                }
            }
        }
    }

    true
}