mod shell;
mod commands;
mod filesystem;
mod process;
mod package_manager;
mod config;
mod app_manager;
mod editor;
mod system;
mod spellscript;
mod security;
mod network;
mod intelligence;
mod completer;
mod plugins;
mod plugin_manager;

fn main() {
    shell::start();
}
