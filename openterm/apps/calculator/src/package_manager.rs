use std::fs::{self, OpenOptions};
use std::io::Write;

const PACKAGE_FILE: &str = "openterm_packages.txt";

pub fn help() {
    println!("===============================");
    println!(" SpellShark Package Manager");
    println!("===============================");
    println!("pkg list");
    println!("pkg install <name>");
    println!("pkg remove <name>");
    println!("pkg info <name>");
    println!("pkg search <name>");
    println!("pkg help");
}

fn get_packages() -> Vec<String> {
    let content = fs::read_to_string(PACKAGE_FILE)
        .unwrap_or_default();

    content
        .lines()
        .filter(|x| !x.trim().is_empty())
        .map(|x| x.trim().to_string())
        .collect()
}

pub fn list() {
    let packages = get_packages();

    if packages.is_empty() {
        println!("No packages installed.");
    } else {
        println!("Installed packages:");

        for package in packages {
            println!("- {}", package);
        }
    }
}

pub fn install(name: &str) {
    let packages = get_packages();

    if packages.contains(&name.to_string()) {
        println!("Package '{}' is already installed.", name);
        return;
    }

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(PACKAGE_FILE)
        .unwrap();

    writeln!(file, "{}", name).unwrap();

    println!("Package '{}' installed.", name);
}

pub fn remove(name: &str) {
    let packages = get_packages();

    if !packages.contains(&name.to_string()) {
        println!("Package '{}' is not installed.", name);
        return;
    }

    let updated: Vec<String> = packages
        .into_iter()
        .filter(|package| package != name)
        .collect();

    fs::write(PACKAGE_FILE, updated.join("\n")).unwrap();

    println!("Package '{}' removed.", name);
}

pub fn info(name: &str) {
    let packages = get_packages();

    if packages.contains(&name.to_string()) {
        println!("===============================");
        println!("Package : {}", name);
        println!("Status  : Installed");
        println!("Source  : SpellShark Repository");
        println!("===============================");
    } else {
        println!("Package '{}' is not installed.", name);
    }
}

pub fn search(name: &str) {
    let packages = get_packages();

    let mut found = false;

    println!("Search results:");

    for package in packages {
        if package.contains(name) {
            println!("- {}", package);
            found = true;
        }
    }

    if !found {
        println!("No matching packages found.");
    }
}