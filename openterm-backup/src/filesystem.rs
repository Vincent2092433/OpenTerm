use std::env;
use std::fs::{self, File};
use std::io::{Read, Write};

pub fn pwd() {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn ls() {
    match fs::read_dir(".") {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(file) => {
                        println!("{}", file.file_name().to_string_lossy());
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}

pub fn cd(path: &str) {
    if let Err(e) = env::set_current_dir(path) {
        println!("Error: {}", e);
    }
}

pub fn mkdir(name: &str) {
    match fs::create_dir(name) {
        Ok(_) => println!("Directory '{}' created.", name),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn touch(name: &str) {
    match File::create(name) {
        Ok(_) => println!("File '{}' created.", name),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn cat(name: &str) {
    let mut file = match File::open(name) {
        Ok(file) => file,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    let mut content = String::new();

    match file.read_to_string(&mut content) {
        Ok(_) => println!("{}", content),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn write_file(name: &str, text: &str) {
    match File::create(name) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(text.as_bytes()) {
                println!("Error: {}", e);
            } else {
                println!("Saved '{}'.", name);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}

pub fn rm(name: &str) {
    match fs::remove_file(name) {
        Ok(_) => println!("File '{}' removed.", name),
        Err(e) => println!("Error: {}", e),
    }
}