use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::Context;
use rustyline::Helper;


pub const COMMANDS: &[&str] = &[
    "help",
    "about",
    "sysinfo",
    "version",
    "whoami",
    "pwd",
    "ls",
    "cd",
    "mkdir",
    "touch",
    "cat",
    "rm",
    "echo",
    "clear",
    "pkg",
    "config",
    "app",
    "security",
    "network",
    "intel",
    "exit",
];


#[derive(Clone)]
pub struct OpenTermCompleter;


impl Completer for OpenTermCompleter {

    type Candidate = Pair;


    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>), ReadlineError> {


        let word = &line[..pos];


        let suggestions = COMMANDS
            .iter()
            .filter(|cmd| cmd.starts_with(word))
            .map(|cmd| Pair {
                display: cmd.to_string(),
                replacement: cmd.to_string(),
            })
            .collect();


        Ok((0, suggestions))
    }
}



impl Hinter for OpenTermCompleter {

    type Hint = String;

}


impl Highlighter for OpenTermCompleter {}

impl Validator for OpenTermCompleter {}

impl Helper for OpenTermCompleter {}
