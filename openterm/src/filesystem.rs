use std::env;
use std::fs::{self, File};
use std::io::Read;


pub fn pwd() {
    println!("{}", env::current_dir().unwrap().display());
}


pub fn ls() {

    if let Ok(files) = fs::read_dir(".") {

        for file in files {

            if let Ok(entry) = file {
                println!("{}", entry.file_name().to_string_lossy());
            }
        }
    }
}


pub fn cd(path: &str) {

    if let Err(e) = env::set_current_dir(path) {
        println!("Error: {}", e);
    }
}


pub fn mkdir(name: &str) {

    if let Err(e) = fs::create_dir(name) {
        println!("Error: {}", e);
    }
}


pub fn touch(name: &str) {

    if let Err(e) = File::create(name) {
        println!("Error: {}", e);
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

    file.read_to_string(&mut content).unwrap();

    println!("{}", content);
}


pub fn rm(name: &str) {

    if let Err(e) = fs::remove_file(name) {
        println!("Error: {}", e);
    }
}