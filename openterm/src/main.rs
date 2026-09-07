mod app_manager;
mod commands;
mod completer;
mod config;
mod editor;
mod filesystem;
mod network;
mod package_manager;
mod process;
mod security;
mod shell;
mod spellscript;
mod system;

fn main() {
    shell::start();
}
