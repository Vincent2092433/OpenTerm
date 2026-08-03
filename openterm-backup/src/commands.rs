use crate::filesystem;
use crate::package_manager;
use crate::process;
use crate::config;
use crate::app_manager;
use crate::system;
use crate::security;
use crate::network;

use std::io::{self, Write};


pub fn execute(input: &str) -> bool {

    let args: Vec<&str> = input
        .split_whitespace()
        .collect();


    if args.is_empty() {
        return true;
    }


    match args[0] {


        "help" => {

            println!("================ SpellShark Commands ================");
            println!("help                 Show help");
            println!("about                About SpellShark");
            println!("sysinfo              Show system information");
            println!("version              Show OpenTerm version");
            println!("whoami               Show current user");
            println!("pwd                  Show current directory");
            println!("ls                   List files");
            println!("cd <dir>             Change directory");
            println!("mkdir <dir>          Create directory");
            println!("touch <file>         Create file");
            println!("cat <file>           Read file");
            println!("rm <file>            Remove file");
            println!("echo <text>          Print text");
            println!("clear                Clear screen");
            println!("pkg                  Package manager");
            println!("config               Configuration");
            println!("app                 Application manager");
            println!("security             Security toolkit");
            println!("network              Network toolkit");
            println!("exit                 Exit OpenTerm");
            println!("====================================================");
        }



        "about" => {

            println!("==================================");
            println!("          SpellShark OS");
            println!("==================================");
            println!("Terminal : OpenTerm");
            println!("Version  : 0.8.0");
            println!("Engine   : Rust");
            println!("==================================");

        }



        "sysinfo" => {
            system::info();
        }


        "version" => {
            system::version();
        }


        "whoami" => {
            system::whoami();
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
            io::stdout().flush().unwrap();

        }



        "pkg" => {

            if args.len() < 2 {

                package_manager::help();

            } else {

                match args[1] {

                    "help" => package_manager::help(),

                    "list" => package_manager::list(),

                    "install" => {

                        if args.len() > 2 {
                            package_manager::install(args[2]);
                        } else {
                            println!("Usage: pkg install <name>");
                        }

                    }


                    "remove" => {

                        if args.len() > 2 {
                            package_manager::remove(args[2]);
                        } else {
                            println!("Usage: pkg remove <name>");
                        }

                    }


                    "info" => {

                        if args.len() > 2 {
                            package_manager::info(args[2]);
                        } else {
                            println!("Usage: pkg info <name>");
                        }

                    }


                    "search" => {

                        if args.len() > 2 {
                            package_manager::search(args[2]);
                        } else {
                            println!("Usage: pkg search <name>");
                        }

                    }


                    _ => println!("Unknown pkg command")

                }

            }

        }




        "config" => {

            if args.len() < 2 {

                config::list();

            } else {

                match args[1] {


                    "set" => {

                        if args.len() > 3 {

                            config::set(args[2], args[3]);

                        } else {

                            println!("Usage: config set <key> <value>");

                        }

                    }



                    "get" => {

                        if args.len() > 2 {

                            config::get(args[2]);

                        } else {

                            println!("Usage: config get <key>");

                        }

                    }



                    "list" => {

                        config::list();

                    }



                    _ => println!("Unknown config command")

                }

            }

        }




        "app" => {

        }

        "security"=>{
           
        }

        "network"=>{
            
            if args.len() < 2 {

                app_manager::help();

            } else {


                match args[1] {


                    "help" => app_manager::help(),


                    "list" => app_manager::list(),


                    "create" => {

                        if args.len() > 2 {

                            app_manager::create(args[2]);

                        } else {

                            println!("Usage: app create <name>");

                        }

                    }



                    "remove" => {

                        if args.len() > 2 {

                            app_manager::remove(args[2]);

                        } else {

                            println!("Usage: app remove <name>");

                        }

                    }



                    "run" => {

                        if args.len() > 2 {

                            app_manager::run(args[2]);

                        } else {

                            println!("Usage: app run <name>");

                        }

                    }



                    "info" => {

                        if args.len() > 2 {

                            app_manager::info(args[2]);

                        } else {

                            println!("Usage: app info <name>");

                        }

                    }



                    _ => println!("Unknown app command")

                }

            }

        }




        "exit" => {

            println!("Goodbye!");
            return false;

        }




        _ => {

            process::run(args[0], &args[1..]);

        }

    }


    true
}
