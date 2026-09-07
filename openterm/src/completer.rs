use rustyline::Context;
use rustyline::Helper;
use rustyline::completion::{Completer, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;

pub struct OpenTermCompleter {
    pub commands: Vec<String>,
}

impl OpenTermCompleter {
    pub fn new() -> Self {
        Self {
            commands: vec![
                "help".into(),
                "about".into(),
                "sysinfo".into(),
                "version".into(),
                "whoami".into(),
                "pwd".into(),
                "ls".into(),
                "cd".into(),
                "mkdir".into(),
                "touch".into(),
                "cat".into(),
                "rm".into(),
                "echo".into(),
                "clear".into(),
                "pkg".into(),
                "config".into(),
                "app".into(),
                "security".into(),
                "network".into(),
                "exit".into(),
            ],
        }
    }
}

impl Completer for OpenTermCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let before_cursor = &line[..pos];

        let start = before_cursor.rfind(' ').map(|i| i + 1).unwrap_or(0);

        let word = &before_cursor[start..];

        let matches = self
            .commands
            .iter()
            .filter(|command| command.starts_with(word))
            .map(|command| Pair {
                display: command.clone(),
                replacement: command.clone(),
            })
            .collect();

        Ok((start, matches))
    }
}

impl Helper for OpenTermCompleter {}

impl Hinter for OpenTermCompleter {
    type Hint = String;
}

impl Highlighter for OpenTermCompleter {}

impl Validator for OpenTermCompleter {}
