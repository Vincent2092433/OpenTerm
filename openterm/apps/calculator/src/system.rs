use std::env;

pub fn info() {
    println!("======================================");
    println!("          SpellShark OS");
    println!("======================================");
    println!("Terminal : OpenTerm");
    println!("Version  : 0.8.0");
    println!("Language : Rust");
    println!("Platform : {}", env::consts::OS);
    println!("CPU Arch : {}", env::consts::ARCH);
    println!("======================================");
}

pub fn version() {
    println!("OpenTerm v0.8.0");
}

pub fn whoami() {
    let user = crate::config::get_value("name");

    if user.is_empty() {
        println!("Guest");
    } else {
        println!("{}", user);
    }
}